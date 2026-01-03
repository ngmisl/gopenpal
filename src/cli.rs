//! Command-line interface definitions.
//!
//! This module defines the CLI structure using clap, including all commands
//! and their arguments.

use clap::{Parser, Subcommand};

/// Personal LLM health and work assistant.
#[derive(Parser, Debug)]
#[command(name = "gopenpal")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Database file path
    #[arg(long, env = "GOPENPAL_DB", default_value = "~/.gopenpal/gopenpal.db")]
    pub database: String,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Launch interactive Terminal User Interface (TUI)
    Tui,

    /// Water intake tracking commands
    Water {
        #[command(subcommand)]
        action: WaterCommands,
    },

    /// Start water reminder service
    Reminder {
        #[command(subcommand)]
        action: ReminderCommands,
    },

    /// Chat with AI assistant
    Chat {
        #[command(subcommand)]
        action: Option<ChatCommands>,
    },

    /// Interact with AI agents (Hydrix, Serhant)
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },

    /// Initialize database and configuration
    Init,
}

#[derive(Subcommand, Debug)]
pub enum WaterCommands {
    /// Log water intake
    Log {
        /// Amount in milliliters
        #[arg(default_value = "250")]
        amount: i64,

        /// Optional notes
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// Show today's water intake
    Today,

    /// Show water intake history
    History {
        /// Number of days to show
        #[arg(short, long, default_value = "7")]
        days: i64,
    },
}

#[derive(Subcommand, Debug)]
pub enum ReminderCommands {
    /// Start reminder service (runs in foreground)
    Start,

    /// Check if reminder should be sent (for cron jobs)
    Check,

    /// Show current reminder settings
    Status,

    /// Update reminder settings
    Config {
        /// Enable/disable reminders
        #[arg(long)]
        enabled: Option<bool>,

        /// Reminder interval in minutes
        #[arg(long)]
        interval: Option<i64>,

        /// Work hours start time (HH:MM)
        #[arg(long)]
        start: Option<String>,

        /// Work hours end time (HH:MM)
        #[arg(long)]
        end: Option<String>,

        /// Work days (comma-separated, e.g., Mon,Tue,Wed)
        #[arg(long)]
        days: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ChatCommands {
    /// Start interactive chat session
    Interactive {
        /// Model to use
        #[arg(
            short,
            long,
            env = "GOPENPAL_MODEL",
            default_value = "anthropic/claude-3.5-sonnet"
        )]
        model: String,
    },

    /// Send a single message
    Send {
        /// Message to send
        message: String,

        /// Model to use
        #[arg(
            short,
            long,
            env = "GOPENPAL_MODEL",
            default_value = "anthropic/claude-3.5-sonnet"
        )]
        model: String,
    },

    /// View chat history
    History {
        /// Number of messages to show
        #[arg(short, long, default_value = "20")]
        limit: i64,
    },

    /// Clear chat history
    Clear,
}

#[derive(Subcommand, Debug)]
pub enum AgentCommands {
    /// Show agent information and status
    Info {
        /// Agent name (Hydrix or Serhant)
        agent: String,
    },

    /// List all agents
    List,

    /// Trigger a message from an agent
    Message {
        /// Agent name (Hydrix or Serhant)
        agent: String,
    },

    /// Start agent daemon (runs in background)
    Daemon {
        /// Agent to run (default: both)
        #[arg(short, long)]
        agent: Option<String>,

        /// Interval in minutes between checks
        #[arg(short, long, default_value = "45")]
        interval: u64,
    },

    /// View agent relationship status
    Relationship {
        /// Agent name (Hydrix or Serhant)
        agent: String,
    },

    /// View unlocked lore
    Lore {
        /// Filter by category (agent_history, world_building, secrets)
        #[arg(short, long)]
        category: Option<String>,
    },

    /// View achievements
    Achievements {
        /// Show only unlocked achievements
        #[arg(short, long)]
        unlocked: bool,
    },
}
