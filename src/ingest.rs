use crate::models::{LogEvent, LogLevel};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Parses a log file and returns structured events
pub struct LogParser {
    /// Regex for ISO8601 timestamps
    timestamp_regex: Regex,
    /// Regex for log levels
    level_regex: Regex,
    /// Regex for CLF (Common Log Format) style logs
    clf_regex: Regex,
}

impl LogParser {
    pub fn new() -> Self {
        Self {
            // Matches: 2024-01-05T12:01:03Z or 2024-01-05 12:01:03
            timestamp_regex: Regex::new(
                r"(\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?)"
            ).unwrap(),
            
            // Matches: ERROR, WARN, INFO, etc. (case-insensitive)
            level_regex: Regex::new(
                r"(?i)\b(ERROR|ERR|WARN|WARNING|INFO|DEBUG|TRACE)\b"
            ).unwrap(),

            // Matches: 127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET /index.html HTTP/1.0" 200 2326
            clf_regex: Regex::new(
                r#"^(\S+) \S+ \S+ \[([\w:/]+\s[+\-]\d{4})\] ".*?" (\d{3}) (\d+|-)"#
            ).unwrap(),
        }
    }
        /// Parse a log file from path
    pub fn parse_file(&self, path: &str) -> Result<Vec<LogEvent>> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open log file: {}", path))?;
        
        let reader = BufReader::new(file);
        let mut events = Vec::new();

        for line in reader.lines() {
            let line = line?;
            
            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            events.push(self.parse_line(&line));
        }

        Ok(events)
    }
        /// Parse a single log line
    fn parse_line(&self, line: &str) -> LogEvent {
        let timestamp = self.extract_timestamp(line);
        let level = self.extract_level(line);
        let message = self.extract_message(line, &timestamp, &level);

        LogEvent {
            timestamp,
            level,
            message,
            raw: line.to_string(),
        }
    }
        /// Extract timestamp if present
    fn extract_timestamp(&self, line: &str) -> Option<DateTime<Utc>> {
        // Try CLF format first as it's more specific
        if let Some(cap) = self.clf_regex.captures(line) {
            if let Some(m) = cap.get(2) {
                if let Ok(dt) = DateTime::parse_from_str(m.as_str(), "%d/%b/%Y:%H:%M:%S %z") {
                    return Some(dt.with_timezone(&Utc));
                }
            }
        }

        // Fallback to general timestamp regex
        self.timestamp_regex
            .captures(line)
            .and_then(|cap| cap.get(1))
            .and_then(|m| {
                // Try parsing with multiple formats
                let ts_str = m.as_str();
                
                // ISO8601 with Z
                if let Ok(dt) = DateTime::parse_from_rfc3339(ts_str) {
                    return Some(dt.with_timezone(&Utc));
                }
                
                // Space-separated format
                if let Ok(dt) = DateTime::parse_from_str(
                    &format!("{}Z", ts_str.replace(" ", "T")),
                    "%Y-%m-%dT%H:%M:%S%Z"
                ) {
                    return Some(dt.with_timezone(&Utc));
                }
                
                None
            })
    }
        /// Extract the actual message (remove timestamp and level)
    fn extract_message(&self, line: &str, timestamp: &Option<DateTime<Utc>>, level: &Option<LogLevel>) -> String {
        let mut msg = line.to_string();

        // Remove timestamp if found
        if timestamp.is_some() {
            msg = self.timestamp_regex.replace(&msg, "").to_string();
        }

        // Remove level if found
        if level.is_some() {
            msg = self.level_regex.replace(&msg, "").to_string();
        }

        // Clean up whitespace
        msg.trim().to_string()
    }
        /// Extract log level if present
    fn extract_level(&self, line: &str) -> Option<LogLevel> {
        // Try to find explicit log level first
        if let Some(cap) = self.level_regex.captures(line) {
            if let Some(m) = cap.get(1) {
                return LogLevel::from_str(m.as_str());
            }
        }

        // If no explicit level, check if it's a CLF log and infer from status code
        if let Some(cap) = self.clf_regex.captures(line) {
            if let Some(m) = cap.get(3) {
                if let Ok(status) = m.as_str().parse::<u16>() {
                    return match status {
                        500..=599 => Some(LogLevel::Error),
                        400..=499 => Some(LogLevel::Warn),
                        _ => Some(LogLevel::Info),
                    };
                }
            }
        }

        None
    }
}