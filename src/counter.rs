// counter.rs - Core line counting logic
// Implements:
//   REQ-1.1: Logical, comment, empty lines counting
//   REQ-2.1/2/3/4: File/dir input, wildcards, recursion, stdin
//   REQ-3.3: Custom language config
//   REQ-3.4: Language override
//   REQ-3.5: Unsupported files exclusion
//   REQ-4.1-4.5: Preprocessor, encoding, etc.
//   REQ-5.1-5.3: Console output, sorting, formatting
//   REQ-6.4: Report creation
//   REQ-6.8: Report export
//   REQ-9.2: Encoding detection
//   REQ-9.4: Parallel processing
//   REQ-9.5: Progress bar
//   REQ-9.7: Metrics logging

use crate::cli::CountArgs;
use crate::config::{AppConfig, MetricsLogger};
use crate::error::{Result, SlocError};
use crate::language::{CommentParser, LanguageDetector, LineType};
use crate::output::{ConsoleOutput, ReportExporter};
use human_format::Formatter;
use crate::report::{FileStats, Report};
use colored::Colorize;
use encoding_rs_io::DecodeReaderBytesBuilder;
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use walkdir::WalkDir;

pub fn execute_count(args: CountArgs) -> Result<()> {
    let start_time = Instant::now();

    // REQ-9.7: Initialize metrics logger with CLI overrides (metrics)
    let app_config = AppConfig::with_cli_overrides(
        args.config.as_deref(),
        args.enable_metrics,
        args.metrics_file.as_ref(),
    )?;

    let metrics_logger = Arc::new(MetricsLogger::new(&app_config.performance));

    // Initialize metrics session
    let args_summary = format!(
        "paths={}, recursive={}, threads={}, format={:?}",
        args.paths.len(),
        args.recursive,
        args.threads,
        args.format
    );
    metrics_logger.init_session("count", &args_summary);
    metrics_logger.log_system_info();
    metrics_logger.log_metric("operation_start", start_time.elapsed().as_secs_f64());

    let mut detector = LanguageDetector::new();

    // REQ-3.3: Load custom language config (custom language definitions)
    if let Some(config_path) = &args.config {
        let load_start = Instant::now();
        detector.load_from_config(config_path)?;
        metrics_logger.log_metric("config_load_time", load_start.elapsed().as_secs_f64());
    }

    // REQ-3.4: Apply language overrides (per estensione)
    for (ext, lang) in &args.language_override {
        detector.add_override(ext.clone(), lang.clone());
    }
    metrics_logger.log_metric(
        "language_overrides_count",
        args.language_override.len() as f64,
    );

    // REQ-2.1/2.2/2.3/2.4: Collect all file paths (input sources)
    let path_collection_start = Instant::now();
    let paths = collect_paths(&args)?;
    metrics_logger.log_metric(
        "path_collection_time",
        path_collection_start.elapsed().as_secs_f64(),
    );
    metrics_logger.log_metric("total_files_to_process", paths.len() as f64);

    // REQ-9.4: Set up parallel processing (thread pool)
    let thread_count = if args.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global()
            .map_err(|e| SlocError::Parse(e.to_string()))?;
        args.threads
    } else {
        rayon::current_num_threads()
    };
    metrics_logger.log_metric("thread_count", thread_count as f64);

    // REQ-9.5: Progress indicator (barra avanzamento)
    let progress = if !args.no_progress {
        let pb = ProgressBar::new(paths.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} | {per_sec}")
                .unwrap()
                .progress_chars("##-"),
        );
        Some(Arc::new(Mutex::new(pb)))
    } else {
        None
    };

    // REQ-1.1, REQ-9.4: Count lines in parallel (core counting)
    let detector = Arc::new(detector);
    let ignore_preprocessor = args.ignore_preprocessor;
    let metrics_clone = Arc::clone(&metrics_logger);

    let processing_start = Instant::now();
    let file_results: Vec<_> = paths.par_iter().map(|path| {
        let file_start = Instant::now();
        let result = count_file(path, &detector, ignore_preprocessor);

        // Log per-file metrics
        if let Ok(ref stats) = result {
            let file_time = file_start.elapsed().as_secs_f64();
            if file_time > 0.001 {
                metrics_clone.log_metric(
                    &format!(
                        "file_process_time_{}",
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                    ),
                    file_time,
                );
            }
            if stats.total_lines > 1000 {
                let throughput = stats.total_lines as f64 / file_time;
                metrics_clone.log_metric("large_file_throughput", throughput);
            }
        }

        if let Some(ref pb) = progress {
            let pb = pb.lock().unwrap();
            pb.inc(1);
            pb.set_message(format!("Processing: {}", path.display()));
        }

        match result {
            Ok(stats) => {
                if stats.language == "Unknown" {
                    Err(path.clone())
                } else {
                    Ok(stats)
                }
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", path.display(), e);
                metrics_clone.log_metric("file_errors", 1.0);
                // treat as unsupported for reporting
                Err(path.clone())
            }
        }
    }).collect();

    let (results, unsupported_files): (Vec<_>, Vec<_>) = file_results.into_iter().partition(|res| res.is_ok());
    let results: Vec<FileStats> = results.into_iter().map(|r| r.unwrap()).collect();
    let unsupported_files: Vec<PathBuf> = unsupported_files.into_iter().map(|e| e.unwrap_err()).collect();

    let processing_time = processing_start.elapsed();
    metrics_logger.log_metric("total_processing_time", processing_time.as_secs_f64());

    if let Some(ref pb) = progress {
        pb.lock().unwrap().finish_with_message("Complete!");
    }

    // Log processing statistics
    let total_lines: usize = results.iter().map(|r| r.total_lines).sum();
    let logical_lines: usize = results.iter().map(|r| r.logical_lines).sum();
    let comment_lines: usize = results.iter().map(|r| r.comment_lines).sum();
    let empty_lines: usize = results.iter().map(|r| r.empty_lines).sum();

    metrics_logger.log_metric("files_processed_successfully", results.len() as f64);
    metrics_logger.log_metric("total_lines_processed", total_lines as f64);
    metrics_logger.log_metric("logical_lines_processed", logical_lines as f64);
    metrics_logger.log_metric("comment_lines_processed", comment_lines as f64);
    metrics_logger.log_metric("empty_lines_processed", empty_lines as f64);

    if processing_time.as_secs_f64() > 0.0 {
        let throughput = total_lines as f64 / processing_time.as_secs_f64();
        metrics_logger.log_metric("overall_throughput_lines_per_sec", throughput);

        let files_per_sec = results.len() as f64 / processing_time.as_secs_f64();
        metrics_logger.log_metric("files_per_second", files_per_sec);
    }

    // REQ-6.4, REQ-6.5, REQ-6.6: Create report (aggregazione risultati)
    let report_creation_start = Instant::now();
    let mut report = Report::new(results, unsupported_files);
    metrics_logger.log_metric(
        "report_creation_time",
        report_creation_start.elapsed().as_secs_f64(),
    );

    // REQ-6.9: Add checksum if requested (opzionale)
    if args.checksum {
        let checksum_start = Instant::now();
        report.calculate_checksum();
        metrics_logger.log_metric(
            "checksum_calculation_time",
            checksum_start.elapsed().as_secs_f64(),
        );
    }

    // REQ-5.1, REQ-5.2, REQ-5.3: Console output (tabella, dettagli, unsupported)
    let console_start = Instant::now();
    let console = ConsoleOutput::new(args.sort, args.details);
    console.display_summary(&report)?;
    metrics_logger.log_metric("console_output_time", console_start.elapsed().as_secs_f64());

    // REQ-6.8: Export report if requested (json/xml/csv)
    if let Some(output_path) = args.output {
        if let Some(format) = args.format {
            let export_start = Instant::now();
            let exporter = ReportExporter::new();
            exporter.export(&report, &output_path, format)?;
            metrics_logger.log_metric("report_export_time", export_start.elapsed().as_secs_f64());
            println!("Report saved to: {}", output_path.display());
        }
    }

    // REQ-9.7: Log final completion metrics (fine operazione)
    let total_time = start_time.elapsed();
    metrics_logger.log_completion(report.summary.total_files, report.summary.total_lines);
    metrics_logger.log_metric("total_operation_time", total_time.as_secs_f64());

    // REQ-9.7: Log memory usage if possible (approximate)
    let memory_estimate = report.files.len() * std::mem::size_of::<FileStats>()
        + report.languages.len() * std::mem::size_of::<crate::report::LanguageStats>();
    metrics_logger.log_metric("memory_usage_estimate_bytes", memory_estimate as f64);

    // REQ-9.7: Output performance: lines/sec (always, regardless of params, human readable)
    let elapsed_secs = total_time.as_secs_f64();
    let total_lines = report.summary.total_lines as f64;
    let lines_per_sec = if elapsed_secs > 0.0 { total_lines / elapsed_secs } else { 0.0 };
    let thread_count = rayon::current_num_threads();
    let perf_str = Formatter::new().with_decimals(2).format(lines_per_sec);
    println!(
        "Performance: {} lines/sec ({} threads)",
        perf_str,
        thread_count
    );
    // Performance summary for large operations
    if total_time.as_secs() >= args.perf_summary_threshold || report.summary.total_files > 1000 {
        println!("\n{}", "Performance Summary:".bright_cyan());
        println!("  Total time: {:.2}s", total_time.as_secs_f64());
        println!("  Files processed: {}", report.summary.total_files);
        println!("  Lines processed: {}", report.summary.total_lines);
        if total_time.as_secs_f64() > 0.0 {
            println!(
                "  Throughput: {:.0} lines/sec",
                report.summary.total_lines as f64 / total_time.as_secs_f64()
            );
        }
        if metrics_logger.is_enabled() {
            println!("  Metrics logged to: {}", metrics_logger.file_path());
        }
    }

    Ok(())
}

