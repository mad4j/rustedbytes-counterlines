// output.rs - Console and file output formatting
// Implements:
//   REQ-1.1: Comment lines in output
//   REQ-5.1: Global summary table
//   REQ-5.2: Language summary table
//   REQ-5.3: Number formatting, alignment
//   REQ-5.4: Sorting
//   REQ-6.1/2/3: Export JSON/XML/CSV
//   REQ-6.7: Output options
//   REQ-6.8: Output path

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
    details: bool,
}

impl ConsoleOutput {
    pub fn new(sort_metric: Option<SortMetric>, details: bool) -> Self {
        Self {
            sort_metric,
            details,
        }
    }

    /// REQ-5.1, REQ-5.2, REQ-5.3: Display summary tables (global, language, file, unsupported)
    pub fn display_summary(&self, report: &Report) -> Result<()> {
        println!("\n{}", "═".repeat(80).blue());
        println!("{}", "Source Lines of Code (SLOC) Report".bold().cyan());
        println!("{}", "═".repeat(80).blue());

        // Global summary
        self.display_global_summary(report);

        // Language summary (REQ-5.2)
        self.display_language_summary(report);

        // File details and unsupported files only if --details is set
        if self.details {
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
            // REQ-3.5.3: Display unsupported files separately
            if !report.unsupported_files.is_empty() {
                println!("\n{}", "Unsupported Files (not counted):".bold().red());
                for path in &report.unsupported_files {
                    println!("  - {}", path.display());
                }
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
            Cell::new("Value").style_spec("br"),
            Cell::new("% ").style_spec("br"),
        ]));

        let total_lines = report.summary.total_lines as f64;

        // Total Files
        table.add_row(Row::new(vec![
            Cell::new("Total Files"),
            Cell::new(&report.summary.total_files.to_formatted_string(&Locale::en)).style_spec("r"),
            Cell::new("").style_spec("r"),
        ]));
        // Unsupported Files
        table.add_row(Row::new(vec![
            Cell::new("Unsupported Files"),
            Cell::new(
                &report
                    .summary
                    .unsupported_files
                    .to_formatted_string(&Locale::en),
            )
            .style_spec("r"),
            Cell::new("").style_spec("r"),
        ]));
        // Total Lines
        table.add_row(Row::new(vec![
            Cell::new("Total Lines"),
            Cell::new(&report.summary.total_lines.to_formatted_string(&Locale::en)).style_spec("r"),
            Cell::new("100.00 %").style_spec("r"),
        ]));
        // Logical Lines
        let logical_pct = if total_lines > 0.0 {
            (report.summary.logical_lines as f64 / total_lines) * 100.0
        } else {
            0.0
        };
        table.add_row(Row::new(vec![
            Cell::new("Logical Lines"),
            Cell::new(
                &report
                    .summary
                    .logical_lines
                    .to_formatted_string(&Locale::en),
            )
            .style_spec("r"),
            Cell::new(&format!("{:.2} %", logical_pct)).style_spec("r"),
        ]));
        // Comment Lines
        let comment_pct = if total_lines > 0.0 {
            (report.summary.comment_lines as f64 / total_lines) * 100.0
        } else {
            0.0
        };
        table.add_row(Row::new(vec![
            Cell::new("Comment Lines"),
            Cell::new(
                &report
                    .summary
                    .comment_lines
                    .to_formatted_string(&Locale::en),
            )
            .style_spec("r"),
            Cell::new(&format!("{:.2} %", comment_pct)).style_spec("r"),
        ]));
        // Empty Lines
        let empty_pct = if total_lines > 0.0 {
            (report.summary.empty_lines as f64 / total_lines) * 100.0
        } else {
            0.0
        };
        table.add_row(Row::new(vec![
            Cell::new("Empty Lines"),
            Cell::new(&report.summary.empty_lines.to_formatted_string(&Locale::en)).style_spec("r"),
            Cell::new(&format!("{:.2} %", empty_pct)).style_spec("r"),
        ]));
        // Languages
        table.add_row(Row::new(vec![
            Cell::new("Languages"),
            Cell::new(
                &report
                    .summary
                    .languages_count
                    .to_formatted_string(&Locale::en),
            )
            .style_spec("r"),
            Cell::new("").style_spec("r"),
        ]));

