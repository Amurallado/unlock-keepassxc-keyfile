use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Launches KeePassXC with the given keyfile and waits for it to close.
pub fn run_keepassxc(binary: &str, keyfile_path: &Path) -> Result<()> {
    eprintln!("Opening KeePassXC...");

    let mut child = Command::new(binary)
        .arg("--keyfile")
        .arg(keyfile_path)
        .spawn()
        .with_context(|| format!("Could not start KeePassXC from '{}'", binary))?;

    eprintln!("Waiting for KeePassXC to close...");

    child
        .wait()
        .context("Error waiting for KeePassXC to finish")?;

    eprintln!("KeePassXC has closed.");

    Ok(())
}

/// Registers a signal handler for Ctrl+C.
///
/// Sets an atomic flag that can be checked by the main loop.
/// The actual cleanup happens through Rust's Drop trait when
/// the process exits, ensuring the temporary keyfile is wiped.
pub fn setup_signal_handler() -> Result<()> {
    let interrupted = Arc::new(AtomicBool::new(false));
    let flag = interrupted.clone();

    ctrlc::set_handler(move || {
        if flag.load(Ordering::SeqCst) {
            // Second Ctrl+C — force exit immediately
            eprintln!("\nForced exit.");
            std::process::exit(1);
        }
        flag.store(true, Ordering::SeqCst);
        eprintln!("\nInterrupted. Cleaning up temporary file...");
        // Exit cleanly so that Drop handlers run
        std::process::exit(0);
    })
    .context("Could not register Ctrl+C signal handler")?;

    Ok(())
}
