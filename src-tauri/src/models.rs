use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single prompt entry in the database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptEntry {
    pub id: String,
    pub content: String,
    pub application: String,
    pub timestamp: DateTime<Utc>,
    pub starred: bool,
    pub tags: Vec<String>,
    pub usage_count: i32,
    pub is_encrypted: bool,
}

/// Filter criteria for querying prompts
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptFilter {
    pub application: Option<String>,
    pub starred: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub search_text: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Statistics about prompt usage
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptStats {
    pub total_prompts: i64,
    pub applications: HashMap<String, i64>,
    pub starred_count: i64,
    pub most_used_prompts: Vec<PromptEntry>,
    pub recent_activity: Vec<PromptEntry>,
}

/// Configuration for system monitoring
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub monitored_applications: Vec<String>,
    pub capture_threshold: u32, // Minimum characters to capture
    pub auto_save: bool,
    pub encryption_enabled: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            monitored_applications: vec![
                "ChatGPT".to_string(),
                "Claude".to_string(),
                "Cursor".to_string(),
                "Grok".to_string(),
                "Perplexity".to_string(),
                "Ollama".to_string(),
            ],
            capture_threshold: 10,
            auto_save: true,
            encryption_enabled: true,
        }
    }
}

/// Represents a detected LLM application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedApplication {
    pub name: String,
    pub process_name: String,
    pub window_title: String,
    pub is_active: bool,
    pub last_activity: DateTime<Utc>,
}

/// Request payload for Ollama API
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

/// Response from Ollama API
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    pub done: bool,
    pub model: String,
    pub created_at: String,
}

/// System information structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub cpu_cores: usize,
    pub memory_total: u64,
    pub memory_available: u64,
    pub disk_space: u64,
}

/// Error types for the application
#[derive(Debug, thiserror::Error)]
pub enum PromptHistError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("System monitoring error: {0}")]
    Monitoring(String),

    #[error("System error: {0}")]
    SystemError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl From<String> for PromptHistError {
    fn from(s: String) -> Self {
        PromptHistError::SystemError(s)
    }
}

/// Result type alias for the application
pub type Result<T> = std::result::Result<T, PromptHistError>;
