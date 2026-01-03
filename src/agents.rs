//! Agent persona system - living AI characters with lore and personality.
//!
//! This module implements the agent world system where AI characters like Hydrix
//! have personalities, moods, backstories, and can proactively interact with users.

use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::Database;
use crate::error::Result;

/// Agent persona with personality and state.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Agent {
    pub id: i64,
    pub name: String,
    pub title: String,
    pub personality_type: String,
    pub current_mood: String,
    pub relationship_level: i64,
    pub last_interaction: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub backstory: Option<String>,
    pub current_state: Option<String>,
}

/// Agent mood definition.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentMood {
    pub id: i64,
    pub agent_name: String,
    pub mood_name: String,
    pub description: Option<String>,
    pub trigger_condition: Option<String>,
    pub message_tone: Option<String>,
}

/// Message from the agent's library.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentMessage {
    pub id: i64,
    pub agent_name: String,
    pub message_type: String,
    pub mood: Option<String>,
    pub content: String,
    pub context_condition: Option<String>,
    pub rarity: Option<String>,
    pub unlock_level: Option<i64>,
}

/// Agent interaction record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentInteraction {
    pub id: i64,
    pub agent_name: String,
    pub timestamp: DateTime<Utc>,
    pub interaction_type: String,
    pub message: Option<String>,
    pub mood: Option<String>,
    pub user_response: Option<String>,
    pub relationship_delta: Option<i64>,
}

/// World lore entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorldLore {
    pub id: i64,
    pub category: String,
    pub title: String,
    pub content: String,
    pub unlock_condition: Option<String>,
    pub unlock_level: Option<i64>,
    pub unlocked: bool,
    pub unlocked_at: Option<DateTime<Utc>>,
}

/// Achievement definition.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Achievement {
    pub id: i64,
    pub achievement_name: String,
    pub description: Option<String>,
    pub unlocked: bool,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub trigger_condition: Option<String>,
}

/// Agent system manager.
pub struct AgentSystem {
    db: Database,
}

impl AgentSystem {
    /// Create a new agent system.
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Get an agent by name.
    pub async fn get_agent(&self, name: &str) -> Result<Option<Agent>> {
        let agent = sqlx::query_as::<_, Agent>(
            "SELECT * FROM agents WHERE name = ?",
        )
        .bind(name)
        .fetch_optional(self.db.pool())
        .await?;

        Ok(agent)
    }

    /// Update agent mood.
    pub async fn update_agent_mood(&self, name: &str, new_mood: &str) -> Result<()> {
        sqlx::query(
            "UPDATE agents SET current_mood = ?, last_interaction = CURRENT_TIMESTAMP WHERE name = ?",
        )
        .bind(new_mood)
        .bind(name)
        .execute(self.db.pool())
        .await?;

        Ok(())
    }

    /// Increase agent relationship level.
    pub async fn increase_relationship(&self, name: &str, amount: i64) -> Result<i64> {
        sqlx::query(
            "UPDATE agents SET relationship_level = relationship_level + ? WHERE name = ?",
        )
        .bind(amount)
        .bind(name)
        .execute(self.db.pool())
        .await?;

        // Get new level
        let agent = self.get_agent(name).await?;
        Ok(agent.map(|a| a.relationship_level).unwrap_or(0))
    }

    /// Get appropriate message for context.
    pub async fn get_message(
        &self,
        agent_name: &str,
        message_type: &str,
        mood: Option<&str>,
        relationship_level: i64,
    ) -> Result<Option<String>> {
        let mut query = String::from(
            "SELECT * FROM agent_message_library \
             WHERE agent_name = ? AND message_type = ? \
             AND (unlock_level IS NULL OR unlock_level <= ?)",
        );

        let mut bindings = vec![
            agent_name.to_string(),
            message_type.to_string(),
            relationship_level.to_string(),
        ];

        if let Some(mood_val) = mood {
            query.push_str(" AND (mood IS NULL OR mood = ?)");
            bindings.push(mood_val.to_string());
        }

        let messages = sqlx::query_as::<_, AgentMessage>(&query)
            .bind(&bindings[0])
            .bind(&bindings[1])
            .bind(&bindings[2])
            .fetch_all(self.db.pool())
            .await?;

        if messages.is_empty() {
            return Ok(None);
        }

        // Weight selection by rarity
        let mut rng = rand::thread_rng();
        let selected = messages.choose(&mut rng);

        Ok(selected.map(|m| m.content.clone()))
    }

    /// Log an agent interaction.
    pub async fn log_interaction(
        &self,
        agent_name: &str,
        interaction_type: &str,
        message: &str,
        mood: &str,
        relationship_delta: i64,
    ) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO agent_interactions \
             (agent_name, interaction_type, message, mood, relationship_delta) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(agent_name)
        .bind(interaction_type)
        .bind(message)
        .bind(mood)
        .bind(relationship_delta)
        .execute(self.db.pool())
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Check and unlock lore based on relationship level.
    pub async fn check_lore_unlocks(&self, relationship_level: i64) -> Result<Vec<WorldLore>> {
        let newly_unlocked = sqlx::query_as::<_, WorldLore>(
            "SELECT * FROM world_lore \
             WHERE unlocked = 0 AND unlock_level <= ? \
             ORDER BY unlock_level ASC",
        )
        .bind(relationship_level)
        .fetch_all(self.db.pool())
        .await?;

        // Mark as unlocked
        for lore in &newly_unlocked {
            sqlx::query(
                "UPDATE world_lore SET unlocked = 1, unlocked_at = CURRENT_TIMESTAMP WHERE id = ?",
            )
            .bind(lore.id)
            .execute(self.db.pool())
            .await?;
        }

        Ok(newly_unlocked)
    }

