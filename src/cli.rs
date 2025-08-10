// cli.rs - Command-line interface definitions
// Implements: REQ-8.1, REQ-8.2, REQ-8.3, REQ-8.4, REQ-9.7

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// REQ-8.2: Display help via --help or -h
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // REQ-8.3: count command
    /// Count lines in specified files/directories
    Count(CountArgs),

    // REQ-8.3: report command
    /// Generate a report from counted lines
    Report(ReportArgs),

    // REQ-8.3: process command
    /// Process an existing report
    Process(ProcessArgs),

    // REQ-8.3: compare command
    /// Compare two reports
    Compare(CompareArgs),
}

#[derive(Parser)]
pub struct CountArgs {
    /// Print per-file statistics and unsupported file list (default: false)
    #[arg(long)]
    pub details: bool,
    // REQ-2.1: Accept file and/or directory paths
    // REQ-2.2: Accept wildcards
    /// Paths to files or directories to count
    #[arg(required = true)]
    pub paths: Vec<String>,

    // REQ-2.3: Recursive directory traversal
    /// Recursively traverse directories
    #[arg(short, long)]
    pub recursive: bool,

    // REQ-2.4: Accept input via stdin
    /// Read file paths from stdin
    #[arg(long)]
    pub stdin: bool,

    // REQ-6.1, REQ-6.2, REQ-6.3: Support JSON, XML, CSV
    /// Output format for report
    #[arg(short = 'f', long, value_enum)]
    pub format: Option<OutputFormat>,

    // REQ-6.8: Customize output paths
    /// Output file path for report
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    // REQ-5.4: Sort console output
    /// Sort output by metric
    #[arg(short, long, value_enum)]
    pub sort: Option<SortMetric>,

    // REQ-3.4: Override language detection
    /// Override language detection for specific extensions
    #[arg(long, value_parser = parse_language_override)]
    pub language_override: Vec<(String, String)>,

    // REQ-3.3: Language definitions via config
    /// Path to language configuration file
    #[arg(long)]
    pub config: Option<PathBuf>,

    // REQ-9.5: Progress indicators (inverted logic - enabled by default)
    /// Disable progress bar
    #[arg(long)]
    pub no_progress: bool,

    // REQ-9.4: Parallel processing
    /// Number of parallel threads (0 = auto)
    #[arg(short = 'j', long, default_value = "0")]
    pub threads: usize,

    // REQ-6.9: Optional checksum
    /// Include checksum in report
    #[arg(long)]
    pub checksum: bool,

    // REQ-4.5: Ignore preprocessor directives
    /// Ignore preprocessor directives
    #[arg(long)]
    pub ignore_preprocessor: bool,

    // REQ-9.7: Performance metrics logging
    /// Enable performance metrics logging
    #[arg(long)]
    pub enable_metrics: bool,

    /// Custom metrics log file path
    #[arg(long)]
    pub metrics_file: Option<PathBuf>,

    /// Show performance summary for operations over this threshold (seconds)
    #[arg(long, default_value = "5")]
    pub perf_summary_threshold: u64,
}

#[derive(Parser)]
pub struct ReportArgs {
    /// Print per-file statistics and unsupported file list (default: false)
    #[arg(long)]
    pub details: bool,
    /// Paths to files or directories to count
    #[arg(required = true)]
    pub paths: Vec<String>,

    /// Output format
    #[arg(short = 'f', long, value_enum, default_value = "json")]
    pub format: OutputFormat,

    /// Output file path
    #[arg(short, long, required = true)]
    pub output: PathBuf,

    /// Recursively traverse directories
    #[arg(short, long)]
    pub recursive: bool,

    /// Include checksum in report
    #[arg(long)]
    pub checksum: bool,

    /// Path to language configuration file
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Number of parallel threads
    #[arg(short = 'j', long, default_value = "0")]
    pub threads: usize,

    /// Enable performance metrics logging
    #[arg(long)]
    pub enable_metrics: bool,

    /// Custom metrics log file path
    #[arg(long)]
    pub metrics_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct ProcessArgs {
    // REQ-7.1: Process existing report
    /// Path to the report file
    #[arg(required = true)]
    pub report: PathBuf,

    /// Sort output by metric
    #[arg(short, long, value_enum)]
    pub sort: Option<SortMetric>,

    /// Export processed results
    #[arg(short, long)]
    pub export: Option<PathBuf>,

    /// Export format
    #[arg(short = 'f', long, value_enum)]
    pub format: Option<OutputFormat>,

    /// Enable performance metrics logging
    #[arg(long)]
    pub enable_metrics: bool,

    /// Custom metrics log file path
    #[arg(long)]
    pub metrics_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct CompareArgs {
    // REQ-7.2: Compare two reports
    /// Path to the first report
    #[arg(required = true)]
    pub report1: PathBuf,

    /// Path to the second report
    #[arg(required = true)]
    pub report2: PathBuf,

    // REQ-7.4: Export comparison results
    /// Export comparison results
    #[arg(short, long)]
    pub export: Option<PathBuf>,

    /// Export format
    #[arg(short = 'f', long, value_enum)]
    pub format: Option<OutputFormat>,

    /// Enable performance metrics logging
    #[arg(long)]
    pub enable_metrics: bool,

    /// Custom metrics log file path
    #[arg(long)]
    pub metrics_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    /// JSON format (REQ-6.1)
    Json,
    /// XML format (REQ-6.2)
    Xml,
    /// CSV format (REQ-6.3)
    Csv,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum SortMetric {
    /// Sort by total lines
    Total,
    /// Sort by logical lines
    Logical,
    /// Sort by empty lines
    Empty,
    /// Sort by file name
    Name,
    /// Sort by language
    Language,
}

fn parse_language_override(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() != 2 {
        return Err("Invalid format. Use: ext=language".to_string());
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}
