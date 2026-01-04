//! AI chat functionality.
//!
//! This module provides interactive chat capabilities using OpenRouter's API,
//! with conversation history persistence and context management.

use chrono::{Local, Datelike};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use tracing::info;

use crate::agent_router::AgentRouter;
use crate::agents::AgentSystem;
use crate::cron::CronManager;
use crate::db::Database;
use crate::error::Result;
use crate::openrouter::{Message, OpenRouterClient};
use crate::personality::PersonalityProfile;
use crate::security::SecurityConfig;
use std::process::{Command, Stdio};

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
    // Get current date and time
    let now = Local::now();
    let date_str = now.format("%A, %B %d, %Y").to_string();
    let time_str = now.format("%I:%M %p").to_string();
    let day_of_week = now.weekday().to_string();

    let mut prompt = format!(
        "You are GopenPal, a multi-agent AI system focused on health and productivity.\n\n\
         === CURRENT CONTEXT ===\n\
         Date: {} ({})\n\
         Time: {}\n\
         Note: Always be aware of the current date and time when providing advice, \
         setting reminders, or discussing tasks.\n\n\
         === AGENT SYSTEM ===\n",
        date_str, day_of_week, time_str
    );

    // Add agent information with personality details
    if let Some(agent_list) = agents["agents"].as_array() {
        prompt.push_str("Available Agents:\n");
        for agent in agent_list {
            if let (Some(name), Some(title), Some(desc), Some(personality)) = (
                agent["name"].as_str(),
                agent["title"].as_str(),
                agent["description"].as_str(),
                agent["personality_type"].as_str()
            ) {
                prompt.push_str(&format!("\n**{}** ({})\n{}\n", name, title, desc));

                // Add personality information
                if let Some(profile) = PersonalityProfile::get_profile(personality) {
                    prompt.push_str(&format!("Personality: {} ({})\n",
                        profile.personality_type,
                        profile.voice_characteristics.tone
                    ));
                }

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
         - Be concise, friendly, and supportive\n\n\
         === MULTI-AGENT COORDINATION ===\n\
         For complex requests spanning multiple domains, use:\n\
         [MULTI_AGENT:Agent1,Agent2,Agent3:request]\n\n\
         Examples:\n\
         - Full status check: [MULTI_AGENT:Hydrix,Serhant,Karen:How is the user doing today?]\n\
         - Health & productivity: [MULTI_AGENT:Hydrix,Serhant:Any advice for better performance?]\n\
         - Task & work balance: [MULTI_AGENT:Karen,Serhant:Help prioritize today's work]\n\n\
         Each agent will provide their expert perspective with their unique personality.\n\n\
         === PERSONALITY CONSISTENCY ===\n\
         CRITICAL: Each agent must ALWAYS maintain their unique personality:\n\
         - Mio: Warm coordinator who bridges agents, uses 'we' and 'together'\n\
         - Hydrix: Caring ancient spirit, enthusiastic about water, uses water metaphors 💧\n\
         - Serhant: High energy motivator, Big Money Energy, direct and punchy 💪🔥\n\
         - Karen: Efficient executive assistant, organized and professional ✓\n\n\
         When delegating, ensure the specialist agent maintains THEIR personality, not yours.\n"
    );

    prompt
}

/// Build fallback prompt if JSON configs are not available.
fn build_fallback_prompt() -> String {
    // Get current date and time
    let now = Local::now();
    let date_str = now.format("%A, %B %d, %Y").to_string();
    let time_str = now.format("%I:%M %p").to_string();

    format!(
        "You are GopenPal, a helpful AI assistant focused on health and productivity. \
         You help users maintain healthy habits like staying hydrated, taking breaks, \
         and managing their work-life balance.\n\n\
         Current Date: {}\n\
         Current Time: {}\n\n\
         You have access to two primary tools:\n\n\
         1. STATS_TOOL - Access water intake statistics and patterns\n\
         2. CRON_TOOL - Manage automated water reminders\n\n\
         === STATS_TOOL (use FIRST when user asks about their habits, progress, or patterns) ===\n\
         Commands (output these EXACTLY as shown):\n\
         - [STATS:SUMMARY:7] - Get 7-day summary with all statistics\n\
         - [STATS:SUMMARY:30] - Get 30-day summary\n\
         - [STATS:HOURLY] - Get hourly drinking patterns\n\
         - [STATS:WEEKLY] - Get weekly patterns\n\
         - [STATS:MONTHLY:6] - Get monthly statistics (last N months)\n\
         - [STATS:COMPARE:7|-14|-7] - Compare periods (this week vs last week)\n\
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
         - Be concise, friendly, and supportive",
        date_str, time_str
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
    router: AgentRouter,
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
        Self {
            db,
            client,
            model,
            router: AgentRouter::new(),
        }
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
        println!("Type 'history' to view recent messages");
        println!("\nAgent Routing:");
        println!("  @Hydrix - Talk to the Hydration Guardian (water/health)");
        println!("  @Serhant - Get Big Money Energy coaching (work/sales/motivation)");
        println!("  @Karen - Executive assistant (tasks/reminders/organization)");
        println!("  @Mio - Coordination and general support (default)\n");

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

            // Route the message to the appropriate agent
            let routing = self.router.route(input);

            // Use the cleaned message if an agent was explicitly mentioned
            let message_to_process = if routing.explicit {
                &routing.cleaned_message
            } else {
                input
            };

            // If high confidence routing to a specialist or explicit mention, delegate directly
            if routing.explicit || (routing.confidence > 0.7 && routing.agent != "Mio") {
                print_colored(&format!("[Routing to {}] ", routing.agent), Color::Yellow)?;
                println!();

                // Save user message to database
                self.db
                    .save_chat_message("user", input, None, None)
                    .await?;

                // Delegate to the specialist agent
                print_colored("Assistant: ", Color::Cyan)?;
                io::stdout().flush()?;

                let agent_response = delegate_to_agent(
                    &routing.agent,
                    message_to_process,
                    &self.db,
                    &self.client,
                    &self.model
                ).await;

                println!("{}\n", agent_response);

                // Save agent response to database
                self.db
                    .save_chat_message(&routing.agent, &agent_response, Some(&self.model), None)
                    .await?;

                continue;
            }

            // Otherwise, proceed with normal chat (Mio coordination)
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

            // Process and execute commands: stats → cron → delegation → multi-agent → grep
            let (content_after_stats, stats_result) =
                process_stats_commands(&assistant_message.content, &self.db).await;
            let (content_after_cron, cron_result) = process_cron_commands(&content_after_stats);
            let (content_after_delegate, delegate_result) =
                process_delegate_commands(&content_after_cron, &self.db, &self.client, &self.model).await;
            let (content_after_multi, multi_agent_result) =
                process_multi_agent_commands(&content_after_delegate, &self.db, &self.client, &self.model).await;
            let (display_content, grep_result) = process_grep_commands(&content_after_multi);

            println!("{}", display_content);

            if let Some(ref result) = stats_result {
                print_colored("\n[STATS] ", Color::Magenta)?;
                println!("{}\n", result);
            }

            if let Some(ref result) = cron_result {
                print_colored("\n[CRON] ", Color::Yellow)?;
                println!("{}\n", result);
            }

            if let Some(ref result) = delegate_result {
                print_colored("\n[DELEGATE] ", Color::Cyan)?;
                println!("{}\n", result);
            }

            if let Some(ref result) = multi_agent_result {
                print_colored("\n[MULTI-AGENT COLLABORATION] ", Color::Blue)?;
                println!("{}\n", result);
            }

            if let Some(ref result) = grep_result {
                print_colored("\n[GREP] ", Color::Green)?;
                println!("{}\n", result);
            }

            if stats_result.is_none() && cron_result.is_none() && delegate_result.is_none()
                && multi_agent_result.is_none() && grep_result.is_none() {
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

        // Process stats commands first, then cron, then delegation, then grep
        let (content_after_stats, stats_result) =
            process_stats_commands(&assistant_message.content, &self.db).await;
        let (content_after_cron, cron_result) = process_cron_commands(&content_after_stats);
        let (content_after_delegate, delegate_result) =
            process_delegate_commands(&content_after_cron, &self.db, &self.client, &self.model).await;
        let (cleaned_content, grep_result) = process_grep_commands(&content_after_delegate);

        // Build full response with stats, cron, delegation, and grep results
        let mut full_response = cleaned_content;
        if let Some(result) = stats_result {
            full_response.push_str(&format!("\n\n[STATS] {}", result));
        }
        if let Some(result) = cron_result {
            full_response.push_str(&format!("\n\n[CRON] {}", result));
        }
        if let Some(result) = delegate_result {
            full_response.push_str(&format!("\n\n[DELEGATE] {}", result));
        }
        if let Some(result) = grep_result {
            full_response.push_str(&format!("\n\n[GREP] {}", result));
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

    // Check for STATS:MONTHLY command
    if let Some(start) = content.find("[STATS:MONTHLY:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let months_str = command
                .trim_start_matches("[STATS:MONTHLY:")
                .trim_end_matches(']');

            if let Ok(months) = months_str.parse::<i64>() {
                let result = match db.get_monthly_statistics(months).await {
                    Ok(stats) => stats,
                    Err(e) => format!("Error fetching monthly statistics: {}", e),
                };
                result_message = Some(result);
                cleaned = cleaned.replace(command, "");
            }
        }
    }

    // Check for STATS:COMPARE command (format: [STATS:COMPARE:7|-14|-7] compares this week to last week)
    if let Some(start) = content.find("[STATS:COMPARE:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let params = command
                .trim_start_matches("[STATS:COMPARE:")
                .trim_end_matches(']');

            let parts: Vec<&str> = params.split('|').collect();
            if parts.len() == 3 {
                if let (Ok(p1), Ok(p2_start), Ok(p2_end)) = (
                    parts[0].parse::<i64>(),
                    parts[1].parse::<i64>(),
                    parts[2].parse::<i64>(),
                ) {
                    let result = match db.compare_periods(p1, p2_start, p2_end).await {
                        Ok(comparison) => comparison,
                        Err(e) => format!("Error comparing periods: {}", e),
                    };
                    result_message = Some(result);
                    cleaned = cleaned.replace(command, "");
                }
            }
        }
    }

    // Check for GOAL:STATUS command
    if content.contains("[GOAL:STATUS]") {
        let result = match db.get_today_goal_progress().await {
            Ok((current, target, percentage, achieved)) => {
                if target > 0 {
                    let mut output = "=== Daily Water Goal Progress ===\n\n".to_string();
                    output.push_str(&format!("Target: {} ml ({:.1} L)\n", target, target as f64 / 1000.0));
                    output.push_str(&format!("Current: {} ml ({:.1} L)\n", current, current as f64 / 1000.0));
                    output.push_str(&format!("Progress: {:.1}%\n", percentage));
                    if achieved {
                        output.push_str("Status: Goal achieved!\n");
                    } else {
                        let remaining = target - current;
                        output.push_str(&format!("Remaining: {} ml ({:.1} L)\n", remaining, remaining as f64 / 1000.0));
                    }
                    output
                } else {
                    "No active daily water goal set.".to_string()
                }
            }
            Err(e) => format!("Error fetching goal progress: {}", e),
        };
        result_message = Some(result);
        cleaned = cleaned.replace("[GOAL:STATUS]", "");
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

/// Process delegation commands in AI response.
///
/// # Arguments
///
/// * `content` - The AI's response content
/// * `db` - Database connection
/// * `client` - OpenRouter client for API calls
/// * `model` - Model to use for delegated responses
///
/// # Returns
///
/// Tuple of (cleaned_content, optional_delegate_result)
async fn process_delegate_commands(
    content: &str,
    db: &Database,
    client: &OpenRouterClient,
    model: &str,
) -> (String, Option<String>) {
    let mut result_message = None;
    let mut cleaned = content.to_string();

    // Check for DELEGATE:HYDRIX command
    if let Some(start) = content.find("[DELEGATE:HYDRIX:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let request = command
                .trim_start_matches("[DELEGATE:HYDRIX:")
                .trim_end_matches(']');

            let result = delegate_to_agent("Hydrix", request, db, client, model).await;
            result_message = Some(result);
            cleaned = cleaned.replace(command, "");
        }
    }

    // Check for DELEGATE:SERHANT command
    if let Some(start) = content.find("[DELEGATE:SERHANT:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let request = command
                .trim_start_matches("[DELEGATE:SERHANT:")
                .trim_end_matches(']');

            let result = delegate_to_agent("Serhant", request, db, client, model).await;
            result_message = Some(result);
            cleaned = cleaned.replace(command, "");
        }
    }

    // Check for DELEGATE:KAREN command
    if let Some(start) = content.find("[DELEGATE:KAREN:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let request = command
                .trim_start_matches("[DELEGATE:KAREN:")
                .trim_end_matches(']');

            let result = delegate_to_agent("Karen", request, db, client, model).await;
            result_message = Some(result);
            cleaned = cleaned.replace(command, "");
        }
    }

    // Check for DELEGATE:BOTH command (coordinate multiple agents)
    if let Some(start) = content.find("[DELEGATE:BOTH:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let request = command
                .trim_start_matches("[DELEGATE:BOTH:")
                .trim_end_matches(']');

            // Get responses from both Hydrix and Serhant
            let hydrix_response = delegate_to_agent("Hydrix", request, db, client, model).await;
            let serhant_response = delegate_to_agent("Serhant", request, db, client, model).await;

            let result = format!(
                "🌊 Hydrix:\n{}\n\n⚡ Serhant:\n{}",
                hydrix_response, serhant_response
            );
            result_message = Some(result);
            cleaned = cleaned.replace(command, "");
        }
    }

    // Clean up any extra whitespace
    cleaned = cleaned.trim().to_string();

    (cleaned, result_message)
}

/// Process multi-agent commands in AI response for coordinated responses.
///
/// This allows Mio to orchestrate multiple agents working together on complex requests.
/// Syntax: [MULTI_AGENT:Agent1,Agent2,Agent3:request]
///
/// # Arguments
///
/// * `content` - The AI's response content
/// * `db` - Database connection
/// * `client` - OpenRouter client for API calls
/// * `model` - Model to use for agent responses
///
/// # Returns
///
/// Tuple of (cleaned_content, optional_multi_agent_result)
async fn process_multi_agent_commands(
    content: &str,
    db: &Database,
    client: &OpenRouterClient,
    model: &str,
) -> (String, Option<String>) {
    let mut result_message = None;
    let mut cleaned = content.to_string();

    // Check for MULTI_AGENT command
    if let Some(start) = content.find("[MULTI_AGENT:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let inner = command
                .trim_start_matches("[MULTI_AGENT:")
                .trim_end_matches(']');

            // Parse agents and request: "Agent1,Agent2:request"
            if let Some(colon_pos) = inner.find(':') {
                let agents_str = &inner[..colon_pos];
                let request = &inner[colon_pos + 1..];

                let agent_names: Vec<&str> = agents_str
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                if !agent_names.is_empty() {
                    // Collect responses from all agents
                    let mut responses = Vec::new();

                    for agent_name in agent_names {
                        let response = delegate_to_agent(agent_name, request, db, client, model).await;

                        // Get agent emoji/icon
                        let agent_icon = match agent_name {
                            "Hydrix" => "💧",
                            "Serhant" => "⚡",
                            "Karen" => "📋",
                            "Mio" => "✨",
                            _ => "🤖",
                        };

                        responses.push(format!("{} {}:\n{}", agent_icon, agent_name, response));
                    }

                    result_message = Some(responses.join("\n\n"));
                    cleaned = cleaned.replace(command, "");
                }
            }
        }
    }

    // Clean up any extra whitespace
    cleaned = cleaned.trim().to_string();

    (cleaned, result_message)
}

