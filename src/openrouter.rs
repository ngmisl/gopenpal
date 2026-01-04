//! OpenRouter API client.
//!
//! This module provides a client for interacting with the OpenRouter API,
//! which provides access to various LLM models through a unified interface.

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

/// OpenRouter API client.
///
/// This client handles authentication and communication with the OpenRouter API.
pub struct OpenRouterClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenRouterClient {
    /// Create a new OpenRouter client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - OpenRouter API key
    ///
    /// # Returns
    ///
    /// A new `OpenRouterClient` instance
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    /// Send a chat completion request.
    ///
    /// # Arguments
    ///
    /// * `model` - Model identifier (e.g., "anthropic/claude-3.5-sonnet")
    /// * `messages` - Conversation history
    /// * `max_tokens` - Maximum tokens in response
    ///
    /// # Returns
    ///
    /// Chat completion response with generated text and usage information
    ///
    /// # Errors
    ///
    /// Returns `AppError::Http` if the request fails
    /// Returns `AppError::OpenRouter` if the API returns an error
    pub async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        max_tokens: Option<u32>,
    ) -> Result<ChatCompletionResponse> {
        let request_body = ChatCompletionRequest {
            model: model.to_string(),
            messages,
            max_tokens,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Unknown error"));
            return Err(AppError::OpenRouter(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        let completion_response = response.json::<ChatCompletionResponse>().await?;

        Ok(completion_response)
    }

    /// Stream a chat completion (for future implementation).
    ///
    /// Note: Streaming is not yet implemented but is planned for future versions
    /// to provide real-time response streaming for better UX.
    #[allow(dead_code)]
    pub async fn chat_completion_stream(
        &self,
        _model: &str,
        _messages: Vec<Message>,
    ) -> Result<()> {
        // Placeholder for streaming implementation
        // This would use Server-Sent Events (SSE) to stream responses
        Err(AppError::OpenRouter(
            "Streaming not yet implemented".to_string(),
        ))
    }
}

/// Chat message for API requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    /// Create a user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    /// Create an assistant message.
    #[allow(dead_code)]
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    /// Create a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

/// Chat completion request body.
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

/// Chat completion response from OpenRouter.
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    #[allow(dead_code)]
    pub id: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

/// Individual choice in the response.
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
    #[allow(dead_code)]
    pub finish_reason: Option<String>,
}

/// Token usage information.
#[derive(Debug, Deserialize)]
pub struct Usage {
    #[allow(dead_code)]
    pub prompt_tokens: u32,
    #[allow(dead_code)]
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_constructors() {
        let user_msg = Message::user("Hello");
        assert_eq!(user_msg.role, "user");
        assert_eq!(user_msg.content, "Hello");

        let assistant_msg = Message::assistant("Hi there!");
        assert_eq!(assistant_msg.role, "assistant");

        let system_msg = Message::system("You are helpful");
        assert_eq!(system_msg.role, "system");
    }

    #[test]
    fn test_client_creation() {
        let client = OpenRouterClient::new("test-key".to_string());
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, "https://openrouter.ai/api/v1");
    }
}
