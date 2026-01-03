//! AI chat functionality.
//!
//! This module provides interactive chat capabilities using OpenRouter's API,
//! with conversation history persistence and context management.

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use tracing::info;

use crate::cron::CronManager;
use crate::db::Database;
use crate::error::Result;
use crate::openrouter::{Message, OpenRouterClient};

/// Load agent and tool configurations from JSON files.
fn load_agent_configs() -> std::result::Result<String, Box<dyn std::error::Error>> {
    // Try to load configs from standard location
    let agents_path = "configs/agents.json";
    let tools_path = "configs/tools.json";

    // If configs don't exist, return fallback prompt
    if !std::path::Path::new(agents_path).exists() || !std::path::Path::new(tools_path).exists() {
        return Ok(build_fallback_prompt());
    }

    let agents_json = fs::read_to_string(agents_path)?;
    let tools_json = fs::read_to_string(tools_path)?;

    let agents: Value = serde_json::from_str(&agents_json)?;
    let tools: Value = serde_json::from_str(&tools_json)?;

    Ok(build_system_prompt_from_configs(&agents, &tools))
}

/// Build system prompt from loaded JSON configs.
fn build_system_prompt_from_configs(agents: &Value, tools: &Value) -> String {
    let mut prompt = String::from(
        "You are GopenPal, a multi-agent AI system focused on health and productivity.\n\n\
         === AGENT SYSTEM ===\n"
    );

    // Add agent information
    if let Some(agent_list) = agents["agents"].as_array() {
        prompt.push_str("Available Agents:\n");
        for agent in agent_list {
            if let (Some(name), Some(title), Some(desc)) = (
                agent["name"].as_str(),
                agent["title"].as_str(),
                agent["description"].as_str()
            ) {
                prompt.push_str(&format!("\n**{}** ({})\n{}\n", name, title, desc));

                if let Some(skills) = agent["skills"].as_array() {
                    prompt.push_str("Skills: ");
                    let skill_names: Vec<&str> = skills.iter()
                        .filter_map(|s| s.as_str())
                        .collect();
                    prompt.push_str(&skill_names.join(", "));
                    prompt.push('\n');
                }
            }
        }
    }

    prompt.push_str("\n=== AVAILABLE TOOLS ===\n");

    // Add tool information
    if let Some(tool_list) = tools["tools"].as_array() {
        for tool in tool_list {
            if let Some(name) = tool["name"].as_str() {
                prompt.push_str(&format!("\n**{}**\n", name));

                if let Some(desc) = tool["description"].as_str() {
                    prompt.push_str(&format!("{}\n", desc));
                }

                if let Some(usage) = tool["usage_notes"].as_str() {
                    prompt.push_str(&format!("Usage: {}\n", usage));
                }

                if let Some(commands) = tool["commands"].as_array() {
                    prompt.push_str("\nCommands:\n");
                    for cmd in commands {
                        if let (Some(syntax), Some(desc)) = (
                            cmd["syntax"].as_str(),
                            cmd["description"].as_str()
                        ) {
                            prompt.push_str(&format!("  {} - {}\n", syntax, desc));

                            if let Some(examples) = cmd["examples"].as_array() {
                                for example in examples {
                                    if let Some(ex) = example.as_str() {
                                        prompt.push_str(&format!("    Example: {}\n", ex));
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(practices) = tool["best_practices"].as_array() {
                    prompt.push_str("\nBest Practices:\n");
                    for practice in practices {
                        if let Some(p) = practice.as_str() {
                            prompt.push_str(&format!("  - {}\n", p));
                        }
                    }
                }
            }
        }
    }

    prompt.push_str(
        "\n=== RESPONSE GUIDELINES ===\n\
         - Use STATS_TOOL to analyze data first, then provide insights\n\
         - When coordinating complex requests, delegate to specialist agents\n\
         - Always explain what you're doing and confirm actions\n\
         - Be encouraging about good habits, gentle about areas to improve\n\
         - Be concise, friendly, and supportive\n"
    );

    prompt
}

/// Build fallback prompt if JSON configs are not available.
fn build_fallback_prompt() -> String {
    String::from(
        "You are GopenPal, a helpful AI assistant focused on health and productivity. \
         You help users maintain healthy habits like staying hydrated, taking breaks, \
         and managing their work-life balance.\n\n\
         You have access to two primary tools:\n\n\
         1. STATS_TOOL - Access water intake statistics and patterns\n\
         2. CRON_TOOL - Manage automated water reminders\n\n\
         === STATS_TOOL (use FIRST when user asks about their habits, progress, or patterns) ===\n\
         Commands (output these EXACTLY as shown):\n\
         - [STATS:SUMMARY:7] - Get 7-day summary with all statistics\n\
         - [STATS:SUMMARY:30] - Get 30-day summary\n\
         - [STATS:HOURLY] - Get hourly drinking patterns\n\
         - [STATS:WEEKLY] - Get weekly patterns\n\
         - [STATS:EFFECTIVENESS] - Get reminder effectiveness stats\n\n\
         Use STATS_TOOL when users ask:\n\
         - \"How am I doing?\" / \"Show my progress\"\n\
         - \"What are my patterns?\" / \"When do I drink most?\"\n\
         - \"Are the reminders helping?\"\n\
         - Any questions about their water intake history or habits\n\n\
         === CRON_TOOL ===\n\
         Commands (output these EXACTLY as shown):\n\
         - [CRON:INSTALL:*/30 * * * *] - Install cron job with schedule\n\
         - [CRON:REMOVE] - Remove cron job\n\
         - [CRON:STATUS] - Check cron job status\n\n\
         Available schedules:\n\
         - */15 * * * * (every 15 minutes)\n\
         - */30 * * * * (every 30 minutes)\n\
         - 0 * * * * (every hour)\n\
         - 0 */2 * * * (every 2 hours)\n\
         - */30 9-17 * * 1-5 (every 30min, work hours only)\n\n\
         === RESPONSE GUIDELINES ===\n\
         - Use STATS_TOOL to analyze data, then provide insights and recommendations\n\
         - When you see statistics, interpret them and suggest improvements\n\
         - Be encouraging about good habits, gentle about areas to improve\n\
         - Always explain what you're doing and confirm actions\n\
         - Be concise, friendly, and supportive"
    )
}

/// Chat session manager.
///
/// This struct handles interactive chat sessions, managing conversation context,
/// history persistence, and user interaction.
pub struct ChatSession {
    db: Database,
    client: OpenRouterClient,
    model: String,
}

impl ChatSession {
    /// Create a new chat session.
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection for history persistence
    /// * `client` - OpenRouter API client
    /// * `model` - Model identifier to use (e.g., "anthropic/claude-3.5-sonnet")
    pub fn new(db: Database, client: OpenRouterClient, model: String) -> Self {
        Self { db, client, model }
    }

    /// Start an interactive chat session.
    ///
    /// This runs a REPL-style loop where users can send messages and receive
    /// responses. The conversation history is maintained and persisted.
    ///
    /// # Errors
    ///
    /// Returns error if API calls or database operations fail
    pub async fn run_interactive(&self) -> Result<()> {
        println!("\n=== GopenPal AI Chat ===");
        println!("Model: {}", self.model);
        println!("Type 'exit' or 'quit' to end the session");
        println!("Type 'clear' to clear chat history");
        println!("Type 'history' to view recent messages\n");

        // Load recent history for context (last 10 messages)
        let history = self.db.get_recent_chat_history(10).await?;
        let mut messages: Vec<Message> = history
            .iter()
            .map(|msg| Message {
                role: msg.role.clone(),
                content: msg.content.clone(),
            })
            .collect();

        // Add system message for health/work assistant context with tools
        if messages.is_empty() {
            let system_prompt = load_agent_configs()
                .unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to load agent configs: {}. Using fallback.", e);
                    build_fallback_prompt()
                });
            let system_msg = Message::system(&system_prompt);
            messages.insert(0, system_msg);
        }

        loop {
            // Prompt for user input
            print_colored("You: ", Color::Green)?;
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            // Handle special commands
            match input.to_lowercase().as_str() {
                "exit" | "quit" => {
                    println!("Goodbye!");
                    break;
                }
                "clear" => {
                    self.db.clear_chat_history().await?;
                    messages.clear();
                    println!("Chat history cleared.");
                    continue;
                }
                "history" => {
                    self.show_history().await?;
                    continue;
                }
                "" => continue,
                _ => {}
            }

            // Add user message
            let user_message = Message::user(input);
            messages.push(user_message.clone());

            // Save user message to database
            self.db
                .save_chat_message(&user_message.role, &user_message.content, None, None)
                .await?;

            // Show loading indicator
            print_colored("Assistant: ", Color::Cyan)?;
            io::stdout().flush()?;

            // Get response from AI
            let response = self
                .client
                .chat_completion(&self.model, messages.clone(), Some(1000))
                .await?;

            let assistant_message = &response.choices[0].message;

            // Process and execute stats commands first, then cron commands
            let (content_after_stats, stats_result) =
                process_stats_commands(&assistant_message.content, &self.db).await;
            let (display_content, cron_result) = process_cron_commands(&content_after_stats);

            println!("{}", display_content);

            if let Some(ref result) = stats_result {
                print_colored("\n[STATS] ", Color::Magenta)?;
                println!("{}\n", result);
            }

            if let Some(ref result) = cron_result {
                print_colored("\n[CRON] ", Color::Yellow)?;
                println!("{}\n", result);
            }

            if stats_result.is_none() && cron_result.is_none() {
                println!();
            }

            // Add assistant response to conversation
            messages.push(assistant_message.clone());

            // Save assistant message to database
            let tokens_used = response.usage.as_ref().map(|u| u.total_tokens as i64);
            self.db
                .save_chat_message(
                    &assistant_message.role,
                    &assistant_message.content,
                    Some(&self.model),
                    tokens_used,
                )
                .await?;

            info!("Chat: tokens_used={:?}, model={}", tokens_used, self.model);
        }

        Ok(())
    }

    /// Send a single message and get a response (non-interactive mode).
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send
    ///
    /// # Returns
    ///
    /// The assistant's response
    ///
    /// # Errors
    ///
    /// Returns error if API call or database operations fail
    pub async fn send_message(&self, message: &str) -> Result<String> {
        let system_prompt = load_agent_configs()
            .unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load agent configs: {}. Using fallback.", e);
                build_fallback_prompt()
            });

        let mut messages = vec![
            Message::system(&system_prompt),
            Message::user(message),
        ];

        // Save user message
        self.db
            .save_chat_message("user", message, None, None)
            .await?;

        let response = self
            .client
            .chat_completion(&self.model, messages.clone(), Some(1000))
            .await?;

        let assistant_message = &response.choices[0].message;
        messages.push(assistant_message.clone());

        // Process stats commands first, then cron commands
        let (content_after_stats, stats_result) =
            process_stats_commands(&assistant_message.content, &self.db).await;
        let (cleaned_content, cron_result) = process_cron_commands(&content_after_stats);

        // Build full response with both stats and cron results
        let mut full_response = cleaned_content;
        if let Some(result) = stats_result {
            full_response.push_str(&format!("\n\n[STATS] {}", result));
        }
        if let Some(result) = cron_result {
            full_response.push_str(&format!("\n\n[CRON] {}", result));
        }

        // Save assistant message
        let tokens_used = response.usage.as_ref().map(|u| u.total_tokens as i64);
        self.db
            .save_chat_message(
                &assistant_message.role,
                &full_response,
                Some(&self.model),
                tokens_used,
            )
            .await?;

        Ok(full_response)
    }

    /// Display recent chat history.
    async fn show_history(&self) -> Result<()> {
        let history = self.db.get_recent_chat_history(20).await?;

        if history.is_empty() {
            println!("No chat history found.");
            return Ok(());
        }

        println!("\n=== Recent Chat History ===\n");
        for msg in &history {
            let color = match msg.role.as_str() {
                "user" => Color::Green,
                "assistant" => Color::Cyan,
                _ => Color::White,
            };

            let role_display = match msg.role.as_str() {
                "user" => "You",
                "assistant" => "Assistant",
                _ => &msg.role,
            };

            print_colored(&format!("{}: ", role_display), color)?;
            println!("{}", msg.content);

            if let Some(tokens) = msg.tokens_used {
                println!("  (tokens: {})", tokens);
            }
            println!();
        }

        Ok(())
    }
}

/// Helper function to print colored text to terminal.
///
/// # Arguments
///
/// * `text` - Text to print
/// * `color` - Color to use
///
/// # Errors
///
/// Returns IO error if writing to stdout fails
fn print_colored(text: &str, color: Color) -> io::Result<()> {
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor
    )?;
    Ok(())
}

/// Process stats commands in AI response.
///
/// # Arguments
///
/// * `content` - The AI's response content
/// * `db` - Database connection for querying statistics
///
/// # Returns
///
/// Tuple of (cleaned_content, optional_stats_result)
async fn process_stats_commands(content: &str, db: &Database) -> (String, Option<String>) {
    let mut result_message = None;
    let mut cleaned = content.to_string();

    // Check for STATS:SUMMARY command with days parameter
    if let Some(start) = content.find("[STATS:SUMMARY:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let days_str = command
                .trim_start_matches("[STATS:SUMMARY:")
                .trim_end_matches(']');

            if let Ok(days) = days_str.parse::<i64>() {
                let result = match db.get_statistics_summary(days).await {
                    Ok(summary) => summary,
                    Err(e) => format!("Error fetching statistics: {}", e),
                };
                result_message = Some(result);
                cleaned = cleaned.replace(command, "");
            }
        }
    }

    // Check for STATS:HOURLY command
    if content.contains("[STATS:HOURLY]") {
        let result = match db.get_hourly_patterns().await {
            Ok(patterns) => {
                let mut output = "=== Hourly Drinking Patterns ===\n\n".to_string();
                for pattern in &patterns {
                    output.push_str(&format!(
                        "{}:00 - {} intakes, {}ml total (avg: {}ml)\n",
                        pattern.hour, pattern.intake_count, pattern.total_ml, pattern.avg_ml
                    ));
                }
                output
            }
            Err(e) => format!("Error fetching hourly patterns: {}", e),
        };
        result_message = Some(result);
        cleaned = cleaned.replace("[STATS:HOURLY]", "");
    }

    // Check for STATS:WEEKLY command
    if content.contains("[STATS:WEEKLY]") {
        let result = match db.get_weekly_patterns().await {
            Ok(patterns) => {
                let mut output = "=== Weekly Drinking Patterns ===\n\n".to_string();
                for pattern in &patterns {
                    output.push_str(&format!(
                        "{} - {} intakes, {}ml total (avg: {}ml)\n",
                        pattern.day_name, pattern.intake_count, pattern.total_ml, pattern.avg_ml
                    ));
                }
                output
            }
            Err(e) => format!("Error fetching weekly patterns: {}", e),
        };
        result_message = Some(result);
        cleaned = cleaned.replace("[STATS:WEEKLY]", "");
    }

    // Check for STATS:EFFECTIVENESS command
    if content.contains("[STATS:EFFECTIVENESS]") {
        let result = match db.get_reminder_effectiveness().await {
            Ok(stats) => {
                let mut output = "=== Reminder Effectiveness ===\n\n".to_string();
                if stats.is_empty() {
                    output.push_str("No reminder data available yet.\n");
                } else {
                    for eff in &stats {
                        output.push_str(&format!(
                            "{} - {:.1}% effective ({}/{} acted upon)\n",
                            eff.reminder_type,
                            eff.effectiveness_percentage,
                            eff.actions_taken,
                            eff.total_reminders
                        ));
                        if let Some(avg_resp) = eff.avg_response_seconds {
                            output.push_str(&format!("  Avg response time: {}s\n", avg_resp));
                        }
                    }
                }
                output
            }
            Err(e) => format!("Error fetching effectiveness stats: {}", e),
        };
        result_message = Some(result);
        cleaned = cleaned.replace("[STATS:EFFECTIVENESS]", "");
    }

    // Clean up any extra whitespace
    cleaned = cleaned.trim().to_string();

    (cleaned, result_message)
}

