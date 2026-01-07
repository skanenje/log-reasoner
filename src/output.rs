use crate::models::LogGroup;
use crate::grouper::GroupStats;

pub struct OutputFormatter;

impl OutputFormatter {
    /// Format output as human-readable text
    pub fn format_text(groups: &[LogGroup], stats: &GroupStats, top_n: usize) {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  LOG ANALYSIS RESULTS");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        println!("Top {} failure patterns:\n", top_n);
        
        for (i, group) in groups.iter().take(top_n).enumerate() {
            println!("┌─ Pattern #{}", i + 1);
            println!("│");
            println!("│  Message: {}", group.pattern);
            println!("│  Occurrences: {}", group.count);
            println!("│  Level: {:?}", group.dominant_level.as_ref().unwrap_or(&crate::models::LogLevel::Info));
            
            if let Some((start, end)) = group.time_window {
                let duration = end.signed_duration_since(start);
                println!("│  Time span: {} seconds", duration.num_seconds());
                println!("│  First seen: {}", start.format("%Y-%m-%d %H:%M:%S"));
                println!("│  Last seen: {}", end.format("%Y-%m-%d %H:%M:%S"));
            }
            
            println!("└─");
            println!();
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  SUMMARY");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        println!("  Total events: {}", stats.total_events);
        println!("  Unique patterns: {}", stats.unique_patterns);
        println!("  Largest cluster: {} events", stats.largest_group);
        println!();
    }

    /// Format output as JSON
    pub fn format_json(groups: &[LogGroup], stats: &GroupStats, top_n: usize) {
        use std::collections::HashMap;

        let mut output = HashMap::new();
        
        let patterns: Vec<HashMap<&str, String>> = groups
            .iter()
            .take(top_n)
            .map(|g| {
                let mut map = HashMap::new();
                map.insert("pattern", g.pattern.clone());
                map.insert("count", g.count.to_string());
                map.insert("level", format!("{:?}", g.dominant_level));
                
                if let Some((start, end)) = g.time_window {
                    map.insert("time_window_start", start.to_rfc3339());
                    map.insert("time_window_end", end.to_rfc3339());
                }
                
                map
            })
            .collect();

        output.insert("patterns", format!("{:?}", patterns));
        output.insert("total_events", stats.total_events.to_string());
        output.insert("unique_patterns", stats.unique_patterns.to_string());

        println!("{}", serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string()));
    }
}