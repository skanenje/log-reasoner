#![allow(dead_code)]
use chrono::{DateTime, Utc};

/// Represents a single parsed log event
#[derive(Debug, Clone)]
pub struct LogEvent {
    /// Timestamp (if parseable)
    pub timestamp: Option<DateTime<Utc>>,
    
    /// Log level (ERROR, WARN, INFO, etc.)
    pub level: Option<LogLevel>,
    
    /// The actual log message
    pub message: String,
    
    /// Original raw line (for debugging)
    pub raw: String,
}

/// Standard log levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    /// Parse from string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "ERROR" | "ERR" => Some(LogLevel::Error),
            "WARN" | "WARNING" => Some(LogLevel::Warn),
            "INFO" => Some(LogLevel::Info),
            "DEBUG" => Some(LogLevel::Debug),
            "TRACE" => Some(LogLevel::Trace),
            _ => None,
        }
    }
}