/// Process grep commands in AI response.
///
/// # Arguments
///
/// * `content` - The AI's response content
///
/// # Returns
///
/// Tuple of (cleaned_content, optional_grep_result)
fn process_grep_commands(content: &str) -> (String, Option<String>) {
    let mut result_message = None;
    let mut cleaned = content.to_string();

    // Load security config for path validation
    let security_config = match SecurityConfig::load_from_file("configs/security.json") {
        Ok(config) => config,
        Err(_) => SecurityConfig::default(),
    };

    // Check for GREP:SEARCH command
    if let Some(start) = content.find("[GREP:SEARCH:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let params = command
                .trim_start_matches("[GREP:SEARCH:")
                .trim_end_matches(']');

            let parts: Vec<&str> = params.split('|').collect();
            if !parts.is_empty() {
                let pattern = parts[0];
                let search_path = if parts.len() > 1 { parts[1] } else { "." };

                // Validate path is within sandbox
                if !security_config.is_path_safe(search_path) {
                    result_message = Some(format!(
                        "Error: Access denied - path '{}' is outside the allowed workspace",
                        search_path
                    ));
                } else {
                    let result = execute_ripgrep_search(pattern, search_path);
                    result_message = Some(result);
                }
            }
            cleaned = cleaned.replace(command, "");
        }
    }

    // Check for GREP:READ command (with optional line range)
    if let Some(start) = content.find("[GREP:READ:") {
        if let Some(end) = content[start..].find(']') {
            let command = &content[start..start + end + 1];
            let params = command
                .trim_start_matches("[GREP:READ:")
                .trim_end_matches(']');

            let parts: Vec<&str> = params.split('|').collect();
            if !parts.is_empty() {
                let file_path = parts[0];

                // Validate path is within sandbox
                if !security_config.is_path_safe(file_path) {
                    result_message = Some(format!(
                        "Error: Access denied - path '{}' is outside the allowed workspace",
                        file_path
                    ));
                } else if parts.len() == 3 {
                    // Read with line range
                    if let (Ok(start_line), Ok(end_line)) = (
                        parts[1].parse::<usize>(),
                        parts[2].parse::<usize>(),
                    ) {
                        let result = read_file_lines(file_path, Some((start_line, end_line)));
                        result_message = Some(result);
                    } else {
                        result_message = Some("Error: Invalid line numbers".to_string());
                    }
                } else {
                    // Read entire file
                    let result = read_file_lines(file_path, None);
                    result_message = Some(result);
                }
            }
            cleaned = cleaned.replace(command, "");
        }
    }

    // Clean up any extra whitespace
    cleaned = cleaned.trim().to_string();

    (cleaned, result_message)
}

