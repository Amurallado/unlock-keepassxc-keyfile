// Cryptography module: encryption and decryption with age (scrypt / passphrase).

use std::fs;
use std::io::{Read, Write};
use std::iter;
use std::path::Path;

use age::secrecy::SecretString;
use anyhow::{bail, Context, Result};
use zeroize::Zeroize;

/// Encrypts the file at `source` and writes the result to `target`.
///
/// Prompts the user for a passphrase twice for confirmation.
/// Uses age scrypt-based (passphrase) encryption.
pub fn encrypt_file(source: &Path, target: &Path) -> Result<()> {
    let plaintext = fs::read(source)
        .with_context(|| format!("Could not read source file: {}", source.display()))?;

    // Prompt for passphrase twice to confirm
    let mut passphrase = rpassword::prompt_password("Enter passphrase: ")
        .context("Error reading passphrase")?;
    let mut passphrase_confirm =
        rpassword::prompt_password("Confirm passphrase: ")
            .context("Error reading passphrase confirmation")?;

    if passphrase != passphrase_confirm {
        // Zeroize both before bailing
        passphrase.zeroize();
        passphrase_confirm.zeroize();
        bail!("Passphrases do not match");
    }

    // Zeroize the confirmation copy immediately
    passphrase_confirm.zeroize();

    let secret = SecretString::from(passphrase.clone());
    // Zeroize the plain String now that SecretString holds it
    passphrase.zeroize();

    let recipient = age::scrypt::Recipient::new(secret);

    // Encrypt the data in memory
    let encryptor = age::Encryptor::with_recipients(iter::once(&recipient as &dyn age::Recipient))
        .expect("a valid recipient was provided");

    let mut encrypted = vec![];
    let mut writer = encryptor
        .wrap_output(&mut encrypted)
        .context("Error initializing encryption")?;
    writer
        .write_all(&plaintext)
        .context("Error writing encrypted data")?;
    writer
        .finish()
        .context("Error finalizing encryption")?;

    // Create parent directories if they don't exist
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Could not create parent directories for: {}",
                target.display()
            )
        })?;
    }

    fs::write(target, &encrypted)
        .with_context(|| format!("Could not write encrypted file: {}", target.display()))?;

    eprintln!("File encrypted successfully at: {}", target.display());
    Ok(())
}

/// Decrypts the file at `source` and returns the plaintext bytes.
///
/// Prompts the user for the passphrase once.
/// Uses age scrypt-based (passphrase) decryption.
pub fn decrypt_file(source: &Path) -> Result<Vec<u8>> {
    let data = fs::read(source)
        .with_context(|| format!("Could not read encrypted file: {}", source.display()))?;

    let mut passphrase = rpassword::prompt_password("Enter passphrase: ")
        .context("Error reading passphrase")?;

    let secret = SecretString::from(passphrase.clone());
    // Zeroize the plain String now that SecretString holds it
    passphrase.zeroize();

    let identity = age::scrypt::Identity::new(secret);

    let decryptor = age::Decryptor::new(&data[..])
        .context("Error reading encrypted file header")?;

    let mut decrypted = vec![];
    let mut reader = decryptor
        .decrypt(iter::once(&identity as &dyn age::Identity))
        .context("Decryption error: wrong passphrase or corrupted file")?;
    reader
        .read_to_end(&mut decrypted)
        .context("Error reading decrypted data")?;

    Ok(decrypted)
}
