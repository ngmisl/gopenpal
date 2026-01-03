//! Cron job management module.
//!
//! This module provides functionality to manage crontab entries for
//! automated water reminders.

use crate::error::{AppError, Result};
use std::process::Command;
use tracing::{error, info};

/// Cron job manager for gopenpal reminders.
pub struct CronManager {
    gopenpal_bin: String,
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

        Self { gopenpal_bin: bin }
    }

    /// Check if a gopenpal reminder cron job is installed.
    ///
    /// # Returns
    ///
    /// `Ok(Some(cron_line))` if installed, `Ok(None)` if not installed
    ///
    /// # Errors
    ///
    /// Returns error if crontab command fails
    pub fn is_installed(&self) -> Result<Option<String>> {
        let output = Command::new("crontab")
            .arg("-l")
            .output()
            .map_err(AppError::Io)?;

        if !output.status.success() {
            // No crontab installed yet
            return Ok(None);
        }

        let crontab = String::from_utf8_lossy(&output.stdout);

        // Look for gopenpal reminder check entries
        for line in crontab.lines() {
            if line.contains("gopenpal") && line.contains("reminder check") {
                return Ok(Some(line.to_string()));
            }
        }

        Ok(None)
    }

    /// Install a cron job for water reminders.
    ///
    /// # Arguments
    ///
    /// * `schedule` - Cron schedule expression (e.g., "*/30 * * * *" for every 30 minutes)
    ///
    /// # Errors
    ///
    /// Returns error if crontab modification fails
    pub fn install(&self, schedule: &str) -> Result<()> {
        // Get current crontab
        let current = self.get_crontab()?;

        // Check if already installed
        if self.is_installed()?.is_some() {
            return Err(AppError::Config(
                "Gopenpal cron job already installed. Remove it first.".to_string(),
            ));
        }

        // Add new entry
        let new_entry = format!("{} {} reminder check", schedule, self.gopenpal_bin);
        let mut new_crontab = current;
        if !new_crontab.is_empty() && !new_crontab.ends_with('\n') {
            new_crontab.push('\n');
        }
        new_crontab.push_str(&new_entry);
        new_crontab.push('\n');

        // Install new crontab
        self.set_crontab(&new_crontab)?;

        info!("Installed cron job: {}", new_entry);
        Ok(())
    }

    /// Remove the gopenpal cron job.
    ///
    /// # Errors
    ///
    /// Returns error if crontab modification fails
    pub fn remove(&self) -> Result<()> {
        let current = self.get_crontab()?;

        // Filter out gopenpal reminder check lines
        let new_crontab: String = current
            .lines()
            .filter(|line| !(line.contains("gopenpal") && line.contains("reminder check")))
            .collect::<Vec<_>>()
            .join("\n");

        // Add trailing newline if not empty
        let new_crontab = if new_crontab.is_empty() {
            String::new()
        } else {
            format!("{}\n", new_crontab)
        };

        self.set_crontab(&new_crontab)?;

        info!("Removed gopenpal cron job");
        Ok(())
    }

    /// Update the cron schedule.
    ///
    /// # Arguments
    ///
    /// * `new_schedule` - New cron schedule expression
    ///
    /// # Errors
    ///
    /// Returns error if crontab modification fails or job not installed
    pub fn update_schedule(&self, new_schedule: &str) -> Result<()> {
        // Remove old job
        self.remove()?;

        // Install with new schedule
        self.install(new_schedule)?;

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
            stdin
                .write_all(content.as_bytes())
                .map_err(AppError::Io)?;
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
