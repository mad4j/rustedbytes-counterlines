// processor.rs - Report processing and comparison
// Implements: REQ-7.1, REQ-7.2, REQ-7.3, REQ-7.4

use crate::cli::{CompareArgs, ProcessArgs, OutputFormat};
use crate::error::{Result, SlocError};
use crate::output::{ConsoleOutput, ReportExporter};
use crate::report::Report;
use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// REQ-7.1: Process existing report without rescanning
pub fn execute_process(args: ProcessArgs) -> Result<()> {
    // Detect format from file extension
    let format = args.format.unwrap_or_else(|| {
        if args.report.extension().and_then(|e| e.to_str()) == Some("json") {
            OutputFormat::Json
        } else if args.report.extension().and_then(|e| e.to_str()) == Some("xml") {
            OutputFormat::Xml
        } else if args.report.extension().and_then(|e| e.to_str()) == Some("csv") {
            OutputFormat::Csv
        } else {
            OutputFormat::Json // Default
        }
    });
    
    let report = Report::from_file(&args.report, format)?;
    
    // Display summary (REQ-7.1: compute global statistics)
    let console = ConsoleOutput::new(args.sort);
    console.display_summary(&report)?;
    
    // Export if requested
    if let Some(export_path) = args.export {
        let export_format = args.format.unwrap_or(OutputFormat::Json);
        let exporter = ReportExporter::new();
        exporter.export(&report, &export_path, export_format)?;
        println!("\nProcessed report exported to: {}", export_path.display());
    }
    
    Ok(())
}

/// REQ-7.2, REQ-7.3, REQ-7.4: Compare two reports
pub fn execute_compare(args: CompareArgs) -> Result<()> {
    // Detect formats
    let format1 = detect_format(&args.report1);
    let format2 = detect_format(&args.report2);
    
    let report1 = Report::from_file(&args.report1, format1)?;
    let report2 = Report::from_file(&args.report2, format2)?;
    
    let comparison = ComparisonResult::compare(&report1, &report2);
    
    // REQ-7.3: Display comparison in console
    display_comparison(&comparison)?;
    
    // REQ-7.4: Export comparison if requested
    if let Some(export_path) = args.export {
        let format = args.format.unwrap_or(OutputFormat::Json);
        export_comparison(&comparison, &export_path, format)?;
        println!("\nComparison exported to: {}", export_path.display());
    }
    
    Ok(())
}

fn detect_format(path: &std::path::PathBuf) -> OutputFormat {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => OutputFormat::Json,
        Some("xml") => OutputFormat::Xml,
        Some("csv") => OutputFormat::Csv,
        _ => OutputFormat::Json,
    }
}

