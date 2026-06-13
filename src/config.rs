use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

/// Main configuration for kplock.
///
/// Serialized/deserialized as TOML and stored in the user's config
/// directory (`~/.config/kplock/config.toml` on Linux,
/// `%APPDATA%/kplock/config.toml` on Windows).
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Path to the age-encrypted keyfile.
    pub encrypted_keyfile_path: String,

    /// Temporary path where the keyfile is decrypted while KeePassXC is open.
    pub temp_keyfile_path: String,

    /// Name or path of the KeePassXC binary.
    pub keepassxc_binary: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            encrypted_keyfile_path: "~/.clave_kp/keyfile.key.age".to_string(),
            temp_keyfile_path: default_temp_keyfile_path(),
            keepassxc_binary: "keepassxc".to_string(),
        }
    }
}

impl Config {
    /// Returns the kplock configuration directory.
    ///
    /// - Linux: `~/.config/kplock/`
    /// - Windows: `%APPDATA%/kplock/`
    pub fn config_dir() -> Result<PathBuf> {
        let base = dirs::config_dir()
            .context("Could not determine the system configuration directory");
        Ok(base?.join("kplock"))
    }

    /// Returns the full path to the configuration file (`config.toml`).
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Loads the configuration from `config.toml`.
    ///
    /// If the file exists, reads and deserializes it. Otherwise, creates
    /// the directory with restrictive permissions, writes a default file
    /// with explanatory comments, and returns the default configuration.
    pub fn load() -> Result<Config> {
        let path = Self::config_path()?;

        if path.exists() {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("Could not read configuration file: {}", path.display()))?;
            let config: Config = toml::from_str(&contents)
                .with_context(|| format!("Error parsing configuration file: {}", path.display()))?;

            // Validate configured paths
            config.validate_paths()?;

            Ok(config)
        } else {
            let config = Config::default();
            let dir = Self::config_dir()?;
            create_dir_restricted(&dir)?;

            let commented_contents = default_config_with_comments(&config);
            fs::write(&path, &commented_contents)
                .with_context(|| format!("Could not write configuration file: {}", path.display()))?;

            Ok(config)
        }
    }

    /// Saves the current configuration to `config.toml`.
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let dir = Self::config_dir()?;
        create_dir_restricted(&dir)?;

        let contents = toml::to_string_pretty(self)
            .context("Could not serialize configuration to TOML")?;
        fs::write(&path, &contents)
            .with_context(|| format!("Could not write configuration file: {}", path.display()))?;

        Ok(())
    }

    /// Expands the `~` character to the user's home directory.
    ///
    /// If the path starts with `~`, it is replaced with the home directory.
    /// Otherwise, it is returned as-is.
    pub fn resolve_path(&self, path: &str) -> PathBuf {
        if let Some(rest) = path.strip_prefix('~') {
            if let Some(home) = dirs::home_dir() {
                // Strip leading separator if present: "~/foo" → "foo"
                let rest = rest.strip_prefix('/').or_else(|| rest.strip_prefix('\\')).unwrap_or(rest);
                return home.join(rest);
            }
        }
        PathBuf::from(path)
    }

    /// Validates that configured paths resolve within the user's home
    /// directory to prevent path-traversal attacks via a malicious config.
    fn validate_paths(&self) -> Result<()> {
        let home = dirs::home_dir()
            .context("Could not determine user home directory")?;

        let encrypted = self.resolve_path(&self.encrypted_keyfile_path);
        let temp = self.resolve_path(&self.temp_keyfile_path);

        // Allow /tmp on Unix as a valid temp location
        let is_valid_path = |p: &Path| -> bool {
            if p.starts_with(&home) {
                return true;
            }
            #[cfg(unix)]
            if p.starts_with("/tmp") {
                return true;
            }
            false
        };

        if !is_valid_path(&encrypted) {
            bail!(
                "encrypted_keyfile_path '{}' resolves outside the user home directory. \
                 Refusing to proceed for security reasons.",
                self.encrypted_keyfile_path
            );
        }

        if !is_valid_path(&temp) {
            bail!(
                "temp_keyfile_path '{}' resolves outside the user home directory. \
                 Refusing to proceed for security reasons.",
                self.temp_keyfile_path
            );
        }

        Ok(())
    }
}

/// Creates a directory with restrictive permissions (0o700 on Unix).
fn create_dir_restricted(dir: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        std::fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(dir)
            .with_context(|| format!("Could not create configuration directory: {}", dir.display()))?;
    }
    #[cfg(not(unix))]
    {
        fs::create_dir_all(dir)
            .with_context(|| format!("Could not create configuration directory: {}", dir.display()))?;
    }
    Ok(())
}

/// Returns the default temporary keyfile path for the current platform.
fn default_temp_keyfile_path() -> String {
    #[cfg(unix)]
    {
        "/tmp/keyfile.key".to_string()
    }
    #[cfg(windows)]
    {
        let temp = std::env::var("TEMP")
            .or_else(|_| std::env::var("TMP"))
            .unwrap_or_else(|_| r"C:\Temp".to_string());
        format!(r"{}\keyfile.key", temp)
    }
}

/// Generates default TOML content with explanatory comments.
fn default_config_with_comments(config: &Config) -> String {
    format!(
        r#"# kplock configuration
# This file was auto-generated with default values.
# Edit the paths to match your system.

# Path to the age-encrypted keyfile.
# You can use ~ to refer to your home directory.
encrypted_keyfile_path = "{encrypted}"

# Temporary path where the keyfile is decrypted while KeePassXC is open.
# The file is securely wiped when KeePassXC is closed.
temp_keyfile_path = "{temp}"

# Name or full path of the KeePassXC binary.
# If it's in the system PATH, just the name is enough.
keepassxc_binary = "{binary}"
"#,
        encrypted = config.encrypted_keyfile_path,
        temp = config.temp_keyfile_path,
        binary = config.keepassxc_binary,
    )
}
