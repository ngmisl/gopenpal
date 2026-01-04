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
    /// Get a reference to the underlying connection pool.
    ///
    /// This is used by other modules that need direct pool access.
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

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
        // Read and execute the migration files
        let migration_sql = include_str!("../migrations/20260103_initial_schema.sql");
        sqlx::query(migration_sql).execute(&self.pool).await?;

        let activity_logging_sql = include_str!("../migrations/20260103_activity_logging.sql");
        sqlx::query(activity_logging_sql)
            .execute(&self.pool)
            .await?;

        let agent_world_sql = include_str!("../migrations/20260103_agent_world.sql");
        sqlx::query(agent_world_sql)
            .execute(&self.pool)
            .await?;

        let goals_sql = include_str!("../migrations/20260104_goals_tracking.sql");
        sqlx::query(goals_sql)
            .execute(&self.pool)
            .await?;

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

    /// Log an activity event.
    ///
    /// # Arguments
    ///
    /// * `activity_type` - Type of activity
    /// * `details` - Optional details about the activity
    /// * `metadata` - Optional JSON metadata
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if insert fails
    #[allow(dead_code)]
    pub async fn log_activity(
        &self,
        activity_type: &str,
        details: Option<&str>,
        metadata: Option<&str>,
    ) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO activity_log (activity_type, details, metadata) VALUES (?, ?, ?)",
        )
        .bind(activity_type)
        .bind(details)
        .bind(metadata)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Log a reminder event.
    ///
    /// # Arguments
    ///
    /// * `reminder_type` - Type of reminder (cron, manual, foreground)
    /// * `action_taken` - Whether the reminder was acted upon
    /// * `water_intake_id` - Optional ID of water intake record if acted upon
    /// * `response_time_seconds` - Optional response time
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if insert fails
    #[allow(dead_code)]
    pub async fn log_reminder_event(
        &self,
        reminder_type: &str,
        action_taken: bool,
        water_intake_id: Option<i64>,
        response_time_seconds: Option<i64>,
    ) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO reminder_events
            (reminder_type, action_taken, water_intake_id, response_time_seconds)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(reminder_type)
        .bind(action_taken)
        .bind(water_intake_id)
        .bind(response_time_seconds)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get daily water statistics for the last N days.
    ///
    /// # Arguments
    ///
    /// * `days` - Number of days to include
    ///
    /// # Returns
    ///
    /// Vector of `DailyWaterStats`
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_daily_water_stats(&self, days: i64) -> Result<Vec<DailyWaterStats>> {
        let stats = sqlx::query_as::<_, DailyWaterStats>(
            r#"
            SELECT
                date,
                intake_count,
                total_ml,
                avg_ml,
                min_ml,
                max_ml
            FROM daily_water_stats
            WHERE date >= DATE('now', ? || ' days')
            ORDER BY date DESC
            "#,
        )
        .bind(format!("-{}", days))
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get hourly drinking patterns.
    ///
    /// # Returns
    ///
    /// Vector of `HourlyPattern` showing which hours user drinks most
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_hourly_patterns(&self) -> Result<Vec<HourlyPattern>> {
        let patterns = sqlx::query_as::<_, HourlyPattern>(
            r#"
            SELECT hour, intake_count, total_ml, avg_ml
            FROM hourly_patterns
            ORDER BY hour
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(patterns)
    }

    /// Get weekly drinking patterns.
    ///
    /// # Returns
    ///
    /// Vector of `WeeklyPattern` showing which days user drinks most
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_weekly_patterns(&self) -> Result<Vec<WeeklyPattern>> {
        let patterns = sqlx::query_as::<_, WeeklyPattern>(
            r#"
            SELECT day_name, day_num, intake_count, total_ml, avg_ml
            FROM weekly_patterns
            ORDER BY day_num
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(patterns)
    }

    /// Get reminder effectiveness statistics.
    ///
    /// # Returns
    ///
    /// Vector of `ReminderEffectiveness` stats by reminder type
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_reminder_effectiveness(&self) -> Result<Vec<ReminderEffectiveness>> {
        let stats = sqlx::query_as::<_, ReminderEffectiveness>(
            r#"
            SELECT
                reminder_type,
                total_reminders,
                actions_taken,
                effectiveness_percentage,
                avg_response_seconds
            FROM reminder_effectiveness
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get recent activity log entries.
    ///
    /// # Arguments
    ///
    /// * `days` - Number of days to include
    /// * `activity_type` - Optional filter by activity type
    ///
    /// # Returns
    ///
    /// Vector of `ActivityLog` entries
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    #[allow(dead_code)]
    pub async fn get_activity_log(
        &self,
        days: i64,
        activity_type: Option<&str>,
    ) -> Result<Vec<ActivityLog>> {
        let logs = if let Some(activity_type) = activity_type {
            sqlx::query_as::<_, ActivityLog>(
                r#"
                SELECT id, timestamp, activity_type, details, metadata
                FROM activity_log
                WHERE timestamp >= DATETIME('now', ? || ' days')
                  AND activity_type = ?
                ORDER BY timestamp DESC
                "#,
            )
            .bind(format!("-{}", days))
            .bind(activity_type)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, ActivityLog>(
                r#"
                SELECT id, timestamp, activity_type, details, metadata
                FROM activity_log
                WHERE timestamp >= DATETIME('now', ? || ' days')
                ORDER BY timestamp DESC
                "#,
            )
            .bind(format!("-{}", days))
            .fetch_all(&self.pool)
            .await?
        };

        Ok(logs)
    }

    /// Get comprehensive water intake statistics and recommendations.
    ///
    /// # Arguments
    ///
    /// * `days` - Number of days to analyze
    ///
    /// # Returns
    ///
    /// Formatted string with statistics and AI-friendly insights
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if queries fail
    pub async fn get_statistics_summary(&self, days: i64) -> Result<String> {
        let daily_stats = self.get_daily_water_stats(days).await?;
        let hourly_patterns = self.get_hourly_patterns().await?;
        let weekly_patterns = self.get_weekly_patterns().await?;
        let effectiveness = self.get_reminder_effectiveness().await?;

        let mut summary = format!("=== Water Intake Statistics (Last {} days) ===\n\n", days);

        // Daily statistics
        summary.push_str("Daily Totals:\n");
        for stat in &daily_stats {
            summary.push_str(&format!(
                "  {} - {}ml ({} times, avg: {}ml per intake)\n",
                stat.date, stat.total_ml, stat.intake_count, stat.avg_ml
            ));
        }

        // Overall averages
        if !daily_stats.is_empty() {
            let total_days = daily_stats.len() as i64;
            let total_intake: i64 = daily_stats.iter().map(|s| s.total_ml).sum();
            let avg_daily = total_intake / total_days;
            summary.push_str(&format!(
                "\nOverall Average: {}ml per day ({} days)\n",
                avg_daily, total_days
            ));
        }

        // Hourly patterns
        summary.push_str("\nHourly Patterns:\n");
        let top_hours: Vec<_> = {
            let mut hours = hourly_patterns.clone();
            hours.sort_by(|a, b| b.intake_count.cmp(&a.intake_count));
            hours.into_iter().take(5).collect()
        };
        for pattern in &top_hours {
            summary.push_str(&format!(
                "  {}:00 - {} intakes, {}ml total\n",
                pattern.hour, pattern.intake_count, pattern.total_ml
            ));
        }

        // Weekly patterns
        summary.push_str("\nWeekly Patterns:\n");
        for pattern in &weekly_patterns {
            summary.push_str(&format!(
                "  {} - {} intakes, {}ml total (avg: {}ml)\n",
                pattern.day_name, pattern.intake_count, pattern.total_ml, pattern.avg_ml
            ));
        }

        // Reminder effectiveness
        if !effectiveness.is_empty() {
            summary.push_str("\nReminder Effectiveness:\n");
            for eff in &effectiveness {
                summary.push_str(&format!(
                    "  {} - {:.1}% effective ({}/{} acted upon)\n",
                    eff.reminder_type,
                    eff.effectiveness_percentage,
                    eff.actions_taken,
                    eff.total_reminders
                ));
                if let Some(avg_resp) = eff.avg_response_seconds {
                    summary.push_str(&format!(
                        "    Avg response time: {}s\n",
                        avg_resp
                    ));
                }
            }
        }

        Ok(summary)
    }

    /// Get monthly statistics for water intake.
    ///
    /// # Arguments
    ///
    /// * `months` - Number of months to analyze (default 6)
    ///
    /// # Returns
    ///
    /// Formatted string with monthly statistics
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if queries fail
    pub async fn get_monthly_statistics(&self, months: i64) -> Result<String> {
        let query = r#"
            SELECT
                strftime('%Y-%m', timestamp) as month,
                COUNT(*) as intake_count,
                SUM(amount_ml) as total_ml,
                AVG(amount_ml) as avg_ml,
                COUNT(DISTINCT DATE(timestamp)) as days_logged
            FROM water_intake
            WHERE timestamp >= datetime('now', '-' || ? || ' months')
            GROUP BY month
            ORDER BY month DESC
        "#;

        #[derive(sqlx::FromRow)]
        struct MonthlyStats {
            month: String,
            intake_count: i64,
            total_ml: i64,
            avg_ml: i64,
            days_logged: i64,
        }

        let stats: Vec<MonthlyStats> = sqlx::query_as(query)
            .bind(months)
            .fetch_all(&self.pool)
            .await?;

        let mut summary = format!("=== Monthly Water Intake Statistics (Last {} months) ===\n\n", months);

        if stats.is_empty() {
            summary.push_str("No water intake data available for this period.\n");
            return Ok(summary);
        }

        for stat in &stats {
            let avg_per_day = if stat.days_logged > 0 {
                stat.total_ml / stat.days_logged
            } else {
                0
            };

            summary.push_str(&format!(
                "{}: Total: {}ml | Days logged: {} | Avg per day: {}ml | {} intakes (avg: {}ml)\n",
                stat.month,
                stat.total_ml,
                stat.days_logged,
                avg_per_day,
                stat.intake_count,
                stat.avg_ml
            ));
        }

        // Add trend analysis
        if stats.len() >= 2 {
            let recent = &stats[0];
            let previous = &stats[1];

            let recent_avg = if recent.days_logged > 0 {
                recent.total_ml / recent.days_logged
            } else {
                0
            };

            let prev_avg = if previous.days_logged > 0 {
                previous.total_ml / previous.days_logged
            } else {
                0
            };

            if prev_avg > 0 {
                let change_pct = ((recent_avg - prev_avg) as f64 / prev_avg as f64) * 100.0;
                summary.push_str(&format!(
                    "\nTrend: {:+.1}% vs previous month\n",
                    change_pct
                ));
            }
        }

        Ok(summary)
    }

    /// Compare statistics between two time periods.
    ///
    /// # Arguments
    ///
    /// * `period1_days` - Number of days for first period
    /// * `period2_start` - Start offset for second period (negative)
    /// * `period2_end` - End offset for second period (negative)
    ///
    /// # Returns
    ///
    /// Formatted comparison string
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if queries fail
    pub async fn compare_periods(&self, period1_days: i64, period2_start: i64, period2_end: i64) -> Result<String> {
        // Period 1 (most recent)
        let stats1 = self.get_daily_water_stats(period1_days).await?;

        // Period 2 (comparison period)
        let query = r#"
            SELECT
                DATE(timestamp) as date,
                COUNT(*) as intake_count,
                SUM(amount_ml) as total_ml,
                AVG(amount_ml) as avg_ml
            FROM water_intake
            WHERE timestamp >= datetime('now', ? || ' days')
              AND timestamp < datetime('now', ? || ' days')
            GROUP BY date
            ORDER BY date DESC
        "#;

        #[derive(sqlx::FromRow)]
        struct DailyStats {
            date: String,
            intake_count: i64,
            total_ml: i64,
            avg_ml: i64,
        }

        let stats2: Vec<DailyStats> = sqlx::query_as(query)
            .bind(format!("{}", period2_start))
            .bind(format!("{}", period2_end))
            .fetch_all(&self.pool)
            .await?;

        let mut summary = "=== Period Comparison ===\n\n".to_string();

        // Calculate totals and averages for period 1
        let total1: i64 = stats1.iter().map(|s| s.total_ml).sum();
        let avg1 = if !stats1.is_empty() {
            total1 / stats1.len() as i64
        } else {
            0
        };

        // Calculate totals and averages for period 2, explicitly using all fields
        let mut total2: i64 = 0;
        for stat in &stats2 {
            total2 += stat.total_ml;
            // Access all fields to avoid unused warnings
            let _ = (stat.intake_count, &stat.date, stat.avg_ml);
        }
        let avg2 = if !stats2.is_empty() {
            total2 / stats2.len() as i64
        } else {
            0
        };

        summary.push_str(&format!(
            "Period 1 (last {} days):\n  Total: {}ml | Avg per day: {}ml | Days: {}\n\n",
            period1_days, total1, avg1, stats1.len()
        ));

        summary.push_str(&format!(
            "Period 2 (comparison):\n  Total: {}ml | Avg per day: {}ml | Days: {}\n\n",
            total2, avg2, stats2.len()
        ));

        // Calculate differences
        if avg2 > 0 {
            let change_pct = ((avg1 - avg2) as f64 / avg2 as f64) * 100.0;
            let change_ml = avg1 - avg2;

            summary.push_str(&format!(
                "Change: {:+}ml per day ({:+.1}%)\n",
                change_ml, change_pct
            ));

            if change_pct > 10.0 {
                summary.push_str("📈 Significant improvement!\n");
            } else if change_pct < -10.0 {
                summary.push_str("📉 Notable decrease\n");
            } else {
                summary.push_str("📊 Relatively stable\n");
            }
        }

        Ok(summary)
    }

    /// Set or update the daily water goal.
    ///
    /// # Arguments
    ///
    /// * `target_ml` - Target water intake in milliliters
    /// * `notes` - Optional notes about the goal
    ///
    /// # Returns
    ///
    /// The ID of the created or updated goal
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if operation fails
    pub async fn set_daily_water_goal(&self, target_ml: i64, notes: Option<&str>) -> Result<i64> {
        // Deactivate any existing active daily_water goals
        sqlx::query(
            r#"
            UPDATE goals
            SET active = 0, updated_at = datetime('now')
            WHERE goal_type = 'daily_water' AND active = 1
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Insert new goal
        let result = sqlx::query(
            r#"
            INSERT INTO goals (goal_type, target_value, unit, notes)
            VALUES ('daily_water', ?, 'ml', ?)
            "#,
        )
        .bind(target_ml as f64)
        .bind(notes)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get the current active daily water goal.
    ///
    /// # Returns
    ///
    /// Optional `Goal` if one exists
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_active_daily_water_goal(&self) -> Result<Option<Goal>> {
        let goal = sqlx::query_as::<_, Goal>(
            r#"
            SELECT id, goal_type, target_value, unit, created_at, updated_at, active, notes
            FROM goals
            WHERE goal_type = 'daily_water' AND active = 1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(goal)
    }

    /// Get today's progress toward the daily water goal.
    ///
    /// # Returns
    ///
    /// Tuple of (today_total_ml, goal_target_ml, percentage, achieved)
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_today_goal_progress(&self) -> Result<(i64, i64, f64, bool)> {
        let today_total = self.get_today_total_ml().await?;

        let goal = self.get_active_daily_water_goal().await?;

        let (target, percentage, achieved) = if let Some(g) = goal {
            let target = g.target_value as i64;
            let pct = if target > 0 {
                (today_total as f64 / target as f64) * 100.0
            } else {
                0.0
            };
            let achieved = today_total >= target;
            (target, pct, achieved)
        } else {
            (0, 0.0, false)
        };

        Ok((today_total, target, percentage, achieved))
    }

    /// Record daily goal progress (called at end of day or on-demand).
    ///
    /// # Arguments
    ///
    /// * `date` - Date to record progress for (YYYY-MM-DD)
    ///
    /// # Returns
    ///
    /// The ID of the created progress record
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if operation fails
    pub async fn record_daily_goal_progress(&self, date: &str) -> Result<i64> {
        let goal = self.get_active_daily_water_goal().await?;

        if let Some(g) = goal {
            // Get total for the specified date
            let total: i64 = sqlx::query_scalar(
                r#"
                SELECT COALESCE(SUM(amount_ml), 0)
                FROM water_intake
                WHERE DATE(timestamp) = ?
                "#,
            )
            .bind(date)
            .fetch_one(&self.pool)
            .await?;

            let target = g.target_value as i64;
            let percentage = if target > 0 {
                (total as f64 / target as f64) * 100.0
            } else {
                0.0
            };
            let achieved = total >= target;

            let result = sqlx::query(
                r#"
                INSERT INTO goal_progress (goal_id, date, actual_value, target_value, achieved, percentage)
                VALUES (?, ?, ?, ?, ?, ?)
                ON CONFLICT(goal_id, date) DO UPDATE SET
                    actual_value = excluded.actual_value,
                    achieved = excluded.achieved,
                    percentage = excluded.percentage
                "#,
            )
            .bind(g.id)
            .bind(date)
            .bind(total as f64)
            .bind(g.target_value)
            .bind(if achieved { 1 } else { 0 })
            .bind(percentage)
            .execute(&self.pool)
            .await?;

            Ok(result.last_insert_rowid())
        } else {
            Ok(0)
        }
    }

    /// Get goal progress history for the last N days.
    ///
    /// # Arguments
    ///
    /// * `days` - Number of days to retrieve
    ///
    /// # Returns
    ///
    /// Vector of `GoalProgress` records
    ///
    /// # Errors
    ///
    /// Returns `AppError::Database` if query fails
    pub async fn get_goal_progress_history(&self, days: i64) -> Result<Vec<GoalProgress>> {
        let progress = sqlx::query_as::<_, GoalProgress>(
            r#"
            SELECT id, goal_id, date, actual_value, target_value, achieved, percentage, created_at
            FROM goal_progress
            WHERE date >= DATE('now', ? || ' days')
            ORDER BY date DESC
            "#,
        )
        .bind(format!("-{}", days))
        .fetch_all(&self.pool)
        .await?;

        Ok(progress)
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

/// Activity log record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct ActivityLog {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub activity_type: String,
    pub details: Option<String>,
    pub metadata: Option<String>,
}

/// Daily water intake statistics.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DailyWaterStats {
    pub date: String,
    pub intake_count: i64,
    pub total_ml: i64,
    pub avg_ml: i64,
    pub min_ml: i64,
    pub max_ml: i64,
}

/// Hourly drinking pattern.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HourlyPattern {
    pub hour: i64,
    pub intake_count: i64,
    pub total_ml: i64,
    pub avg_ml: i64,
}

/// Weekly drinking pattern.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WeeklyPattern {
    pub day_name: String,
    pub day_num: i64,
    pub intake_count: i64,
    pub total_ml: i64,
    pub avg_ml: i64,
}

/// Reminder effectiveness statistics.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReminderEffectiveness {
    pub reminder_type: String,
    pub total_reminders: i64,
    pub actions_taken: i64,
    pub effectiveness_percentage: f64,
    pub avg_response_seconds: Option<f64>,
}

/// Goal record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Goal {
    pub id: i64,
    pub goal_type: String,
    pub target_value: f64,
    pub unit: String,
    pub created_at: String,
    pub updated_at: String,
    pub active: i64,
    pub notes: Option<String>,
}

/// Goal progress record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GoalProgress {
    pub id: i64,
    pub goal_id: i64,
    pub date: String,
    pub actual_value: f64,
    pub target_value: f64,
    pub achieved: i64,
    pub percentage: f64,
    pub created_at: String,
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

    #[tokio::test]
    async fn test_set_daily_water_goal() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();

        // Set a goal
        let goal_id = db.set_daily_water_goal(2000, Some("Test goal")).await.unwrap();
        assert!(goal_id > 0);

        // Get the active goal
        let goal = db.get_active_daily_water_goal().await.unwrap();
        assert!(goal.is_some());
        let g = goal.unwrap();
        assert_eq!(g.target_value as i64, 2000);
        assert_eq!(g.notes, Some("Test goal".to_string()));
    }

    #[tokio::test]
    async fn test_get_today_goal_progress() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();

        // Set a goal
        db.set_daily_water_goal(2000, None).await.unwrap();

        // Log some water
        db.record_water_intake(500, None).await.unwrap();
        db.record_water_intake(750, None).await.unwrap();

        // Check progress
        let (current, target, percentage, achieved) = db.get_today_goal_progress().await.unwrap();
        assert_eq!(current, 1250);
        assert_eq!(target, 2000);
        assert!((percentage - 62.5).abs() < 0.1);
        assert!(!achieved);

        // Log more water to achieve goal
        db.record_water_intake(800, None).await.unwrap();

        let (current2, _target2, percentage2, achieved2) = db.get_today_goal_progress().await.unwrap();
        assert_eq!(current2, 2050);
        assert!((percentage2 - 102.5).abs() < 0.1);
        assert!(achieved2);
    }

    #[tokio::test]
    async fn test_record_daily_goal_progress() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();

        // Set a goal
        db.set_daily_water_goal(2000, None).await.unwrap();

        // Log water
        db.record_water_intake(1800, None).await.unwrap();

        // Record progress for today
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let progress_id = db.record_daily_goal_progress(&today).await.unwrap();
        assert!(progress_id > 0);

        // Get progress history
        let history = db.get_goal_progress_history(7).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].actual_value as i64, 1800);
        assert_eq!(history[0].target_value as i64, 2000);
        assert_eq!(history[0].achieved, 0);
    }

    #[tokio::test]
    async fn test_goal_progress_default_goal() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();

        // Migration creates a default goal of 2000ml
        let goal = db.get_active_daily_water_goal().await.unwrap();
        assert!(goal.is_some());
        assert_eq!(goal.unwrap().target_value as i64, 2000);

        // Check progress with default goal and no water logged
        let (current, target, percentage, achieved) = db.get_today_goal_progress().await.unwrap();
        assert_eq!(current, 0);
        assert_eq!(target, 2000);
        assert_eq!(percentage, 0.0);
        assert!(!achieved);
    }
}
