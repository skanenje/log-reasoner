mod models;
mod ingest;

use ingest::LogParser;

fn main() {
    println!("Log Reasoner v0.1.0\n");

    // For now, we'll test with a hardcoded path
    // Later this will come from CLI args
    let test_log_path = "test_logs.txt";

    let parser = LogParser::new();
    
    match parser.parse_file(test_log_path) {
        Ok(events) => {
            println!("✓ Parsed {} log events\n", events.len());
            
            // Show first 5 events as sample
            for (i, event) in events.iter().take(5).enumerate() {
                println!("Event {}:", i + 1);
                println!("  Timestamp: {:?}", event.timestamp);
                println!("  Level: {:?}", event.level);
                println!("  Message: {}", event.message);
                println!();
            }
        }
        Err(e) => {
            eprintln!("✗ Error parsing logs: {}", e);
            std::process::exit(1);
        }
    }
}