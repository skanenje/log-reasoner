# Log Reasoner

A powerful, fast log analysis tool that intelligently groups and analyzes log events to identify patterns and provide actionable insights.

## Features

- **Intelligent Pattern Recognition**: Automatically groups similar log events by normalizing variable parts (numbers, IP addresses, UUIDs, etc.)
- **Multiple Log Format Support**: 
  - Common Log Format (CLF)
  - ISO8601 timestamps
  - Standard log levels (ERROR, WARN, INFO, DEBUG, TRACE)
- **Error Filtering**: Focus on error-level logs with the `--errors-only` flag
- **Flexible Output**: Human-readable text format or structured JSON output
- **Performance**: Built with Rust for fast parsing and analysis of large log files
- **Statistics**: Provides comprehensive statistics including total events, unique patterns, and time windows

## Installation

### Prerequisites

- Rust 1.70+ and Cargo installed ([rustup.rs](https://rustup.rs/))

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd log-reasoner

# Build the project
cargo build --release

# The binary will be available at target/release/log-reasoner
```

### Development Build

```bash
cargo build
# Binary available at target/debug/log-reasoner
```

## Usage

### Basic Analysis

Analyze a log file and display the top 5 patterns:

```bash
log-reasoner analyze <log-file>
```

### Advanced Options

```bash
log-reasoner analyze <log-file> \
    --top 10 \              # Show top 10 patterns (default: 5)
    --min-count 5 \         # Only show patterns with at least 5 occurrences (default: 1)
    --output json \         # Output as JSON (default: text)
    --errors-only           # Filter to only ERROR level logs
```

### Examples

**Analyze all logs and show top 10 patterns:**
```bash
log-reasoner analyze test_logs.txt --top 10
```

**Analyze only errors in JSON format:**
```bash
log-reasoner analyze app.log --errors-only --output json --top 20
```

**Find patterns that occur at least 10 times:**
```bash
log-reasoner analyze server.log --min-count 10
```

## How It Works

1. **Parsing**: The tool parses log files line by line, extracting:
   - Timestamps (ISO8601 or CLF format)
   - Log levels (ERROR, WARN, INFO, etc.)
   - Log messages

2. **Normalization**: Variable parts of log messages are normalized:
   - Numbers → `<VAR>`
   - IP addresses → `<VAR>`
   - UUIDs → `<VAR>`
   
   This allows similar log events to be grouped together.

3. **Grouping**: Log events with the same normalized pattern are grouped together, tracking:
   - Total occurrences
   - Dominant log level
   - Time window (earliest to latest occurrence)

4. **Analysis**: Groups are sorted by frequency and filtered based on your criteria.

## Output Format

### Text Output

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  LOG ANALYSIS RESULTS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Top 5 failure patterns:

┌─ Pattern #1
│
│  Message: <VAR> - - [<VAR>] "GET / HTTP/<VAR>" <VAR> <VAR>
│  Occurrences: 150
│  Level: Info
│  Time span: 3600 seconds
│  First seen: 2009-07-15 14:58:59
│  Last seen: 2009-07-15 15:58:59
└─

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  SUMMARY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Total events: 3110
  Unique patterns: 45
  Largest cluster: 150 events
```

### JSON Output

```json
{
  "patterns": [
    {
      "pattern": "<VAR> - - [<VAR>] \"GET / HTTP/<VAR>\" <VAR> <VAR>",
      "count": "150",
      "level": "Some(Info)",
      "time_window_start": "2009-07-15T21:58:59+00:00",
      "time_window_end": "2009-07-15T22:58:59+00:00"
    }
  ],
  "total_events": "3110",
  "unique_patterns": "45"
}
```

## Supported Log Formats

### Common Log Format (CLF)
```
127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET /index.html HTTP/1.0" 200 2326
```

### ISO8601 Timestamps
```
2024-01-05T12:01:03Z ERROR Database connection failed
2024-01-05 12:01:03 WARN Retrying connection...
```

### Standard Log Levels
The tool recognizes the following log levels (case-insensitive):
- `ERROR` / `ERR`
- `WARN` / `WARNING`
- `INFO`
- `DEBUG`
- `TRACE`

## Project Structure

```
log-reasoner/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   ├── main.rs         # Entry point and orchestration
│   ├── cli.rs          # Command-line interface definitions
│   ├── models.rs       # Data structures (LogEvent, LogGroup, LogLevel)
│   ├── ingest.rs       # Log parsing logic
│   ├── grouper.rs      # Pattern grouping and normalization
│   └── output.rs       # Output formatting (text/JSON)
└── test_logs.txt       # Sample log file for testing
```

## Dependencies

- `chrono` - Date and time parsing/manipulation
- `regex` - Regular expression parsing and matching
- `anyhow` - Ergonomic error handling
- `clap` - Command-line argument parsing
- `serde` / `serde_json` - JSON serialization

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Add your license here]

## Version

Current version: 0.1.0

