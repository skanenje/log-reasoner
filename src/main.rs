mod models;
mod ingest;
mod grouper;
mod cli;
mod output;
mod backends;
mod embedding;

use clap::Parser;
use cli::{Cli, Commands};
use ingest::LogParser;
use grouper::LogGrouper;
use output::OutputFormatter;
use backends::ollama::OllamaBackend;
use embedding::EmbeddingGenerator;
use std::time::Instant;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            file,
            top,
            min_count,
            output,
            errors_only,
        } => {
            analyze_logs(&file, top, min_count, &output, errors_only);
        }
    }
}

fn analyze_logs(file_path: &str, top_n: usize, min_count: usize, output_format: &str, errors_only: bool) {
    println!("Log Reasoner v0.1.0");
    println!("Analyzing: {}\n", file_path);

    // Step 1: Parse logs
    let start = Instant::now();
    let parser = LogParser::new();
    let mut events = match parser.parse_file(file_path) {
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

    // Filter for errors only if requested
    if errors_only {
        events.retain(|e| {
            matches!(e.level, Some(models::LogLevel::Error))
        });
        println!("✓ Filtered to {} ERROR events", events.len());
    }

    // Step 2: Group similar events
    let start = Instant::now();
    let grouper = LogGrouper::new();
    let mut groups = grouper.group_events(events);
    let group_time = start.elapsed();
    
    // Filter by minimum count
    groups.retain(|g| g.count >= min_count);
    
    let stats = LogGrouper::get_stats(&groups);
    println!("✓ Grouped into {} unique patterns ({:.2?})", stats.unique_patterns, group_time);

    // Step 3: Generate embeddings (optional - check if Ollama is available)
    let ollama = OllamaBackend::new();
    
    match ollama.check_available() {
        Ok(_) => {
            println!("\n✓ Ollama detected, generating embeddings...");
            
            let start = Instant::now();
            let embedding_gen = EmbeddingGenerator::new(ollama);
            
            match embedding_gen.embed_groups(&groups) {
                Ok(embeddings) => {
                    let embed_time = start.elapsed();
                    println!("✓ Generated embeddings ({:.2?})", embed_time);
                    
                    // TODO: Use embeddings for semantic clustering (Step 15)
                    // For now, just show we got them
                    println!("  Embedding dimension: {}", embeddings[0].1.len());
                }
                Err(e) => {
                    eprintln!("⚠ Warning: Failed to generate embeddings: {}", e);
                    eprintln!("  Continuing with pattern-based grouping only...\n");
                }
            }
        }
        Err(e) => {
            eprintln!("⚠ Warning: Ollama not available: {}", e);
            eprintln!("  Continuing with pattern-based grouping only...\n");
        }
    }

    // Step 4: Output results
    match output_format {
        "json" => OutputFormatter::format_json(&groups, &stats, top_n),
        _ => OutputFormatter::format_text(&groups, &stats, top_n),
    }
}