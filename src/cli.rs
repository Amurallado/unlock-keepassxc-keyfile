use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// CLI entry point.
#[derive(Parser)]
#[command(name = "kplock", about = "Secure CLI tool to unlock KeePassXC")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Commands {
    /// Encrypt a keyfile with age.
    Encrypt {
        /// Path to the plain-text keyfile.
        #[arg(short, long, default_value = "./keyfile.key")]
        input: PathBuf,

        /// Optional output path for the encrypted file.
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Unlock KeePassXC using the encrypted keyfile.
    Unlock,
}
