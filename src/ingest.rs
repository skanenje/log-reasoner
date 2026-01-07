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
}