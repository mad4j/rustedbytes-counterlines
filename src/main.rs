// main.rs - Entry point for SLOC counter CLI tool
// Implements: REQ-8.1, REQ-8.2, REQ-8.3, REQ-8.4

mod cli;
mod config;
mod counter;
mod error;
mod language;
mod output;
mod processor;
mod report;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    // REQ-8.1: Provide a command-line interface
    let cli = Cli::parse();

    // REQ-8.3: Support multiple commands
    match cli.command {
        Commands::Count(args) => {
            // REQ-8.3: count command
            counter::execute_count(args)?;
        }
        Commands::Report(args) => {
            // REQ-8.3: report command
            report::execute_report(args)?;
        }
        Commands::Process(args) => {
            // REQ-8.3: process command
            processor::execute_process(args)?;
        }
        Commands::Compare(args) => {
            // REQ-8.3: compare command
            processor::execute_compare(args)?;
        }
    }

    Ok(())
}