    /// Get all unlocked lore.
    pub async fn get_unlocked_lore(&self) -> Result<Vec<WorldLore>> {
        let lore = sqlx::query_as::<_, WorldLore>(
            "SELECT * FROM world_lore WHERE unlocked = 1 ORDER BY unlocked_at DESC",
        )
        .fetch_all(self.db.pool())
        .await?;

        Ok(lore)
    }

    /// Check and unlock achievements.
    pub async fn check_achievement(&self, trigger: &str) -> Result<Option<Achievement>> {
        let achievement = sqlx::query_as::<_, Achievement>(
            "SELECT * FROM achievements WHERE trigger_condition = ? AND unlocked = 0",
        )
        .bind(trigger)
        .fetch_optional(self.db.pool())
        .await?;

        if let Some(ach) = &achievement {
            sqlx::query(
                "UPDATE achievements SET unlocked = 1, unlocked_at = CURRENT_TIMESTAMP WHERE id = ?",
            )
            .bind(ach.id)
            .execute(self.db.pool())
            .await?;
        }

        Ok(achievement)
    }

    /// Determine mood based on user's hydration patterns.
    pub async fn determine_mood(
        &self,
        agent_name: &str,
    ) -> Result<String> {
        // Get recent water intake stats
        let recent_total = self.db.get_today_total_ml().await?;
        let last_intake = self.db.get_today_water_intake().await?;

        let mood = if recent_total >= 2000 {
            "joyful"
        } else if last_intake.is_empty() ||
                   (Utc::now() - last_intake[0].timestamp).num_hours() >= 3 {
            "concerned"
        } else if recent_total >= 1500 {
            "proud"
        } else if rand::thread_rng().gen_bool(0.1) {
            // 10% chance of random special mood
            match rand::thread_rng().gen_range(0..3) {
                0 => "nostalgic",
                1 => "contemplative",
                _ => "playful",
            }
        } else {
            "hopeful"
        };

        self.update_agent_mood(agent_name, mood).await?;
        Ok(mood.to_string())
    }

    /// Generate a proactive message from the agent.
    pub async fn generate_proactive_message(&self, agent_name: &str) -> Result<Option<String>> {
        let agent = match self.get_agent(agent_name).await? {
            Some(a) => a,
            None => return Ok(None),
        };

        // Determine current mood based on patterns
        let mood = self.determine_mood(agent_name).await?;

        // Select message type based on context
        let message_type = self.select_message_type(&mood).await?;

        // Get appropriate message
        let mut message = self
            .get_message(agent_name, &message_type, Some(&mood), agent.relationship_level)
            .await?;

        // If we got a message, personalize it and log the interaction
        if let Some(ref mut msg) = message {
            *msg = self.personalize_message(msg).await?;

            // Log the interaction
            let relationship_gain = match message_type.as_str() {
                "celebration" => 5,
                "encouragement" => 2,
                "lore" => 3,
                _ => 1,
            };

            self.log_interaction(agent_name, &message_type, msg, &mood, relationship_gain)
                .await?;

            // Increase relationship
            let new_level = self.increase_relationship(agent_name, relationship_gain).await?;

            // Check for lore unlocks
            let new_lore = self.check_lore_unlocks(new_level).await?;
            if !new_lore.is_empty() {
                msg.push_str(&format!(
                    "\n\n✨ [LORE UNLOCKED: {}]",
                    new_lore[0].title
                ));
            }
        }

        Ok(message)
    }

    /// Select appropriate message type based on context.
    async fn select_message_type(&self, mood: &str) -> Result<String> {
        let mut rng = rand::thread_rng();

        let message_type = match mood {
            "joyful" => {
                if rng.gen_bool(0.7) {
                    "celebration"
                } else {
                    "random"
                }
            }
            "concerned" => "concern",
            "proud" => "encouragement",
            "nostalgic" | "contemplative" => {
                if rng.gen_bool(0.6) {
                    "lore"
                } else {
                    "random"
                }
            }
            "playful" => "random",
            _ => {
                // Default mood - mix it up
                match rng.gen_range(0..5) {
                    0 => "greeting",
                    1 => "encouragement",
                    2 => "random",
                    3 => "observation",
                    _ => "lore",
                }
            }
        };

        Ok(message_type.to_string())
    }

    /// Personalize message with user data.
    async fn personalize_message(&self, message: &str) -> Result<String> {
        let mut personalized = message.to_string();

        // Replace placeholders with actual data
        if personalized.contains("{hour}") {
            let patterns = self.db.get_hourly_patterns().await?;
            if let Some(top_pattern) = patterns.iter().max_by_key(|p| p.intake_count) {
                personalized = personalized.replace("{hour}", &top_pattern.hour.to_string());
            }
        }

        if personalized.contains("{day}") {
            let patterns = self.db.get_weekly_patterns().await?;
            if let Some(top_pattern) = patterns.iter().max_by_key(|p| p.intake_count) {
                personalized = personalized.replace("{day}", &top_pattern.day_name);
            }
        }

        Ok(personalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_system_creation() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();
        let agent_system = AgentSystem::new(db);

        let agent = agent_system.get_agent("Hydrix").await.unwrap();
        assert!(agent.is_some());
    }
}