/// REQ-2.1, REQ-2.2, REQ-2.3, REQ-2.4: Collect file paths from various sources
fn collect_paths(args: &CountArgs) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    // REQ-2.4: Read from stdin if requested
    if args.stdin {
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            let path = PathBuf::from(line.trim());
            if path.exists() {
                paths.push(path);
            } else {
                eprintln!("Warning: Path does not exist: {}", path.display());
            }
        }
    }

    // Process command-line paths
    for path_str in &args.paths {
        // REQ-2.2: Handle wildcards
        if path_str.contains('*') || path_str.contains('?') {
            for entry in glob(path_str).map_err(|e| SlocError::Parse(e.to_string()))? {
                match entry {
                    Ok(path) => {
                        if path.is_file() {
                            paths.push(path);
                        } else if path.is_dir() && args.recursive {
                            collect_directory_files(&path, &mut paths)?;
                        }
                    }
                    Err(e) => eprintln!("Warning: Glob error: {}", e),
                }
            }
        } else {
            let path = PathBuf::from(path_str);

            // REQ-2.5: Validate paths
            if !path.exists() {
                return Err(SlocError::FileNotFound { path });
            }

            if path.is_file() {
                paths.push(path);
            } else if path.is_dir() {
                // REQ-2.3: Recursive directory traversal
                if args.recursive {
                    collect_directory_files(&path, &mut paths)?;
                } else {
                    eprintln!(
                        "Warning: {} is a directory. Use -r for recursive traversal.",
                        path.display()
                    );
                }
            }
        }
    }

    // REQ-9.3: Ensure deterministic output
    paths.sort();
    paths.dedup();

    Ok(paths)
}

