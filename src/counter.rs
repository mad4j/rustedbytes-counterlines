// counter.rs - Core line counting logic
// Implements: REQ-1.1, REQ-2.1, REQ-2.2, REQ-2.3, REQ-2.4, REQ-4.1, REQ-4.2, REQ-4.3, REQ-4.4, REQ-4.5, REQ-9.2, REQ-9.4, REQ-9.5

use crate::cli::CountArgs;
use crate::error::{Result, SlocError};
use crate::language::{CommentParser,LanguageDetector, LineType};
use crate::output::{ConsoleOutput, ReportExporter};
use crate::report::{FileStats, Report};
use encoding_rs_io::DecodeReaderBytesBuilder;
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

pub fn execute_count(args: CountArgs) -> Result<()> {
    let mut detector = LanguageDetector::new();
    
    // REQ-3.3: Load custom language config
    if let Some(config_path) = &args.config {
        detector.load_from_config(config_path)?;
    }
    
    // REQ-3.4: Apply language overrides
    for (ext, lang) in &args.language_override {
        detector.add_override(ext.clone(), lang.clone());
    }
    
    // Collect all file paths
    let paths = collect_paths(&args)?;
    
    // REQ-9.4: Set up parallel processing
    if args.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global()
            .map_err(|e| SlocError::Parse(e.to_string()))?;
    }
    
    // REQ-9.5: Progress indicator
    let progress = if args.progress {
        let pb = ProgressBar::new(paths.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        Some(Arc::new(Mutex::new(pb)))
    } else {
        None
    };
    
    // Count lines in parallel (REQ-9.4)
    let detector = Arc::new(detector);
    let ignore_preprocessor = args.ignore_preprocessor;
    
    let results: Vec<FileStats> = paths
        .par_iter()
        .filter_map(|path| {
            let result = count_file(path, &detector, ignore_preprocessor);
            
            if let Some(ref pb) = progress {
                let pb = pb.lock().unwrap();
                pb.inc(1);
                pb.set_message(format!("Processing: {}", path.display()));
            }
            
            match result {
                Ok(stats) => Some(stats),
                Err(e) => {
                    eprintln!("Error processing {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect();
    
    if let Some(ref pb) = progress {
        pb.lock().unwrap().finish_with_message("Complete!");
    }
    
    // Create report (REQ-6.4, REQ-6.5, REQ-6.6)
    let mut report = Report::new(results);
    
    // REQ-6.9: Add checksum if requested
    if args.checksum {
        report.calculate_checksum();
    }
    
    // REQ-5.1, REQ-5.2, REQ-5.3: Console output
    let console = ConsoleOutput::new(args.sort);
    console.display_summary(&report)?;
    
    // REQ-6.8: Export report if requested
    if let Some(output_path) = args.output {
        if let Some(format) = args.format {
            let exporter = ReportExporter::new();
            exporter.export(&report, &output_path, format)?;
            println!("Report saved to: {}", output_path.display());
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
                    eprintln!("Warning: {} is a directory. Use -r for recursive traversal.", path.display());
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
fn count_file(path: &Path, detector: &Arc<LanguageDetector>, ignore_preprocessor: bool) -> Result<FileStats> {
    // REQ-3.2: Detect language
    let language = detector.detect(path);
    let language_name = language.map(|l| l.name.clone()).unwrap_or_else(|| "Unknown".to_string());
    
    // REQ-9.2: Handle different encodings
    let file = File::open(path)?;
    let reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_8))
        .build(file);
    let reader = BufReader::new(reader);
    
    let mut total_lines = 0;
    let mut logical_lines = 0;
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
                    LineType::Comment => {}, // Comment but not empty
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
        empty_lines,
    })
}