/// Execute ripgrep search.
fn execute_ripgrep_search(pattern: &str, search_path: &str) -> String {
    // Try using ripgrep (rg) first
    let rg_result = Command::new("rg")
        .arg("--line-number")
        .arg("--no-heading")
        .arg("--color=never")
        .arg("--max-count=50") // Limit to 50 matches per file
        .arg(pattern)
        .arg(search_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match rg_result {
        Ok(child) => {
            let output = child.wait_with_output();
            match output {
                Ok(output) => {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        if stdout.is_empty() {
                            format!("No matches found for pattern '{}'", pattern)
                        } else {
                            let lines: Vec<&str> = stdout.lines().take(100).collect(); // Limit total output
                            if lines.len() == 100 {
                                format!(
                                    "=== Search Results (showing first 100 matches) ===\n\n{}\n\n... (results truncated)",
                                    lines.join("\n")
                                )
                            } else {
                                format!(
                                    "=== Search Results ({} matches) ===\n\n{}",
                                    lines.len(),
                                    lines.join("\n")
                                )
                            }
                        }
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if stderr.contains("No such file or directory") {
                            format!("Error: Path '{}' not found", search_path)
                        } else {
                            format!("Search error: {}", stderr)
                        }
                    }
                }
                Err(e) => format!("Error executing search: {}", e),
            }
        }
        Err(_) => {
            // Fallback message if ripgrep not available
            "Error: ripgrep (rg) is not installed. Please install it to use search functionality.".to_string()
        }
    }
}

