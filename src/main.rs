//! GopenPal - Personal LLM health and work assistant.
//!
//! A CLI application for managing water intake, work habits, and AI-powered assistance.

mod agent_daemon;
mod agents;
mod chat;
mod cli;
mod cron;
mod db;
mod error;
mod openrouter;
mod reminder;
mod tui;

use anyhow::Context;
use chrono::NaiveTime;
use clap::Parser;
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use chat::ChatSession;
use cli::{ChatCommands, Cli, Commands, ReminderCommands, WaterCommands};
use db::Database;
use openrouter::OpenRouterClient;
use reminder::ReminderService;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Application error: {:#}", e);
        std::process::exit(1);
    }
}

/// Main application entry point.
///
/// This function initializes the application, parses CLI arguments,
/// and dispatches to the appropriate command handler.
async fn run() -> anyhow::Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level)),
        )
        .init();

    info!("Starting GopenPal");

    // Expand tilde in database path
    // This is a Python-like convenience for home directory paths
    let db_path = expand_tilde(&cli.database);
    let db_url = format!("sqlite://{}", db_path.display());

    // Connect to database
    let db = Database::new(&db_url)
        .await
        .context("Failed to connect to database")?;

    // Handle commands
    match cli.command {
        Commands::Tui => {
            db.migrate().await?;
            let api_key = std::env::var("OPENROUTER_API_KEY").ok();
            tui::run_tui(db, api_key).await?;
        }

        Commands::Init => {
            info!("Initializing database");
            db.migrate().await.context("Failed to run migrations")?;
            println!("Database initialized successfully at {}", db_path.display());
            println!("\nNext steps:");
            println!("1. Set your OpenRouter API key: export OPENROUTER_API_KEY=your-key");
            println!("2. Launch TUI: gopenpal tui");
            println!("3. Or use CLI: gopenpal water log 250");
            println!("4. Set up cron job: */30 * * * * gopenpal reminder check");
        }

        Commands::Water { action } => {
            db.migrate().await?;
            handle_water_command(&db, action).await?;
        }

        Commands::Reminder { action } => {
            db.migrate().await?;
            handle_reminder_command(&db, action).await?;
        }

        Commands::Chat { action } => {
            db.migrate().await?;
            handle_chat_command(&db, action).await?;
        }
    }

    Ok(())
}

/// Handle water intake commands.
async fn handle_water_command(db: &Database, action: WaterCommands) -> anyhow::Result<()> {
    match action {
        WaterCommands::Log { amount, notes } => {
            db.record_water_intake(amount, notes.as_deref()).await?;
            println!("Logged {} ml of water", amount);

            let total = db.get_today_total_ml().await?;
            println!(
                "Today's total: {} ml ({:.1} L)",
                total,
                total as f64 / 1000.0
            );
        }

        WaterCommands::Today => {
            let total = db.get_today_total_ml().await?;
            let records = db.get_today_water_intake().await?;

            println!(
                "Today's water intake: {} ml ({:.1} L)",
                total,
                total as f64 / 1000.0
            );
            println!("\nEntries:");
            for record in records {
                let time = record.timestamp.format("%H:%M");
                let notes = record.notes.unwrap_or_default();
                println!("  {} - {} ml {}", time, record.amount_ml, notes);
            }
        }

        WaterCommands::History { days: _ } => {
            // Future implementation: show historical data
            println!("History feature coming soon!");
        }
    }

    Ok(())
}