/// REQ-2.3: Recursively collect files from directory
fn collect_directory_files(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    for entry in WalkDir::new(dir).follow_links(true) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    paths.push(entry.path().to_path_buf());
                }
            }
            Err(e) => eprintln!("Warning: Error accessing {}: {}", dir.display(), e),
        }
    }
    Ok(())
}

/// REQ-4.1: Count lines in a single file
fn count_file(
    path: &Path,
    detector: &Arc<LanguageDetector>,
    ignore_preprocessor: bool,
) -> Result<FileStats> {
    // REQ-3.2: Detect language
    let language = detector.detect(path);
    let language_name = language
        .map(|l| l.name.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    // REQ-9.2: Handle different encodings
    let file = File::open(path)?;
    let reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_8))
        .build(file);
    let reader = BufReader::new(reader);

    let mut total_lines = 0;
    let mut logical_lines = 0;
    let mut comment_lines = 0;
    let mut empty_lines = 0;

    if let Some(lang) = language {
        let parser = CommentParser::new(lang.clone(), ignore_preprocessor);
        let mut in_multiline = false;
        let mut depth = 0;

        for line in reader.lines() {
            let line = line?;
            total_lines += 1;

            // REQ-4.2, REQ-4.3: Handle multi-line comments
            if parser.is_in_multiline_comment(&line, &mut in_multiline, &mut depth) {
                // Line is part of a multi-line comment
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    empty_lines += 1;
                }
            } else {
                // REQ-4.4: Parse line type
                match parser.parse_line(&line) {
                    LineType::Empty => empty_lines += 1,
                    LineType::Comment => comment_lines += 1,
                    LineType::Logical | LineType::Mixed => logical_lines += 1,
                }
            }
        }
    } else {
        // Unknown language - count non-empty lines as logical
        for line in reader.lines() {
            let line = line?;
            total_lines += 1;

            if line.trim().is_empty() {
                empty_lines += 1;
            } else {
                logical_lines += 1;
            }
        }
    }

    Ok(FileStats {
        path: path.to_path_buf(),
        language: language_name,
        total_lines,
        logical_lines,
        comment_lines,
        empty_lines,
    })
}
