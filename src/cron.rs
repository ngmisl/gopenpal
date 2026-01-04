//! Cron job management module.
//!
//! This module provides functionality to manage crontab entries for
//! automated water reminders.

use crate::error::{AppError, Result};
use std::process::Command;
use tracing::{error, info};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronConfig {
    pub schedule: Option<String>,
    pub installed: bool,
}

/// Cron job manager for gopenpal reminders.
pub struct CronManager {
    gopenpal_bin: String,
    config_path: PathBuf,
}

impl CronManager {
    /// Create a new cron manager.
    ///
    /// # Arguments
    ///
    /// * `gopenpal_bin` - Path to the gopenpal binary (default: uses `which`)
    pub fn new(gopenpal_bin: Option<String>) -> Self {
        let bin = gopenpal_bin.unwrap_or_else(|| {
            // Try to find gopenpal in PATH
            which_gopenpal().unwrap_or_else(|| "$HOME/.cargo/bin/gopenpal".to_string())
        });

        Self {
            gopenpal_bin: bin,
            config_path: PathBuf::from("world/cron.json"),
        }
    }

    /// Load config from file.
    fn load_config(&self) -> Result<CronConfig> {
        if !self.config_path.exists() {
            return Ok(CronConfig {
                schedule: None,
                installed: false,
            });
        }
        let content = fs::read_to_string(&self.config_path)?;
        let config: CronConfig = serde_json::from_str(&content).unwrap_or(CronConfig {
            schedule: None,
            installed: false,
        });
        Ok(config)
    }

    /// Save config to file.
    fn save_config(&self, config: &CronConfig) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content =
            serde_json::to_string_pretty(config).map_err(|e| AppError::Config(e.to_string()))?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    /// Sync system crontab with local config (idempotent).
    pub fn sync(&self) -> Result<()> {
        let config = self.load_config()?;
        if config.installed {
            if let Some(schedule) = &config.schedule {
                // Force install/update
                self.apply_crontab(schedule)?;
            }
        } else {
            // Ensure removed
            self.remove_crontab()?;
        }
        Ok(())
    }

    /// Check if a gopenpal reminder cron job is installed (in config).
    pub fn is_installed(&self) -> Result<Option<String>> {
        let config = self.load_config()?;
        if config.installed {
            Ok(config.schedule)
        } else {
            Ok(None)
        }
    }

    /// Install a cron job for water reminders.
    pub fn install(&self, schedule: &str) -> Result<()> {
        // validate schedule simple check
        if schedule.split_whitespace().count() != 5 {
            return Err(AppError::Config("Invalid cron schedule format".to_string()));
        }

        self.apply_crontab(schedule)?;

        // Save state
        let config = CronConfig {
            schedule: Some(schedule.to_string()),
            installed: true,
        };
        self.save_config(&config)?;

        info!("Installed cron job: {}", schedule);
        Ok(())
    }

    /// Remove the gopenpal cron job.
    pub fn remove(&self) -> Result<()> {
        self.remove_crontab()?;

        // Save state
        let config = CronConfig {
            schedule: None,
            installed: false,
        };
        self.save_config(&config)?;

        info!("Removed gopenpal cron job");
        Ok(())
    }

    /// Update the cron schedule.
    pub fn update_schedule(&self, new_schedule: &str) -> Result<()> {
        self.install(new_schedule)
    }

    /// Internal: Apply to system crontab
    fn apply_crontab(&self, schedule: &str) -> Result<()> {
        let current = self.get_crontab()?;

        // Remove existing gopenpal lines to avoid duplicates
        let mut lines: Vec<String> = current
            .lines()
            .filter(|line| !(line.contains("gopenpal") && line.contains("reminder check")))
            .map(|s| s.to_string())
            .collect();

        // Add new line
        let new_entry = format!("{} {} reminder check", schedule, self.gopenpal_bin);
        lines.push(new_entry);

        // Reassemble
        let new_crontab = lines.join("\n") + "\n";
        self.set_crontab(&new_crontab)?;
        Ok(())
    }

    /// Internal: Remove from system crontab
    fn remove_crontab(&self) -> Result<()> {
        let current = self.get_crontab()?;
        let new_crontab: String = current
            .lines()
            .filter(|line| !(line.contains("gopenpal") && line.contains("reminder check")))
            .collect::<Vec<_>>()
            .join("\n");

        let final_crontab = if new_crontab.is_empty() {
            String::new()
        } else {
            new_crontab + "\n"
        };

        self.set_crontab(&final_crontab)?;
        Ok(())
    }

    /// Get common cron schedule presets.
    pub fn presets() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Every 15 minutes", "*/15 * * * *"),
            ("Every 30 minutes", "*/30 * * * *"),
            ("Every hour", "0 * * * *"),
            ("Every 2 hours", "0 */2 * * *"),
            ("Work hours only (9-5, hourly)", "0 9-17 * * 1-5"),
            ("Work hours only (9-5, every 30min)", "*/30 9-17 * * 1-5"),
        ]
    }

    /// Get the current crontab contents.
    fn get_crontab(&self) -> Result<String> {
        let output = Command::new("crontab").arg("-l").output();

        match output {
            Ok(out) if out.status.success() => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
            Ok(_) => Ok(String::new()), // No crontab exists yet
            Err(e) => {
                error!("Failed to read crontab: {}", e);
                Err(AppError::Io(e))
            }
        }
    }

    /// Set the crontab contents.
    fn set_crontab(&self, content: &str) -> Result<()> {
        let mut child = Command::new("crontab")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(AppError::Io)?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(content.as_bytes()).map_err(AppError::Io)?;
        }

        let status = child.wait().map_err(AppError::Io)?;

        if !status.success() {
            return Err(AppError::Config("Failed to update crontab".to_string()));
        }

        Ok(())
    }
}

/// Find the gopenpal binary in PATH.
fn which_gopenpal() -> Option<String> {
    let output = Command::new("which").arg("gopenpal").output().ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presets() {
        let presets = CronManager::presets();
        assert!(!presets.is_empty());
        assert!(presets.iter().any(|(_, sched)| sched.contains("*/30")));
    }

    #[test]
    fn test_cron_manager_creation() {
        let manager = CronManager::new(Some("/usr/bin/gopenpal".to_string()));
        assert_eq!(manager.gopenpal_bin, "/usr/bin/gopenpal");
    }
}