/// Handle reminder commands.
async fn handle_reminder_command(db: &Database, action: ReminderCommands) -> anyhow::Result<()> {
    match action {
        ReminderCommands::Start => {
            println!("Starting water reminder service...");
            println!("Press Ctrl+C to stop");

            let service = ReminderService::new(db.clone());
            service.run().await?;
        }

        ReminderCommands::Check => {
            // Cron-compatible command: check once and send reminder if needed
            use chrono::Local;
            use notify_rust::Notification;

            let settings = db.get_reminder_settings().await?;

            if !settings.enabled {
                return Ok(());
            }

            let now = Local::now();
            let current_time = now.time();

            // Check if we're within work hours
            if current_time < settings.work_hours_start || current_time >= settings.work_hours_end {
                return Ok(());
            }

            // Check if it's a work day
            let weekday = now.format("%a").to_string();
            if !settings.work_days.contains(&weekday) {
                return Ok(());
            }

            // Send reminder
            let total_ml = db.get_today_total_ml().await?;
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
                .show()?;

            info!("Sent water reminder at {}", now.format("%H:%M"));
        }

        ReminderCommands::Status => {
            let settings = db.get_reminder_settings().await?;
            println!("Reminder Settings:");
            println!("  Enabled: {}", settings.enabled);
            println!("  Interval: {} minutes", settings.interval_minutes);
            println!(
                "  Work hours: {} - {}",
                settings.work_hours_start, settings.work_hours_end
            );
            println!("  Work days: {}", settings.work_days);
        }

        ReminderCommands::Config {
            enabled,
            interval,
            start,
            end,
            days,
        } => {
            let mut settings = db.get_reminder_settings().await?;

            if let Some(e) = enabled {
                settings.enabled = e;
            }
            if let Some(i) = interval {
                settings.interval_minutes = i;
            }
            if let Some(s) = start {
                settings.work_hours_start = NaiveTime::parse_from_str(&s, "%H:%M")
                    .context("Invalid time format, use HH:MM")?;
            }
            if let Some(e) = end {
                settings.work_hours_end = NaiveTime::parse_from_str(&e, "%H:%M")
                    .context("Invalid time format, use HH:MM")?;
            }
            if let Some(d) = days {
                settings.work_days = d;
            }

            db.update_reminder_settings(&settings).await?;
            println!("Reminder settings updated");
        }
    }

    Ok(())
}

/// Handle chat commands.
async fn handle_chat_command(db: &Database, action: Option<ChatCommands>) -> anyhow::Result<()> {
    // Get API key from environment
    let api_key = std::env::var("OPENROUTER_API_KEY")
        .context("OPENROUTER_API_KEY environment variable not set")?;

    let client = OpenRouterClient::new(api_key);

    match action {
        None | Some(ChatCommands::Interactive { .. }) => {
            let model = if let Some(ChatCommands::Interactive { model }) = action {
                model
            } else {
                std::env::var("GOPENPAL_MODEL")
                    .unwrap_or_else(|_| "anthropic/claude-3.5-sonnet".to_string())
            };

            let session = ChatSession::new(db.clone(), client, model);
            session.run_interactive().await?;
        }

        Some(ChatCommands::Send { message, model }) => {
            let session = ChatSession::new(db.clone(), client, model);
            let response = session.send_message(&message).await?;
            println!("{}", response);
        }

        Some(ChatCommands::History { limit }) => {
            let history = db.get_recent_chat_history(limit).await?;

            if history.is_empty() {
                println!("No chat history found");
                return Ok(());
            }

            println!("Recent chat history:\n");
            for msg in history {
                println!(
                    "[{}] {}: {}",
                    msg.timestamp.format("%Y-%m-%d %H:%M"),
                    msg.role,
                    msg.content
                );
                if let Some(tokens) = msg.tokens_used {
                    println!("  (tokens: {})", tokens);
                }
                println!();
            }
        }

        Some(ChatCommands::Clear) => {
            db.clear_chat_history().await?;
            println!("Chat history cleared");
        }
    }

    Ok(())
}

/// Expand tilde in path to home directory.
///
/// # Arguments
///
/// * `path` - Path potentially containing tilde
///
/// # Returns
///
/// Expanded path with tilde replaced by home directory
fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return PathBuf::from(path.replacen('~', &home.display().to_string(), 1));
        }
    }
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let path = "~/test/path";
        let expanded = expand_tilde(path);
        assert!(!expanded.to_string_lossy().contains('~'));
    }

    #[test]
    fn test_expand_tilde_no_tilde() {
        let path = "/absolute/path";
        let expanded = expand_tilde(path);
        assert_eq!(expanded.to_string_lossy(), path);
    }
}
