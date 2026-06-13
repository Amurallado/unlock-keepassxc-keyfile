use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Temporary keyfile with automatic secure cleanup (RAII).
///
/// When dropped, overwrites the file contents with zeros and deletes it.
pub struct TempKeyfile {
    path: PathBuf,
    size: usize,
}

impl TempKeyfile {
    /// Creates a new temporary keyfile with the provided data.
    ///
    /// - Creates parent directories if they don't exist.
    /// - On Unix, sets permissions to 0o600 (owner read/write only).
    /// - Writes the data, then flushes and calls sync_all.
    pub fn new(path: &Path, data: &[u8]) -> Result<Self> {
        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Could not create parent directories: {:?}", parent))?;
        }

        // Create the file
        let mut file = fs::File::create(path)
            .with_context(|| format!("Could not create temporary file: {:?}", path))?;

        // Unix: set permissions to 0o600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(0o600);
            file.set_permissions(perms)
                .with_context(|| format!("Could not set permissions 0600: {:?}", path))?;
        }

        // Write data
        file.write_all(data)
            .with_context(|| format!("Could not write data to: {:?}", path))?;

        // Flush and sync
        file.flush()
            .with_context(|| "Error flushing temporary file")?;
        file.sync_all()
            .with_context(|| "Error syncing temporary file")?;

        Ok(Self {
            path: path.to_path_buf(),
            size: data.len(),
        })
    }

    /// Returns the path of the temporary file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Secure cleanup: overwrites with zeros and deletes the file.
    ///
    /// Does not panic; individual errors are ignored to ensure
    /// cleanup proceeds as far as possible.
    fn secure_cleanup(&self) {
        if !self.path.exists() {
            return;
        }

        eprintln!("Performing secure cleanup of: {:?}", self.path);

        // Overwrite with zeros
        if let Ok(mut file) = fs::OpenOptions::new().write(true).open(&self.path) {
            let zeros = vec![0u8; self.size];
            let _ = file.write_all(&zeros);
            let _ = file.flush();
            let _ = file.sync_all();
        }

        // Delete the file
        if fs::remove_file(&self.path).is_ok() {
            eprintln!("Temporary file deleted successfully.");
        } else {
            eprintln!("WARNING: Could not delete temporary file: {:?}", self.path);
        }
    }
}

impl Drop for TempKeyfile {
    fn drop(&mut self) {
        self.secure_cleanup();
    }
}
