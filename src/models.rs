use chrono::{DateTime, Utc};

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