/// Read file contents with optional line range.
fn read_file_lines(file_path: &str, line_range: Option<(usize, usize)>) -> String {
    // Validate path before reading
    let security_config = match SecurityConfig::load_from_file("configs/security.json") {
        Ok(config) => config,
        Err(_) => SecurityConfig::default(),
    };

    // Use validate_path for stricter validation (follows symlinks, canonicalizes)
    let validated_path = match security_config.validate_path(file_path) {
        Ok(path) => path,
        Err(e) => return format!("Security error: {}", e),
    };

    // Get relative path for display (cleaner output)
    let display_path = security_config
        .get_relative_path(file_path)
        .unwrap_or_else(|_| PathBuf::from(file_path));

    match fs::read_to_string(&validated_path) {
        Ok(contents) => {
            let lines: Vec<&str> = contents.lines().collect();

            if let Some((start, end)) = line_range {
                if start == 0 || start > lines.len() || end > lines.len() || start > end {
                    return format!(
                        "Error: Invalid line range {}:{} for file with {} lines",
                        start,
                        end,
                        lines.len()
                    );
                }

                let selected_lines: Vec<String> = lines[(start - 1)..end]
                    .iter()
                    .enumerate()
                    .map(|(i, line)| format!("{:4} | {}", start + i, line))
                    .collect();

                format!(
                    "=== {} (lines {}-{}) ===\n\n{}",
                    display_path.display(),
                    start,
                    end,
                    selected_lines.join("\n")
                )
            } else {
                // Read entire file
                let max_lines = 500; // Limit total lines to prevent overwhelming output
                let total_lines = lines.len();

                let display_lines: Vec<String> = lines
                    .iter()
                    .take(max_lines)
                    .enumerate()
                    .map(|(i, line)| format!("{:4} | {}", i + 1, line))
                    .collect();

                if total_lines > max_lines {
                    format!(
                        "=== {} (showing first {} of {} lines) ===\n\n{}\n\n... (file truncated)",
                        display_path.display(), max_lines, total_lines,
                        display_lines.join("\n")
                    )
                } else {
                    format!(
                        "=== {} ({} lines) ===\n\n{}",
                        display_path.display(),
                        total_lines,
                        display_lines.join("\n")
                    )
                }
            }
        }
        Err(e) => format!("Error reading file '{}': {}", file_path, e),
    }
}

