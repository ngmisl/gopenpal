//! Database operations and types.
//!
//! This module handles all SQLite database interactions using sqlx,
//! including water intake tracking, chat history, and settings management.

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::{FromRow, Row};
use std::path::Path;
use std::str::FromStr;

use crate::error::Result;

/// Database connection pool wrapper.
///
/// This struct provides a typed interface to the SQLite database pool
/// and exposes methods for all database operations.
#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection.
    ///
    /// # Arguments
    ///
    /// * `database_url` - Path to the SQLite database file
    ///
    /// # Returns
    ///
    /// A new `Database` instance with connection pool
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if connection fails
    pub async fn new(database_url: &str) -> Result<Self> {
        // Create database file if it doesn't exist
        // The `Path` check ensures the parent directory exists
        if let Some(parent) = Path::new(database_url.trim_start_matches("sqlite://")).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Run database migrations.
    ///
    /// This executes all SQL migration files to set up the schema.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if migrations fail
    pub async fn migrate(&self) -> Result<()> {
        // Read and execute the migration file
        let migration_sql = include_str!("../migrations/20260103_initial_schema.sql");
        sqlx::query(migration_sql).execute(&self.pool).await?;
        Ok(())
    }

    /// Record water intake.
    ///
    /// # Arguments
    ///
    /// * `amount_ml` - Amount of water consumed in milliliters
    /// * `notes` - Optional notes about the intake
    ///
    /// # Returns
    ///
    /// The ID of the created record
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if insert fails
    pub async fn record_water_intake(&self, amount_ml: i64, notes: Option<&str>) -> Result<i64> {
        let result = sqlx::query("INSERT INTO water_intake (amount_ml, notes) VALUES (?, ?)")
            .bind(amount_ml)
            .bind(notes)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get water intake for today.
    ///
    /// # Returns
    ///
    /// Vector of `WaterIntake` records for the current day
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_today_water_intake(&self) -> Result<Vec<WaterIntake>> {
        let records = sqlx::query_as::<_, WaterIntake>(
            r#"
            SELECT id, timestamp, amount_ml, notes
            FROM water_intake
            WHERE DATE(timestamp) = DATE('now')
            ORDER BY timestamp DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    /// Get total water intake for today in milliliters.
    ///
    /// # Returns
    ///
    /// Total amount in milliliters
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_today_total_ml(&self) -> Result<i64> {
        let row = sqlx::query(
            r#"
            SELECT COALESCE(SUM(amount_ml), 0) as total
            FROM water_intake
            WHERE DATE(timestamp) = DATE('now')
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        let total: i64 = row.try_get("total")?;
        Ok(total)
    }

    /// Get reminder settings.
    ///
    /// # Returns
    ///
    /// Current `ReminderSettings`
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_reminder_settings(&self) -> Result<ReminderSettings> {
        let settings = sqlx::query_as::<_, ReminderSettings>(
            r#"
            SELECT
                enabled,
                interval_minutes,
                work_hours_start,
                work_hours_end,
                work_days
            FROM reminder_settings
            WHERE id = 1
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(settings)
    }

    /// Update reminder settings.
    ///
    /// # Arguments
    ///
    /// * `settings` - New settings to apply
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if update fails
    pub async fn update_reminder_settings(&self, settings: &ReminderSettings) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE reminder_settings
            SET enabled = ?, interval_minutes = ?, work_hours_start = ?,
                work_hours_end = ?, work_days = ?
            WHERE id = 1
            "#,
        )
        .bind(settings.enabled)
        .bind(settings.interval_minutes)
        .bind(settings.work_hours_start)
        .bind(settings.work_hours_end)
        .bind(&settings.work_days)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Save a chat message to history.
    ///
    /// # Arguments
    ///
    /// * `role` - Message role (user, assistant, or system)
    /// * `content` - Message content
    /// * `model` - Optional model name used
    /// * `tokens_used` - Optional token count
    ///
    /// # Returns
    ///
    /// The ID of the created record
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if insert fails
    pub async fn save_chat_message(
        &self,
        role: &str,
        content: &str,
        model: Option<&str>,
        tokens_used: Option<i64>,
    ) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO chat_history (role, content, model, tokens_used) VALUES (?, ?, ?, ?)",
        )
        .bind(role)
        .bind(content)
        .bind(model)
        .bind(tokens_used)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get recent chat history.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of messages to retrieve
    ///
    /// # Returns
    ///
    /// Vector of recent `ChatMessage` records
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_recent_chat_history(&self, limit: i64) -> Result<Vec<ChatMessage>> {
        let messages = sqlx::query_as::<_, ChatMessage>(
            r#"
            SELECT id, timestamp, role, content, model, tokens_used
            FROM chat_history
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        // Reverse to get chronological order
        // This is more intuitive for chat display
        Ok(messages.into_iter().rev().collect())
    }

    /// Clear all chat history.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if deletion fails
    pub async fn clear_chat_history(&self) -> Result<()> {
        sqlx::query("DELETE FROM chat_history")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

/// Water intake record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WaterIntake {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub amount_ml: i64,
    pub notes: Option<String>,
}

/// Reminder settings configuration.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReminderSettings {
    pub enabled: bool,
    pub interval_minutes: i64,
    pub work_hours_start: NaiveTime,
    pub work_hours_end: NaiveTime,
    pub work_days: String,
}

/// Chat message record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub role: String,
    pub content: String,
    pub model: Option<String>,
    pub tokens_used: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_creation() {
        let db = Database::new("sqlite::memory:")
            .await
            .expect("Failed to create database");
        db.migrate().await.expect("Failed to run migrations");
    }

    #[tokio::test]
    async fn test_water_intake() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();

        let id = db
            .record_water_intake(250, Some("Morning water"))
            .await
            .unwrap();
        assert!(id > 0);

        let total = db.get_today_total_ml().await.unwrap();
        assert_eq!(total, 250);
    }

    #[tokio::test]
    async fn test_reminder_settings() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();

        let settings = db.get_reminder_settings().await.unwrap();
        assert!(settings.enabled);
        assert_eq!(settings.interval_minutes, 60);
    }

    #[tokio::test]
    async fn test_chat_history() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();

        db.save_chat_message("user", "Hello!", None, None)
            .await
            .unwrap();
        db.save_chat_message("assistant", "Hi there!", Some("gpt-4"), Some(50))
            .await
            .unwrap();

        let history = db.get_recent_chat_history(10).await.unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].role, "user");
        assert_eq!(history[1].role, "assistant");
    }
}
