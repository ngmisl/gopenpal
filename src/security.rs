//! Security module for path validation and sandboxing.
//!
//! This module ensures that all file operations are restricted to the current
//! working directory and its subdirectories, preventing agents from accessing
//! files outside the designated workspace.

use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::error::{AppError, Result};

/// Security configuration for path validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Root directory - agents cannot access files outside this
    pub sandbox_root: PathBuf,
    /// Whether to enforce strict sandboxing
    pub enforce_sandbox: bool,
    /// Additional allowed directories (relative to sandbox_root)
    pub allowed_subdirs: Vec<PathBuf>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            sandbox_root: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            enforce_sandbox: true,
            allowed_subdirs: vec![
                PathBuf::from("configs"),
                PathBuf::from("migrations"),
                PathBuf::from(".gopenpal_pids"),
                PathBuf::from(".gopenpal_logs"),
            ],
        }
    }
}

impl SecurityConfig {
    /// Create a new security configuration with a specific sandbox root.
    pub fn new(sandbox_root: PathBuf) -> Self {
        Self {
            sandbox_root,
            enforce_sandbox: true,
            allowed_subdirs: vec![
                PathBuf::from("configs"),
                PathBuf::from("migrations"),
                PathBuf::from(".gopenpal_pids"),
                PathBuf::from(".gopenpal_logs"),
            ],
        }
    }

    /// Validate that a path is within the sandbox.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to validate
    ///
    /// # Returns
    ///
    /// Returns Ok(PathBuf) with the canonicalized path if valid,
    /// or Err(AppError) if the path is outside the sandbox.
    pub fn validate_path<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        let path = path.as_ref();

        // If sandboxing is not enforced, allow all paths (for development/testing)
        if !self.enforce_sandbox {
            return Ok(path.to_path_buf());
        }

        // Resolve the path to absolute
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.sandbox_root.join(path)
        };

        // Canonicalize both paths to resolve .. and symlinks
        let canonical_path = absolute_path.canonicalize().map_err(|e| {
            AppError::Security(format!(
                "Cannot access path '{}': {}",
                path.display(),
                e
            ))
        })?;

        let canonical_root = self.sandbox_root.canonicalize().map_err(|e| {
            AppError::Security(format!(
                "Cannot determine sandbox root '{}': {}",
                self.sandbox_root.display(),
                e
            ))
        })?;

        // Check if the canonical path starts with the sandbox root
        if !canonical_path.starts_with(&canonical_root) {
            return Err(AppError::Security(format!(
                "Access denied: Path '{}' is outside the allowed sandbox directory '{}'",
                path.display(),
                canonical_root.display()
            )));
        }

        Ok(canonical_path)
    }

    /// Validate multiple paths at once.
    pub fn validate_paths<P: AsRef<Path>>(&self, paths: &[P]) -> Result<Vec<PathBuf>> {
        paths.iter().map(|p| self.validate_path(p)).collect()
    }

    /// Check if a path is within the sandbox without canonicalizing.
    ///
    /// This is useful for checking paths that don't exist yet.
    pub fn is_path_safe<P: AsRef<Path>>(&self, path: P) -> bool {
        if !self.enforce_sandbox {
            return true;
        }

        let path = path.as_ref();

        // Resolve to absolute
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.sandbox_root.join(path)
        };

        // Normalize the path (resolve .. without following symlinks)
        if let Ok(normalized) = Self::normalize_path(&absolute_path) {
            if let Ok(canonical_root) = self.sandbox_root.canonicalize() {
                return normalized.starts_with(&canonical_root);
            }
        }

        false
    }

    /// Normalize a path without following symlinks.
    fn normalize_path(path: &Path) -> std::io::Result<PathBuf> {
        let mut components = Vec::new();
        for component in path.components() {
            match component {
                std::path::Component::ParentDir => {
                    if !components.is_empty() {
                        components.pop();
                    }
                }
                std::path::Component::CurDir => {}
                comp => components.push(comp),
            }
        }

        let mut result = PathBuf::new();
        for component in components {
            result.push(component);
        }
        Ok(result)
    }

    /// Get a relative path from the sandbox root if possible.
    pub fn get_relative_path<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        let validated = self.validate_path(&path)?;
        let canonical_root = self.sandbox_root.canonicalize()?;

        validated
            .strip_prefix(&canonical_root)
            .map(|p| p.to_path_buf())
            .map_err(|_| {
                AppError::Security(format!(
                    "Path '{}' is not within sandbox",
                    path.as_ref().display()
                ))
            })
    }

    /// Load security configuration from a JSON file.
    ///
    /// If the file doesn't exist, returns the default configuration.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| AppError::Security(format!("Failed to read security config: {}", e)))?;

        let mut config: Self = serde_json::from_str(&content)
            .map_err(|e| AppError::Security(format!("Failed to parse security config: {}", e)))?;

        // Resolve relative sandbox_root to absolute path
        if config.sandbox_root == PathBuf::from(".") || config.sandbox_root.is_relative() {
            config.sandbox_root = env::current_dir()
                .map_err(|e| AppError::Security(format!("Failed to get current directory: {}", e)))?;
        }

        Ok(config)
    }

    /// Save security configuration to a JSON file.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::Security(format!("Failed to serialize security config: {}", e)))?;

        fs::write(path, content)
            .map_err(|e| AppError::Security(format!("Failed to write security config: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_sandbox_within_allowed() {
        let temp_dir = env::temp_dir().join("gopenpal_test");
        fs::create_dir_all(&temp_dir).unwrap();

        let config = SecurityConfig::new(temp_dir.clone());

        // Should allow paths within the sandbox
        let test_path = temp_dir.join("test.txt");
        assert!(config.is_path_safe(&test_path));

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_sandbox_parent_directory() {
        let temp_dir = env::temp_dir().join("gopenpal_test");
        fs::create_dir_all(&temp_dir).unwrap();

        let config = SecurityConfig::new(temp_dir.clone());

        // Should deny parent directory access
        let parent_path = temp_dir.join("..").join("outside.txt");
        assert!(!config.is_path_safe(&parent_path));

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_sandbox_absolute_outside() {
        let temp_dir = env::temp_dir().join("gopenpal_test");
        fs::create_dir_all(&temp_dir).unwrap();

        let config = SecurityConfig::new(temp_dir.clone());

        // Should deny absolute paths outside sandbox
        let outside_path = PathBuf::from("/etc/passwd");
        assert!(!config.is_path_safe(&outside_path));

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_disable_sandbox() {
        let temp_dir = env::temp_dir().join("gopenpal_test");
        fs::create_dir_all(&temp_dir).unwrap();

        let mut config = SecurityConfig::new(temp_dir.clone());
        config.enforce_sandbox = false;

        // Should allow any path when sandboxing is disabled
        let outside_path = PathBuf::from("/tmp/outside.txt");
        assert!(config.is_path_safe(&outside_path));

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