/// Delegate a request to a specific agent.
async fn delegate_to_agent(
    agent_name: &str,
    request: &str,
    db: &Database,
    client: &OpenRouterClient,
    model: &str,
) -> String {
    let agent_system = AgentSystem::new(db.clone());

    // Get the agent details
    let agent = match agent_system.get_agent(agent_name).await {
        Ok(Some(a)) => a,
        Ok(None) => return format!("{} is not available right now.", agent_name),
        Err(e) => return format!("Error connecting to {}: {}", agent_name, e),
    };

    // Build personality-aware agent prompt
    let agent_prompt = if let Some(profile) = PersonalityProfile::get_profile(&agent.personality_type) {
        // Use detailed personality profile
        let mut prompt = profile.build_agent_prompt(
            &agent.name,
            &agent.title,
            agent.backstory.as_deref().unwrap_or(""),
            &agent.current_mood
        );

        prompt.push_str(&format!(
            "=== CURRENT INTERACTION ===\n\
             Relationship Level with User: {}/10\n\
             User's Request: \"{}\"\n\n\
             Respond in character, maintaining your unique voice and personality. \
             Use your characteristic tone, vocabulary, and style.",
            agent.relationship_level, request
        ));

        prompt
    } else {
        // Fallback to basic prompt if personality profile not found
        let mut prompt = format!(
            "You are {}, {}.\n\n",
            agent.name,
            agent.title
        );

        if let Some(backstory) = &agent.backstory {
            prompt.push_str(&format!("Background: {}\n\n", backstory));
        }

        prompt.push_str(&format!(
            "Current Mood: {}\n\
             Relationship Level: {}\n\n\
             The user has asked: \"{}\"\n\n\
             Respond as {} would, using your expertise and personality. Be helpful and stay in character.",
            agent.current_mood, agent.relationship_level, request, agent.name
        ));

        prompt
    };

    // Call the API to get agent's response
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: agent_prompt,
        },
        Message {
            role: "user".to_string(),
            content: request.to_string(),
        },
    ];

    match client.chat_completion(model, messages, Some(500)).await {
        Ok(response) => {
            let reply = response.choices[0].message.content.clone();

            // Log the interaction
            let _ = agent_system
                .log_interaction(agent_name, "delegation", &reply, &agent.current_mood, 2)
                .await;

            reply
        }
        Err(e) => format!("Error getting response from {}: {}", agent_name, e),
    }
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
