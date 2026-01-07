mod models;
mod ingest;
mod grouper;

use ingest::LogParser;
use grouper::LogGrouper;
use std::time::Instant;

fn main() {
    println!("Log Reasoner v0.1.0\n");

    let test_log_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "test_logs.txt".to_string());

    println!("Analyzing: {}\n", test_log_path);

    // Step 1: Parse logs
    let start = Instant::now();
    let parser = LogParser::new();
    let events = match parser.parse_file(&test_log_path) {
        Ok(events) => {
            let parse_time = start.elapsed();
            println!("✓ Parsed {} log events ({:.2?})", events.len(), parse_time);
            events
        }
        Err(e) => {
            eprintln!("✗ Error parsing logs: {}", e);
            std::process::exit(1);
        }
    };

    // Step 2: Group similar events
    let start = Instant::now();
    let grouper = LogGrouper::new();
    let groups = grouper.group_events(events);
    let group_time = start.elapsed();
    
    let stats = LogGrouper::get_stats(&groups);
    println!("✓ Grouped into {} unique patterns ({:.2?})\n", stats.unique_patterns, group_time);

    // Step 3: Display top groups
    println!("Top failure patterns:\n");
    for (i, group) in groups.iter().take(5).enumerate() {
        println!("{}. Pattern: {}", i + 1, group.pattern);
        println!("   Occurrences: {}", group.count);
        println!("   Level: {:?}", group.dominant_level);
        
        if let Some((start, end)) = group.time_window {
            let duration = end.signed_duration_since(start);
            println!("   Time window: {} seconds", duration.num_seconds());
        }
        
        println!();
    }

    // Step 4: Show stats
    println!("Summary:");
    println!("  Total events: {}", stats.total_events);
    println!("  Unique patterns: {}", stats.unique_patterns);
    println!("  Largest group: {} events", stats.largest_group);
}