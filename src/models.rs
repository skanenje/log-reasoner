#![allow(dead_code)]
use chrono::{DateTime, Utc};
use std::collections::HashMap;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// Represents a group of similar log events
#[derive(Debug, Clone)]
pub struct LogGroup {
    /// Representative message (the pattern)
    pub pattern: String,
    
    /// All events in this group
    pub events: Vec<LogEvent>,
    
    /// Count of events
    pub count: usize,
    
    /// Most common log level in this group
    pub dominant_level: Option<LogLevel>,
    
    /// Time window (earliest to latest)
    pub time_window: Option<(DateTime<Utc>, DateTime<Utc>)>,
    
    /// Track level counts incrementally for efficiency
    level_counts: HashMap<LogLevel, usize>,
}

impl LogGroup {
    pub fn new(pattern: String) -> Self {
        Self {
            pattern,
            events: Vec::new(),
            count: 0,
            dominant_level: None,
            time_window: None,
            level_counts: HashMap::new(),
        }
    }

    /// Add an event to this group
    pub fn add_event(&mut self, event: LogEvent) {
        // Update time window
        if let Some(ts) = event.timestamp {
            self.time_window = match self.time_window {
                None => Some((ts, ts)),
                Some((start, end)) => {
                    Some((start.min(ts), end.max(ts)))
                }
            };
        }

        // Update level counts incrementally (O(1) instead of O(N))
        if let Some(ref level) = event.level {
            // Get current dominant count before mutating
            let current_dominant_count = self.dominant_level
                .as_ref()
                .and_then(|dl| self.level_counts.get(dl).copied())
                .unwrap_or(0);
            
            // Update the count for this level
            let count = self.level_counts.entry(level.clone()).or_insert(0);
            *count += 1;
            let new_count = *count;
            
            // Update dominant level if this level now has the highest count
            if new_count > current_dominant_count {
                self.dominant_level = Some(level.clone());
            }
        }

        self.events.push(event);
        self.count += 1;
    }

}