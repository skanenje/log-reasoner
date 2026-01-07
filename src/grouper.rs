use crate::models::{LogEvent, LogGroup};
use regex::Regex;
use std::collections::HashMap;

pub struct LogGrouper {
    /// Regex to replace variable parts (numbers, IDs, timestamps)
    normalizer: Regex,
}

impl LogGrouper {
    pub fn new() -> Self {
        Self {
            // Replace numbers, UUIDs, IPs, etc. with placeholders
            normalizer: Regex::new(
                r"(\d+\.?\d*|[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}|\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b)"
            ).unwrap(),
        }
    }

    /// Group log events by pattern
    pub fn group_events(&self, events: Vec<LogEvent>) -> Vec<LogGroup> {
        let mut groups: HashMap<String, LogGroup> = HashMap::new();

        for event in events {
            let pattern = self.normalize_message(&event.message);
            
            groups
                .entry(pattern.clone())
                .or_insert_with(|| LogGroup::new(pattern))
                .add_event(event);
        }

        // Convert to sorted vector (most frequent first)
        let mut result: Vec<LogGroup> = groups.into_values().collect();
        result.sort_by(|a, b| b.count.cmp(&a.count));

        result
    }

    /// Normalize a message by replacing variable parts
    fn normalize_message(&self, message: &str) -> String {
        // Replace all numbers/IDs with <VAR>
        let normalized = self.normalizer.replace_all(message, "<VAR>");
        
        // Clean up multiple spaces
        normalized.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Get statistics about grouping
    pub fn get_stats(groups: &[LogGroup]) -> GroupStats {
        let total_events: usize = groups.iter().map(|g| g.count).sum();
        let unique_patterns = groups.len();
        let largest_group = groups.first().map(|g| g.count).unwrap_or(0);

        GroupStats {
            total_events,
            unique_patterns,
            largest_group,
        }
    }
}

/// Statistics about log grouping
#[derive(Debug)]
pub struct GroupStats {
    pub total_events: usize,
    pub unique_patterns: usize,
    pub largest_group: usize,
}