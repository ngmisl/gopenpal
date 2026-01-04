//! Utility functions for common operations with proper error handling.

use crate::error::{AppError, Result};
use chrono::NaiveTime;

/// Parse a time string in HH:MM format.
///
/// # Arguments
///
/// * `time_str` - Time string in HH:MM format (e.g., "09:00", "17:30")
///
/// # Returns
///
/// Returns `Ok(NaiveTime)` if parsing succeeds, or `Err(AppError::TimeParse)` if invalid.
///
/// # Examples
///
/// ```
/// use gopenpal::utils::parse_time;
///
/// let time = parse_time("14:30").unwrap();
/// assert_eq!(time.hour(), 14);
/// assert_eq!(time.minute(), 30);
/// ```
pub fn parse_time(time_str: &str) -> Result<NaiveTime> {
    NaiveTime::parse_from_str(time_str, "%H:%M")
        .map_err(|e| AppError::TimeParse(format!(
            "Failed to parse time '{}': {}. Expected format: HH:MM",
            time_str, e
        )))
}

/// Get a required environment variable.
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Returns
///
/// Returns `Ok(String)` with the variable value, or `Err(AppError::Env)` if not found or invalid.
///
/// # Examples
///
/// ```
/// use gopenpal::utils::get_required_env;
///
/// let api_key = get_required_env("OPENROUTER_API_KEY")?;
/// ```
pub fn get_required_env(key: &str) -> Result<String> {
    std::env::var(key)
        .map_err(|e| AppError::Env(format!(
            "Environment variable '{}': {}",
            key, e
        )))
}

/// Get an optional environment variable with a default value.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `default` - Default value if not found
///
/// # Returns
///
/// Returns the environment variable value or the default.
pub fn get_env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn test_parse_time_valid() {
        let time = parse_time("14:30").unwrap();
        assert_eq!(time.hour(), 14);
        assert_eq!(time.minute(), 30);
    }

    #[test]
    fn test_parse_time_invalid() {
        let result = parse_time("25:00");
        assert!(result.is_err());
        match result {
            Err(AppError::TimeParse(_)) => (),
            _ => panic!("Expected TimeParse error"),
        }
    }

    #[test]
    fn test_get_env_or_fallback() {
        let value = get_env_or("NONEXISTENT_VAR_12345", "default");
        assert_eq!(value, "default");
    }
}
