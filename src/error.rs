//! Error types for the gopenpal application.
//!
//! This module defines custom error types using `thiserror` for library-level
//! errors and provides conversions for use with `anyhow` at the application level.

use thiserror::Error;

/// Application-level errors.
///
/// This enum represents all possible errors that can occur in the application.
/// Each variant provides context about what went wrong.
#[derive(Error, Debug)]
pub enum AppError {
    /// Database operation failed
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// HTTP request failed
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    /// Environment variable not found or invalid
    #[error("Environment variable error: {0}")]
    Env(String),

    /// OpenRouter API error
    #[error("OpenRouter API error: {0}")]
    OpenRouter(String),

    /// Desktop notification error
    #[error("Notification error: {0}")]
    Notification(String),

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid time format
    #[error("Time parse error: {0}")]
    TimeParse(String),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Security violation (path access outside sandbox)
    #[error("Security error: {0}")]
    Security(String),
}

/// Specialized Result type for application errors.
///
/// This type alias simplifies function signatures throughout the codebase
/// by providing a default error type.
pub type Result<T> = std::result::Result<T, AppError>;