/// REQ-7.2: Comparison result structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub report1_generated: chrono::DateTime<chrono::Utc>,
    pub report2_generated: chrono::DateTime<chrono::Utc>,
    pub global_delta: GlobalDelta,
    pub language_deltas: Vec<LanguageDelta>,
    pub new_files: Vec<String>,
    pub removed_files: Vec<String>,
    pub modified_files: Vec<FileDelta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalDelta {
    pub files_delta: i64,
    pub total_lines_delta: i64,
    pub logical_lines_delta: i64,
    pub empty_lines_delta: i64,
    pub languages_delta: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageDelta {
    pub language: String,
    pub files_delta: i64,
    pub total_lines_delta: i64,
    pub logical_lines_delta: i64,
    pub empty_lines_delta: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDelta {
    pub path: String,
    pub total_lines_delta: i64,
    pub logical_lines_delta: i64,
    pub empty_lines_delta: i64,
}

impl ComparisonResult {
    /// REQ-7.2: Compare two reports
    fn compare(report1: &Report, report2: &Report) -> Self {
        // Create file maps for comparison
        let files1: HashMap<_, _> = report1.files.iter()
            .map(|f| (f.path.clone(), f))
            .collect();
        let files2: HashMap<_, _> = report2.files.iter()
            .map(|f| (f.path.clone(), f))
            .collect();
        
        // Find new, removed, and modified files
        let mut new_files = Vec::new();
        let mut removed_files = Vec::new();
        let mut modified_files = Vec::new();
        
        for (path, file2) in &files2 {
            if let Some(file1) = files1.get(path) {
                // File exists in both - check if modified
                if file1.total_lines != file2.total_lines ||
                   file1.logical_lines != file2.logical_lines ||
                   file1.empty_lines != file2.empty_lines {
                    modified_files.push(FileDelta {
                        path: path.to_string_lossy().to_string(),
                        total_lines_delta: file2.total_lines as i64 - file1.total_lines as i64,
                        logical_lines_delta: file2.logical_lines as i64 - file1.logical_lines as i64,
                        empty_lines_delta: file2.empty_lines as i64 - file1.empty_lines as i64,
                    });
                }
            } else {
                new_files.push(path.to_string_lossy().to_string());
            }
        }
        
        for path in files1.keys() {
            if !files2.contains_key(path) {
                removed_files.push(path.to_string_lossy().to_string());
            }
        }
        
        // Calculate global deltas
        let global_delta = GlobalDelta {
            files_delta: report2.summary.total_files as i64 - report1.summary.total_files as i64,
            total_lines_delta: report2.summary.total_lines as i64 - report1.summary.total_lines as i64,
            logical_lines_delta: report2.summary.logical_lines as i64 - report1.summary.logical_lines as i64,
            empty_lines_delta: report2.summary.empty_lines as i64 - report1.summary.empty_lines as i64,
            languages_delta: report2.summary.languages_count as i64 - report1.summary.languages_count as i64,
        };
        
        // Calculate language deltas
        let lang1: HashMap<_, _> = report1.languages.iter()
            .map(|l| (l.language.clone(), l))
            .collect();
        let lang2: HashMap<_, _> = report2.languages.iter()
            .map(|l| (l.language.clone(), l))
            .collect();
        
        let mut language_deltas = Vec::new();
        let all_languages = lang1.keys().chain(lang2.keys()).collect::<std::collections::HashSet<_>>();
        
        for language in all_languages {
            let stats1 = lang1.get(&*language);
            let stats2 = lang2.get(&*language);
            
            let delta = LanguageDelta {
                language: language.to_string(),
                files_delta: stats2.map(|s| s.file_count as i64).unwrap_or(0) -
                            stats1.map(|s| s.file_count as i64).unwrap_or(0),
                total_lines_delta: stats2.map(|s| s.total_lines as i64).unwrap_or(0) -
                                  stats1.map(|s| s.total_lines as i64).unwrap_or(0),
                logical_lines_delta: stats2.map(|s| s.logical_lines as i64).unwrap_or(0) -
                                    stats1.map(|s| s.logical_lines as i64).unwrap_or(0),
                empty_lines_delta: stats2.map(|s| s.empty_lines as i64).unwrap_or(0) -
                                  stats1.map(|s| s.empty_lines as i64).unwrap_or(0),
            };
            
            if delta.files_delta != 0 || delta.total_lines_delta != 0 {
                language_deltas.push(delta);
            }
        }
        
        language_deltas.sort_by(|a, b| a.language.cmp(&b.language));
        
        ComparisonResult {
            report1_generated: report1.generated_at,
            report2_generated: report2.generated_at,
            global_delta,
            language_deltas,
            new_files,
            removed_files,
            modified_files,
        }
    }
}

/// REQ-7.3: Display comparison results in console
fn display_comparison(comparison: &ComparisonResult) -> Result<()> {
    println!("\n{}", "═".repeat(80).blue());
    println!("{}", "Report Comparison".bold().cyan());
    println!("{}", "═".repeat(80).blue());
    
    println!("\n{}", "Timestamps:".bold());
    println!("  Report 1: {}", comparison.report1_generated.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  Report 2: {}", comparison.report2_generated.format("%Y-%m-%d %H:%M:%S UTC"));
    
    // Global changes
    println!("\n{}", "Global Changes".bold().green());
    println!("{}", "─".repeat(40).green());
    
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Metric").style_spec("b"),
        Cell::new("Delta").style_spec("b"),
    ]));
    
    display_delta_row(&mut table, "Files", comparison.global_delta.files_delta);
    display_delta_row(&mut table, "Total Lines", comparison.global_delta.total_lines_delta);
    display_delta_row(&mut table, "Logical Lines", comparison.global_delta.logical_lines_delta);
    display_delta_row(&mut table, "Empty Lines", comparison.global_delta.empty_lines_delta);
    display_delta_row(&mut table, "Languages", comparison.global_delta.languages_delta);
    
    table.printstd();
    
    // Language changes
    if !comparison.language_deltas.is_empty() {
        println!("\n{}", "Language Changes".bold().green());
        println!("{}", "─".repeat(80).green());
        
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Language").style_spec("b"),
            Cell::new("Files Δ").style_spec("b"),
            Cell::new("Total Δ").style_spec("b"),
            Cell::new("Logical Δ").style_spec("b"),
            Cell::new("Empty Δ").style_spec("b"),
        ]));
        
        for lang in &comparison.language_deltas {
            table.add_row(Row::new(vec![
                Cell::new(&lang.language),
                Cell::new(&format_delta(lang.files_delta)),
                Cell::new(&format_delta(lang.total_lines_delta)),
                Cell::new(&format_delta(lang.logical_lines_delta)),
                Cell::new(&format_delta(lang.empty_lines_delta)),
            ]));
        }
        
        table.printstd();
    }
    
    // File changes summary
    if !comparison.new_files.is_empty() {
        println!("\n{}: {}", "New Files".bold().green(), comparison.new_files.len());
        if comparison.new_files.len() <= 10 {
            for file in &comparison.new_files {
                println!("  + {}", file.green());
            }
        }
    }
    
    if !comparison.removed_files.is_empty() {
        println!("\n{}: {}", "Removed Files".bold().red(), comparison.removed_files.len());
        if comparison.removed_files.len() <= 10 {
            for file in &comparison.removed_files {
                println!("  - {}", file.red());
            }
        }
    }
    
    if !comparison.modified_files.is_empty() {
        println!("\n{}: {}", "Modified Files".bold().yellow(), comparison.modified_files.len());
        if comparison.modified_files.len() <= 10 {
            for file in &comparison.modified_files {
                println!("  ~ {} ({})", 
                    file.path.yellow(),
                    format_delta(file.total_lines_delta));
            }
        }
    }
    
    Ok(())
}