/// Process cron commands in AI response.
///
/// # Arguments
///
/// * `content` - The AI's response content
///
/// # Returns
///
/// Tuple of (cleaned_content, optional_cron_result)
fn process_cron_commands(content: &str) -> (String, Option<String>) {
    let cron = CronManager::new(None);
    let mut result_message = None;
    let mut cleaned = content.to_string();

    // Check for CRON:STATUS command
    if content.contains("[CRON:STATUS]") {
        let status = match cron.is_installed() {
            Ok(Some(line)) => format!("Cron job is installed: {}", line),
            Ok(None) => "No cron job installed".to_string(),
            Err(e) => format!("Error checking status: {}", e),
        };
        result_message = Some(status);
        cleaned = cleaned.replace("[CRON:STATUS]", "");
    }

    // Check for CRON:REMOVE command
    if content.contains("[CRON:REMOVE]") {
        let result = match cron.remove() {
            Ok(_) => "Successfully removed cron job".to_string(),
            Err(e) => format!("Error removing cron job: {}", e),
        };
        result_message = Some(result);
        cleaned = cleaned.replace("[CRON:REMOVE]", "");
    }

    // Check for CRON:INSTALL command with schedule
    if let Some(start) = content.find("[CRON:INSTALL:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let schedule = command
                .trim_start_matches("[CRON:INSTALL:")
                .trim_end_matches(']');

            let result = match cron.is_installed() {
                Ok(Some(_)) => {
                    // Update existing
                    match cron.update_schedule(schedule) {
                        Ok(_) => format!("Updated cron job to run: {}", schedule),
                        Err(e) => format!("Error updating cron job: {}", e),
                    }
                }
                Ok(None) => {
                    // Install new
                    match cron.install(schedule) {
                        Ok(_) => format!("Installed cron job to run: {}", schedule),
                        Err(e) => format!("Error installing cron job: {}", e),
                    }
                }
                Err(e) => format!("Error: {}", e),
            };

            result_message = Some(result);
            cleaned = cleaned.replace(command, "");
        }
    }

    // Clean up any extra whitespace
    cleaned = cleaned.trim().to_string();

    (cleaned, result_message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_session_creation() {
        let db = Database::new("sqlite::memory:");
        let client = OpenRouterClient::new("test-key".to_string());

        // This is async, so we just test the synchronous creation
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let db = db.await.unwrap();
            let session = ChatSession::new(db, client, "test-model".to_string());
            assert_eq!(session.model, "test-model");
        });
    }
}
