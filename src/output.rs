// output.rs - Console and file output formatting
// Implements: REQ-5.1, REQ-5.2, REQ-5.3, REQ-5.4, REQ-6.1, REQ-6.2, REQ-6.3, REQ-6.7, REQ-6.8, REQ-1.1 (comment lines)

use crate::cli::{OutputFormat, SortMetric};
use crate::error::{Result, SlocError};
use crate::report::Report;
use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use prettytable::{Cell, Row, Table};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct ConsoleOutput {
    sort_metric: Option<SortMetric>,
}

impl ConsoleOutput {
    pub fn new(sort_metric: Option<SortMetric>) -> Self {
        Self { sort_metric }
    }

    /// REQ-5.1, REQ-5.2, REQ-5.3: Display summary tables
    pub fn display_summary(&self, report: &Report) -> Result<()> {
        println!("\n{}", "═".repeat(80).blue());
        println!("{}", "Source Lines of Code (SLOC) Report".bold().cyan());
        println!("{}", "═".repeat(80).blue());

        // Global summary
        self.display_global_summary(report);

        // Language summary (REQ-5.2)
        self.display_language_summary(report);

        // File details if not too many
        if report.files.len() <= 20 {
            self.display_file_details(report);
        } else {
            println!(
                "\n{}",
                format!(
                    "({} files processed, use --sort to see details)",
                    report.files.len()
                )
                .yellow()
            );
        }

        // REQ-3.5: Display unsupported files
        if !report.unsupported_files.is_empty() {
            println!("\n{}", "Unsupported Files (not counted):".bold().red());
            for path in &report.unsupported_files {
                println!("  - {}", path.display());
            }
        }

        // Display checksum if present
        if let Some(checksum) = &report.checksum {
            println!("\n{}: {}", "Checksum".bold(), checksum.green());
        }

        Ok(())
    }

    /// REQ-5.1: Display global summary
    fn display_global_summary(&self, report: &Report) {
        println!("\n{}", "Global Summary".bold().green());
        println!("{}", "─".repeat(40).green());

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Metric").style_spec("b"),
            Cell::new("Value").style_spec("b"),
        ]));

        // REQ-5.3: Format with thousands separators
        table.add_row(Row::new(vec![
            Cell::new("Total Files"),
            Cell::new(&report.summary.total_files.to_formatted_string(&Locale::en)),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Total Lines"),
            Cell::new(&report.summary.total_lines.to_formatted_string(&Locale::en)),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Logical Lines"),
            Cell::new(
                &report
                    .summary
                    .logical_lines
                    .to_formatted_string(&Locale::en),
            ),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Comment Lines"),
            Cell::new(&report.summary.comment_lines.to_formatted_string(&Locale::en)),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Empty Lines"),
            Cell::new(&report.summary.empty_lines.to_formatted_string(&Locale::en)),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Languages"),
            Cell::new(
                &report
                    .summary
                    .languages_count
                    .to_formatted_string(&Locale::en),
            ),
        ]));

        // REQ-5.3: Calculate and display percentages with 2 decimal places
        if report.summary.total_lines > 0 {
            let logical_pct =
                (report.summary.logical_lines as f64 / report.summary.total_lines as f64) * 100.0;
            let empty_pct =
                (report.summary.empty_lines as f64 / report.summary.total_lines as f64) * 100.0;

            table.add_row(Row::new(vec![
                Cell::new("Code Density"),
                Cell::new(&format!("{:.2}%", logical_pct)),
            ]));

            table.add_row(Row::new(vec![
                Cell::new("Empty Ratio"),
                Cell::new(&format!("{:.2}%", empty_pct)),
            ]));
        }

        table.printstd();
    }

    /// REQ-5.2: Display language summary
    fn display_language_summary(&self, report: &Report) {
        println!("\n{}", "Language Summary".bold().green());
        println!("{}", "─".repeat(80).green());

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Language").style_spec("b"),
            Cell::new("Files").style_spec("b"),
            Cell::new("Total").style_spec("b"),
            Cell::new("Logical").style_spec("b"),
            Cell::new("Comment").style_spec("b"),
            Cell::new("Empty").style_spec("b"),
            Cell::new("Density %").style_spec("b"),
        ]));

        let mut languages = report.languages.clone();

        // REQ-5.4: Sort by metric if specified
        match self.sort_metric {
            Some(SortMetric::Total) => languages.sort_by_key(|l| std::cmp::Reverse(l.total_lines)),
            Some(SortMetric::Logical) => {
                languages.sort_by_key(|l| std::cmp::Reverse(l.logical_lines))
            }
            Some(SortMetric::Empty) => languages.sort_by_key(|l| std::cmp::Reverse(l.empty_lines)),
            Some(SortMetric::Language) | Some(SortMetric::Name) | None => {
                languages.sort_by(|a, b| a.language.cmp(&b.language))
            }
        }

        for lang in &languages {
            let density = if lang.total_lines > 0 {
                (lang.logical_lines as f64 / lang.total_lines as f64) * 100.0
            } else {
                0.0
            };

            table.add_row(Row::new(vec![
                Cell::new(&lang.language),
                Cell::new(&lang.file_count.to_formatted_string(&Locale::en)),
                Cell::new(&lang.total_lines.to_formatted_string(&Locale::en)),
                Cell::new(&lang.logical_lines.to_formatted_string(&Locale::en)),
                Cell::new(&lang.comment_lines.to_formatted_string(&Locale::en)),
                Cell::new(&lang.empty_lines.to_formatted_string(&Locale::en)),
                Cell::new(&format!("{:.2}", density)),
            ]));
        }

        table.printstd();
    }

    /// Display file details
    fn display_file_details(&self, report: &Report) {
        println!("\n{}", "File Details".bold().green());
        println!("{}", "─".repeat(80).green());

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("File").style_spec("b"),
            Cell::new("Language").style_spec("b"),
            Cell::new("Total").style_spec("b"),
            Cell::new("Logical").style_spec("b"),
            Cell::new("Comment").style_spec("b"),
            Cell::new("Empty").style_spec("b"),
        ]));

        let mut files = report.files.clone();

        // REQ-5.4: Sort by metric
        match self.sort_metric {
            Some(SortMetric::Total) => files.sort_by_key(|f| std::cmp::Reverse(f.total_lines)),
            Some(SortMetric::Logical) => files.sort_by_key(|f| std::cmp::Reverse(f.logical_lines)),
            Some(SortMetric::Empty) => files.sort_by_key(|f| std::cmp::Reverse(f.empty_lines)),
            Some(SortMetric::Name) => files.sort_by(|a, b| a.path.cmp(&b.path)),
            Some(SortMetric::Language) => files.sort_by(|a, b| a.language.cmp(&b.language)),
            None => {}
        }

        for file in &files {
            let filename = file
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?");

            table.add_row(Row::new(vec![
                Cell::new(filename),
                Cell::new(&file.language),
                Cell::new(&file.total_lines.to_formatted_string(&Locale::en)),
                Cell::new(&file.logical_lines.to_formatted_string(&Locale::en)),
                Cell::new(&file.comment_lines.to_formatted_string(&Locale::en)),
                Cell::new(&file.empty_lines.to_formatted_string(&Locale::en)),
            ]));
        }

        table.printstd();
    }
}

