fn main() {
    println!("Hello, world!");
}
mod models;

use models::{LogEvent, LogLevel};

fn main() {
    println!("Log Reasoner v0.1.0");
    
    // Placeholder - we'll build this incrementally
    let sample_log = LogEvent {
        timestamp: None,
        level: Some(LogLevel::Error),
        message: "Database connection timeout".to_string(),
        raw: "2024-01-05 12:01:03 ERROR Database connection timeout".to_string(),
    };
    
    println!("Parsed log: {:?}", sample_log);
}