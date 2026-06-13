// kplock: Secure CLI tool to unlock KeePassXC
// with an age-encrypted keyfile.

mod cli;
mod config;
mod crypto;
mod keyfile;
mod launcher;

use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use cli::{Cli, Commands};
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { input, output } => cmd_encrypt(input, output),
        Commands::Unlock => cmd_unlock(),
    }
}

/// `encrypt` command: encrypts a plain-text keyfile with age.
fn cmd_encrypt(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    eprintln!("--- kplock: Keyfile Encryption ---");

    // Verify input file exists
    if !input.exists() {
        anyhow::bail!(
            "Could not find the keyfile at: {}\nCreate a keyfile before proceeding.",
            input.display()
        );
    }

    // Determine output path
    let config = Config::load()?;
    let target = match output {
        Some(p) => p,
        None => config.resolve_path(&config.encrypted_keyfile_path),
    };

    eprintln!(
        "Encrypting '{}' → '{}'",
        input.display(),
        target.display()
    );

    // Encrypt the file
    crypto::encrypt_file(&input, &target)?;

    // Ask if user wants to securely delete the original
    eprint!("Do you want to securely delete the original keyfile '{}'? (y/n): ", input.display());
    io::stdout().flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    if response.trim().eq_ignore_ascii_case("y") {
        // Secure delete using TempKeyfile (overwrites with zeros before deleting)
        let data = std::fs::read(&input)
            .context("Could not read original file for secure deletion")?;
        // Create TempKeyfile on existing file to trigger RAII cleanup
        let _guard = keyfile::TempKeyfile::new(&input, &data)
            .context("Could not prepare secure deletion of original file")?;
        // When going out of scope, _guard automatically executes secure_cleanup
        eprintln!("✅ Original file securely deleted.");
    }

    // Update config with the encrypted file path
    let mut config = Config::load()?;
    config.encrypted_keyfile_path = target.to_string_lossy().to_string();
    config.save()?;

    eprintln!("\n--- Encryption completed! ---");
    eprintln!("Use 'kplock unlock' to unlock KeePassXC.");
    Ok(())
}

/// `unlock` command: decrypts the keyfile, launches KeePassXC, and cleans up on close.
fn cmd_unlock() -> Result<()> {
    let config = Config::load()?;

    let encrypted_path = config.resolve_path(&config.encrypted_keyfile_path);
    let temp_path = config.resolve_path(&config.temp_keyfile_path);

    // Verify encrypted file exists
    if !encrypted_path.exists() {
        anyhow::bail!(
            "Could not find encrypted keyfile at: {}\nRun 'kplock encrypt' first.",
            encrypted_path.display()
        );
    }

    // Register signal handler (Ctrl+C)
    launcher::setup_signal_handler()?;

    // Decrypt keyfile
    eprintln!("🔐 Decrypting keyfile...");
    let decrypted_data = crypto::decrypt_file(&encrypted_path)?;

    // Create secure temporary file (RAII: automatically cleaned up out of scope)
    let temp_keyfile = keyfile::TempKeyfile::new(&temp_path, &decrypted_data)
        .context("Could not create temporary keyfile")?;

    // Launch KeePassXC and wait for it to close
    launcher::run_keepassxc(&config.keepassxc_binary, temp_keyfile.path())?;

    // When going out of scope, temp_keyfile.drop() automatically executes secure_cleanup
    eprintln!("✅ Done.");
    Ok(())
}