fn display_delta_row(table: &mut Table, label: &str, delta: i64) {
    table.add_row(Row::new(vec![
        Cell::new(label),
        Cell::new(&format_delta(delta)),
    ]));
}

fn format_delta(delta: i64) -> String {
    if delta > 0 {
        format!("+{}", delta.to_formatted_string(&Locale::en)).green().to_string()
    } else if delta < 0 {
        delta.to_formatted_string(&Locale::en).red().to_string()
    } else {
        "0".to_string()
    }
}

/// REQ-7.4: Export comparison results
fn export_comparison(comparison: &ComparisonResult, path: &std::path::Path, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(comparison)
                .map_err(|e| SlocError::Serialization(e.to_string()))?;
            std::fs::write(path, json)?;
        }
        OutputFormat::Xml => {
            let xml = serde_xml_rs::to_string(comparison)
                .map_err(|e| SlocError::Serialization(e.to_string()))?;
            std::fs::write(path, xml)?;
        }
        OutputFormat::Csv => {
            // CSV export for comparison - simplified format
            let mut wtr = csv::Writer::from_path(path)
                .map_err(|e| SlocError::Serialization(e.to_string()))?;
            wtr.write_record(&["Type", "Name", "Files Delta", "Total Delta", "Logical Delta", "Empty Delta"])
                .map_err(|e| SlocError::Serialization(e.to_string()))?;
            
            // Global
            wtr.write_record(&[
                "Global", 
                "Summary",
                &comparison.global_delta.files_delta.to_string(),
                &comparison.global_delta.total_lines_delta.to_string(),
                &comparison.global_delta.logical_lines_delta.to_string(),
                &comparison.global_delta.empty_lines_delta.to_string(),
            ]).map_err(|e| SlocError::Serialization(e.to_string()))?;
            
            // Languages
            for lang in &comparison.language_deltas {
                wtr.write_record(&[
                    "Language",
                    &lang.language,
                    &lang.files_delta.to_string(),
                    &lang.total_lines_delta.to_string(),
                    &lang.logical_lines_delta.to_string(),
                    &lang.empty_lines_delta.to_string(),
                ]).map_err(|e| SlocError::Serialization(e.to_string()))?;
            }
            
            wtr.flush().map_err(|e| SlocError::Serialization(e.to_string()))?;
        }
    }
    
    Ok(())
}