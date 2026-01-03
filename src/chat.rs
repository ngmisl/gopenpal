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

        // Add system message for health/work assistant context
        if messages.is_empty() {
            let system_msg = Message::system(
                "You are GopenPal, a helpful AI assistant focused on health and productivity. \
                 You help users maintain healthy habits like staying hydrated, taking breaks, \
                 and managing their work-life balance. Be concise, friendly, and supportive.",
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
            println!("{}\n", assistant_message.content);

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
                "You are GopenPal, a helpful AI assistant focused on health and productivity.",
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

        // Save assistant message
        let tokens_used = response.usage.as_ref().map(|u| u.total_tokens as i64);
        self.db
            .save_chat_message(
                &assistant_message.role,
                &assistant_message.content,
                Some(&self.model),
                tokens_used,
            )
            .await?;

        Ok(assistant_message.content.clone())
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
