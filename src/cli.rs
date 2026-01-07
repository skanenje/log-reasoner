use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "log-reasoner")]
#[command(version = "0.1.0")]
#[command(about = "AI-powered log analysis tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze a log file and generate insights
    Analyze {
        /// Path to the log file
        #[arg(value_name = "FILE")]
        file: String,

        /// Number of top patterns to display
        #[arg(short = 't', long = "top", default_value = "5")]
        top: usize,

        /// Minimum occurrences to report a pattern
        #[arg(short = 'm', long = "min-count", default_value = "1")]
        min_count: usize,

        /// Output format (text or json)
        #[arg(short = 'o', long = "output", default_value = "text")]
        output: String,

        /// Show only ERROR level logs
        #[arg(long = "errors-only")]
        errors_only: bool,
    },
}