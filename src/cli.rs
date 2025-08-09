// cli.rs - Command-line interface definitions
// Implements: REQ-8.1, REQ-8.2, REQ-8.3, REQ-8.4

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

pub struct Cli {
    /// REQ-8.2: Display help via --help or -h
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Count lines in specified files/directories
    /// REQ-8.3: count command
    Count(CountArgs),
    
    /// Generate a report from counted lines
    /// REQ-8.3: report command
    Report(ReportArgs),
    
    /// Process an existing report
    /// REQ-8.3: process command
    Process(ProcessArgs),
    
    /// Compare two reports
    /// REQ-8.3: compare command
    Compare(CompareArgs),
}

#[derive(Parser)]
pub struct CountArgs {
    /// Paths to files or directories to count
    /// REQ-2.1: Accept file and/or directory paths
    /// REQ-2.2: Accept wildcards
    #[arg(required = true)]
    pub paths: Vec<String>,
    
    /// Recursively traverse directories
    /// REQ-2.3: Recursive directory traversal
    #[arg(short, long)]
    pub recursive: bool,
    
    /// Read file paths from stdin
    /// REQ-2.4: Accept input via stdin
    #[arg(long)]
    pub stdin: bool,
    
    /// Output format for report
    /// REQ-6.1, REQ-6.2, REQ-6.3: Support JSON, XML, CSV
    #[arg(short = 'f', long, value_enum)]
    pub format: Option<OutputFormat>,
    
    /// Output file path for report
    /// REQ-6.8: Customize output paths
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    
    /// Sort output by metric
    /// REQ-5.4: Sort console output
    #[arg(short, long, value_enum)]
    pub sort: Option<SortMetric>,
    
    /// Override language detection for specific extensions
    /// REQ-3.4: Override language detection
    #[arg(long, value_parser = parse_language_override)]
    pub language_override: Vec<(String, String)>,
    
    /// Path to language configuration file
    /// REQ-3.3: Language definitions via config
    #[arg(long)]
    pub config: Option<PathBuf>,
    
    /// Disable progress bar
    /// REQ-9.5: Progress indicators (inverted logic - enabled by default)
    #[arg(long)]
    pub no_progress: bool,
    
    /// Number of parallel threads (0 = auto)
    /// REQ-9.4: Parallel processing
    #[arg(short = 'j', long, default_value = "0")]
    pub threads: usize,
    
    /// Include checksum in report
    /// REQ-6.9: Optional checksum
    #[arg(long)]
    pub checksum: bool,
    
    /// Ignore preprocessor directives
    /// REQ-4.5: Ignore preprocessor directives
    #[arg(long)]
    pub ignore_preprocessor: bool,
}

#[derive(Parser)]
pub struct ReportArgs {
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
}

#[derive(Parser)]
pub struct ProcessArgs {
    /// Path to the report file
    /// REQ-7.1: Process existing report
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
}

#[derive(Parser)]
pub struct CompareArgs {
    /// Path to the first report
    /// REQ-7.2: Compare two reports
    #[arg(required = true)]
    pub report1: PathBuf,
    
    /// Path to the second report
    #[arg(required = true)]
    pub report2: PathBuf,
    
    /// Export comparison results
    /// REQ-7.4: Export comparison results
    #[arg(short, long)]
    pub export: Option<PathBuf>,
    
    /// Export format
    #[arg(short = 'f', long, value_enum)]
    pub format: Option<OutputFormat>,
}

#[derive(Clone, Copy, ValueEnum)]
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
        return Err(format!("Invalid format. Use: ext=language"));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}