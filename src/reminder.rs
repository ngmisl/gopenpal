//! Water reminder service.
//!
//! This module implements a background service that sends desktop notifications
//! to remind users to drink water during work hours.

use chrono::{Local, Timelike};
use notify_rust::Notification;
use tokio::time::{interval, Duration};
use tracing::{error, info};

use crate::db::Database;
use crate::error::Result;

/// Water reminder service.
///
/// This service runs in the background and sends periodic notifications
/// based on configured reminder settings and work schedule.
pub struct ReminderService {
    db: Database,
}

impl ReminderService {
    /// Create a new reminder service.
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection for accessing settings
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Start the reminder service.
    ///
    /// This method runs indefinitely, checking settings and sending notifications
    /// at the configured interval. It respects work hours and enabled/disabled state.
    ///
    /// # Errors
    ///
    /// Logs errors but continues running even if individual operations fail
    pub async fn run(&self) -> Result<()> {
        info!("Starting water reminder service");

        // Check every minute for whether to send a reminder
        // This provides responsiveness to settings changes while being efficient
        let mut check_interval = interval(Duration::from_secs(60));

        let mut last_reminder_minute: Option<u32> = None;

        loop {
            check_interval.tick().await;

            // Fetch current settings
            let settings = match self.db.get_reminder_settings().await {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to fetch reminder settings: {}", e);
                    continue;
                }
            };

            if !settings.enabled {
                continue;
            }

            let now = Local::now();
            let current_time = now.time();
            let current_minute = now.minute();

            // Check if we're within work hours
            if current_time < settings.work_hours_start || current_time >= settings.work_hours_end {
                continue;
            }

            // Check if it's a work day
            let weekday = now.format("%a").to_string();
            if !settings.work_days.contains(&weekday) {
                continue;
            }

            // Check if enough time has passed since last reminder
            // We use minute-based tracking to avoid sending multiple reminders per minute
            if let Some(last_minute) = last_reminder_minute {
                let minutes_since_last = if current_minute >= last_minute {
                    current_minute - last_minute
                } else {
                    // Handle hour wraparound
                    60 - last_minute + current_minute
                };

                if (minutes_since_last as i64) < settings.interval_minutes {
                    continue;
                }
            }

            // Send reminder notification
            if let Err(e) = self.send_reminder().await {
                error!("Failed to send reminder: {}", e);
            } else {
                last_reminder_minute = Some(current_minute);
                info!("Sent water reminder at {}", now.format("%H:%M"));
            }
        }
    }

    /// Send a water reminder notification.
    ///
    /// # Errors
    ///
    /// Returns error if notification fails to send or database query fails
    async fn send_reminder(&self) -> Result<()> {
        // Get today's water intake
        let total_ml = self.db.get_today_total_ml().await?;
        let total_liters = total_ml as f64 / 1000.0;

        let message = if total_ml == 0 {
            "Time to drink some water! You haven't logged any water today.".to_string()
        } else {
            format!(
                "Time to drink some water! You've had {:.1}L today.",
                total_liters
            )
        };

        Notification::new()
            .summary("Water Reminder")
            .body(&message)
            .icon("dialog-information")
            .timeout(5000)
            .show()
            .map_err(|e| crate::error::AppError::Notification(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reminder_service_creation() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();

        let service = ReminderService::new(db);
        // Just verify we can create the service
        assert!(std::mem::size_of_val(&service) > 0);
    }
}