        table.printstd();
    }

    /// REQ-5.2: Display language summary
    fn display_language_summary(&self, report: &Report) {
        println!("\n{}", "Language Summary".bold().green());
        println!("{}", "─".repeat(80).green());

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Language").style_spec("b"),
            Cell::new("Files").style_spec("br"),
            Cell::new("Total").style_spec("br"),
            Cell::new("Logical").style_spec("br"),
            Cell::new("Comment").style_spec("br"),
            Cell::new("Empty").style_spec("br"),
            Cell::new("Density %").style_spec("br"),
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
                Cell::new(&lang.file_count.to_formatted_string(&Locale::en)).style_spec("r"),
                Cell::new(&lang.total_lines.to_formatted_string(&Locale::en)).style_spec("r"),
                Cell::new(&lang.logical_lines.to_formatted_string(&Locale::en)).style_spec("r"),
                Cell::new(&lang.comment_lines.to_formatted_string(&Locale::en)).style_spec("r"),
                Cell::new(&lang.empty_lines.to_formatted_string(&Locale::en)).style_spec("r"),
                Cell::new(&format!("{:.2} %", density)).style_spec("r"),
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
            Cell::new("Total").style_spec("br"),
            Cell::new("Logical").style_spec("br"),
            Cell::new("Comment").style_spec("br"),
            Cell::new("Empty").style_spec("br"),
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
                Cell::new(&file.total_lines.to_formatted_string(&Locale::en)).style_spec("r"),
                Cell::new(&file.logical_lines.to_formatted_string(&Locale::en)).style_spec("r"),
                Cell::new(&file.comment_lines.to_formatted_string(&Locale::en)).style_spec("r"),
                Cell::new(&file.empty_lines.to_formatted_string(&Locale::en)).style_spec("r"),
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
        // Create a simplified XML-compatible version of the report
        let xml_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<report>
  <reportFormatVersion>{}</reportFormatVersion>
  <generatedAt>{}</generatedAt>
  <summary>
    <totalFiles>{}</totalFiles>
    <totalLines>{}</totalLines>
    <logicalLines>{}</logicalLines>
    <commentLines>{}</commentLines>
    <emptyLines>{}</emptyLines>
    <languagesCount>{}</languagesCount>
    <unsupportedFiles>{}</unsupportedFiles>
  </summary>
  <files>
{}</files>
  <languages>
{}</languages>
  <unsupportedFiles>
{}</unsupportedFiles>
{}</report>"#,
            report.report_format_version,
            report.generated_at.to_rfc3339(),
            report.summary.total_files,
            report.summary.total_lines,
            report.summary.logical_lines,
            report.summary.comment_lines,
            report.summary.empty_lines,
            report.summary.languages_count,
            report.summary.unsupported_files,
            self.format_files_xml(&report.files),
            self.format_languages_xml(&report.languages),
            self.format_unsupported_files_xml(&report.unsupported_files),
            self.format_checksum_xml(&report.checksum)
        );

        let mut file = File::create(path)?;
        file.write_all(xml_content.as_bytes())?;
        Ok(())
    }

    /// Format files section for XML
    fn format_files_xml(&self, files: &[crate::report::FileStats]) -> String {
        files
            .iter()
            .map(|f| {
                format!(
                    r#"    <file>
      <path>{}</path>
      <language>{}</language>
      <totalLines>{}</totalLines>
      <logicalLines>{}</logicalLines>
      <commentLines>{}</commentLines>
      <emptyLines>{}</emptyLines>
    </file>"#,
                    self.escape_xml(&f.path.to_string_lossy()),
                    self.escape_xml(&f.language),
                    f.total_lines,
                    f.logical_lines,
                    f.comment_lines,
                    f.empty_lines
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Format languages section for XML
    fn format_languages_xml(&self, languages: &[crate::report::LanguageStats]) -> String {
        languages
            .iter()
            .map(|l| {
                format!(
                    r#"    <language>
      <name>{}</name>
      <fileCount>{}</fileCount>
      <totalLines>{}</totalLines>
      <logicalLines>{}</logicalLines>
      <commentLines>{}</commentLines>
      <emptyLines>{}</emptyLines>
    </language>"#,
                    self.escape_xml(&l.language),
                    l.file_count,
                    l.total_lines,
                    l.logical_lines,
                    l.comment_lines,
                    l.empty_lines
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Format unsupported files section for XML
    fn format_unsupported_files_xml(&self, unsupported_files: &[std::path::PathBuf]) -> String {
        unsupported_files
            .iter()
            .map(|f| {
                format!(
                    r#"    <unsupportedFile>{}</unsupportedFile>"#,
                    self.escape_xml(&f.to_string_lossy())
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Format checksum section for XML
    fn format_checksum_xml(&self, checksum: &Option<String>) -> String {
        match checksum {
            Some(checksum) => format!("\n  <checksum>{}</checksum>", self.escape_xml(checksum)),
            None => String::new(),
        }
    }

    /// Escape XML special characters
    fn escape_xml(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    /// REQ-6.3: Export as CSV
    fn export_csv(&self, report: &Report, path: &Path) -> Result<()> {
        let mut wtr = csv::Writer::from_path(path)
            .map_err(|e| SlocError::Io(std::io::Error::other(e.to_string())))?;

        // Write header
        wtr.write_record([
            "Path",
            "Language",
            "Total Lines",
            "Logical Lines",
            "Comment Lines",
            "Empty Lines",
        ])
        .map_err(|e| SlocError::Io(std::io::Error::other(e.to_string())))?;

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
            .map_err(|e| SlocError::Io(std::io::Error::other(e.to_string())))?;
        }

        // REQ-3.5: Add unsupported files section
        if !report.unsupported_files.is_empty() {
            wtr.write_record(["--- Unsupported Files (not counted) ---"])
                .map_err(|e| SlocError::Io(std::io::Error::other(e.to_string())))?;
            for path in &report.unsupported_files {
                wtr.write_record(&[path.to_string_lossy().to_string()])
                    .map_err(|e| SlocError::Io(std::io::Error::other(e.to_string())))?;
            }
        }

        wtr.flush()
            .map_err(|e| SlocError::Io(std::io::Error::other(e.to_string())))?;
        Ok(())
    }
}