pub struct ReportExporter;

impl ReportExporter {
    pub fn new() -> Self {
        Self
    }

    /// REQ-6.8: Export report to file
    pub fn export(&self, report: &Report, path: &Path, format: OutputFormat) -> Result<()> {
        match format {
            OutputFormat::Json => self.export_json(report, path),
            OutputFormat::Xml => self.export_xml(report, path),
            OutputFormat::Csv => self.export_csv(report, path),
        }
    }

    /// REQ-6.1: Export as JSON
    fn export_json(&self, report: &Report, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(report)
            .map_err(|e| SlocError::Serialization(e.to_string()))?;

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// REQ-6.2: Export as XML
    fn export_xml(&self, report: &Report, path: &Path) -> Result<()> {
        let xml =
            serde_xml_rs::to_string(report).map_err(|e| SlocError::Serialization(e.to_string()))?;

        let mut file = File::create(path)?;
        file.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// REQ-6.3: Export as CSV
    fn export_csv(&self, report: &Report, path: &Path) -> Result<()> {
        let mut wtr = csv::Writer::from_path(path).map_err(|e| {
            SlocError::Io(std::io::Error::other(
                e.to_string(),
            ))
        })?;

        // Write header
        wtr.write_record([
            "Path",
            "Language",
            "Total Lines",
            "Logical Lines",
            "Comment Lines",
            "Empty Lines",
        ])
        .map_err(|e| {
            SlocError::Io(std::io::Error::other(
                e.to_string(),
            ))
        })?;

        // Write file data
        for file in &report.files {
            wtr.write_record(&[
                file.path.to_string_lossy().to_string(),
                file.language.clone(),
                file.total_lines.to_string(),
                file.logical_lines.to_string(),
                file.comment_lines.to_string(),
                file.empty_lines.to_string(),
            ])
            .map_err(|e| {
                SlocError::Io(std::io::Error::other(
                    e.to_string(),
                ))
            })?;
        }

        // REQ-3.5: Add unsupported files section
        if !report.unsupported_files.is_empty() {
            wtr.write_record(&["--- Unsupported Files (not counted) ---"]).map_err(|e| SlocError::Io(std::io::Error::other(e.to_string())))?;
            for path in &report.unsupported_files {
                wtr.write_record(&[path.to_string_lossy().to_string()]).map_err(|e| SlocError::Io(std::io::Error::other(e.to_string())))?;
            }
        }

        wtr.flush().map_err(|e| {
            SlocError::Io(std::io::Error::other(
                e.to_string(),
            ))
        })?;
        Ok(())
    }
}
