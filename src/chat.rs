//! AI chat functionality.
//!
//! This module provides interactive chat capabilities using OpenRouter's API,
//! with conversation history persistence and context management.

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{self, Write};
use tracing::info;

use crate::cron::CronManager;
use crate::db::Database;
use crate::error::Result;
use crate::openrouter::{Message, OpenRouterClient};

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
            let system_msg = Message::system(
                "You are GopenPal, a helpful AI assistant focused on health and productivity. \
                 You help users maintain healthy habits like staying hydrated, taking breaks, \
                 and managing their work-life balance.\n\n\
                 You have access to the CRON_TOOL to manage automated water reminders.\n\n\
                 CRON_TOOL commands (output these EXACTLY as shown when user requests cron changes):\n\
                 - [CRON:INSTALL:*/30 * * * *] - Install cron job with schedule\n\
                 - [CRON:REMOVE] - Remove cron job\n\
                 - [CRON:STATUS] - Check cron job status\n\n\
                 Available schedules:\n\
                 - */15 * * * * (every 15 minutes)\n\
                 - */30 * * * * (every 30 minutes)\n\
                 - 0 * * * * (every hour)\n\
                 - 0 */2 * * * (every 2 hours)\n\
                 - */30 9-17 * * 1-5 (every 30min, work hours only)\n\n\
                 When user asks to set up/change/remove reminders, use the CRON_TOOL.\n\
                 Always explain what you're doing and confirm the action.\n\
                 Be concise, friendly, and supportive in all other responses.",
            );
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

            // Process and execute cron commands
            let (display_content, cron_result) = process_cron_commands(&assistant_message.content);
            println!("{}", display_content);

            if let Some(result) = cron_result {
                print_colored("\n[CRON] ", Color::Yellow)?;
                println!("{}\n", result);
            } else {
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
        let mut messages = vec![
            Message::system(
                "You are GopenPal, a helpful AI assistant focused on health and productivity. \
                 You help users maintain healthy habits like staying hydrated, taking breaks, \
                 and managing their work-life balance.\n\n\
                 You have access to the CRON_TOOL to manage automated water reminders.\n\n\
                 CRON_TOOL commands (output these EXACTLY as shown when user requests cron changes):\n\
                 - [CRON:INSTALL:*/30 * * * *] - Install cron job with schedule\n\
                 - [CRON:REMOVE] - Remove cron job\n\
                 - [CRON:STATUS] - Check cron job status\n\n\
                 Available schedules:\n\
                 - */15 * * * * (every 15 minutes)\n\
                 - */30 * * * * (every 30 minutes)\n\
                 - 0 * * * * (every hour)\n\
                 - 0 */2 * * * (every 2 hours)\n\
                 - */30 9-17 * * 1-5 (every 30min, work hours only)\n\n\
                 When user asks to set up/change/remove reminders, use the CRON_TOOL.\n\
                 Always explain what you're doing and confirm the action.\n\
                 Be concise, friendly, and supportive in all other responses.",
            ),
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

        // Process cron commands and get cleaned response
        let (cleaned_content, cron_result) = process_cron_commands(&assistant_message.content);

        // Append cron result to response if any
        let full_response = if let Some(result) = cron_result {
            format!("{}\n\n[CRON] {}", cleaned_content, result)
        } else {
            cleaned_content
        };

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
