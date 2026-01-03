//! Agent daemon - background service for proactive agent interactions.
//!
//! This service runs in the background and triggers random agent messages
//! based on time, patterns, and user behavior.

use notify_rust::Notification;
use rand::Rng;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use crate::agents::AgentSystem;
use crate::db::Database;
use crate::error::Result;

/// Agent daemon configuration.
pub struct AgentDaemonConfig {
    /// Base interval between agent checks (in minutes).
    pub base_interval_minutes: u64,
    /// Randomness factor (0.0 to 1.0) for varying intervals.
    pub randomness: f64,
    /// Probability of sending a message on each check (0.0 to 1.0).
    pub message_probability: f64,
}

impl Default for AgentDaemonConfig {
    fn default() -> Self {
        Self {
            base_interval_minutes: 45, // Check every ~45 minutes
            randomness: 0.3,             // ±30% variation
            message_probability: 0.4,    // 40% chance on each check
        }
    }
}

/// Agent daemon service.
pub struct AgentDaemon {
    agent_system: AgentSystem,
    config: AgentDaemonConfig,
}

impl AgentDaemon {
    /// Create a new agent daemon.
    pub fn new(db: Database, config: AgentDaemonConfig) -> Self {
        Self {
            agent_system: AgentSystem::new(db),
            config,
        }
    }

    /// Create daemon with default configuration.
    pub fn with_defaults(db: Database) -> Self {
        Self::new(db, AgentDaemonConfig::default())
    }

    /// Run the daemon (blocks indefinitely).
    pub async fn run(&self) -> Result<()> {
        info!("🌊 Hydrix awakens... The agent daemon is now running.");

        // Send initial greeting
        self.send_agent_message("Hydrix", "greeting").await?;

        let mut rng = rand::thread_rng();
        let mut loop_count = 0u64;

        loop {
            // Calculate next interval with randomness
            let base_interval_secs = self.config.base_interval_minutes * 60;
            let randomness_range = (base_interval_secs as f64 * self.config.randomness) as u64;
            let offset = rng.gen_range(0..randomness_range * 2) - randomness_range;
            let interval = Duration::from_secs((base_interval_secs as i64 + offset as i64) as u64);

            info!(
                "Next agent check in {} minutes",
                interval.as_secs() / 60
            );

            time::sleep(interval).await;

            // Roll for message
            if rng.gen_bool(self.config.message_probability) {
                if let Err(e) = self.trigger_proactive_interaction("Hydrix").await {
                    error!("Failed to send proactive message: {}", e);
                }
            }

            // Check for achievements periodically (every 10th iteration, approximately every ~7.5 hours)
            if loop_count.is_multiple_of(10) {
                if let Err(e) = self.agent_system.auto_check_achievements().await {
                    error!("Failed to check achievements: {}", e);
                }
            }

            loop_count += 1;
        }
    }

    /// Trigger a proactive interaction from an agent.
    async fn trigger_proactive_interaction(&self, agent_name: &str) -> Result<()> {
        info!("🌊 {} is reaching out...", agent_name);

        // Generate proactive message based on current context
        if let Some(message) = self
            .agent_system
            .generate_proactive_message(agent_name)
            .await?
        {
            self.send_notification(agent_name, &message)?;
            info!("{}: {}", agent_name, message);
        }

        Ok(())
    }

    /// Send a specific type of agent message.
    async fn send_agent_message(&self, agent_name: &str, message_type: &str) -> Result<()> {
        let agent = self
            .agent_system
            .get_agent(agent_name)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound(format!("Agent: {}", agent_name)))?;

        if let Some(message) = self
            .agent_system
            .get_message(
                agent_name,
                message_type,
                Some(&agent.current_mood),
                agent.relationship_level,
            )
            .await?
        {
            self.send_notification(agent_name, &message)?;
            info!("{}: {}", agent_name, message);

            // Log the interaction
            self.agent_system
                .log_interaction(agent_name, message_type, &message, &agent.current_mood, 1)
                .await?;
        }

        Ok(())
    }

    /// Send desktop notification.
    fn send_notification(&self, agent_name: &str, message: &str) -> Result<()> {
        Notification::new()
            .summary(&format!("💧 {}", agent_name))
            .body(message)
            .icon("💧")
            .timeout(10000) // 10 seconds
            .show()
            .map_err(|e| crate::error::AppError::Notification(e.to_string()))?;

        Ok(())
    }

    /// One-time message trigger (for testing or manual use).
    pub async fn send_one_message(&self, agent_name: &str) -> Result<String> {
        if let Some(message) = self
            .agent_system
            .generate_proactive_message(agent_name)
            .await?
        {
            self.send_notification(agent_name, &message)?;
            Ok(message)
        } else {
            Ok("No message generated".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_daemon_creation() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();
        let _daemon = AgentDaemon::with_defaults(db);
    }
}
