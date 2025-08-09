// report.rs - Report structure and generation
// Implements: REQ-6.1, REQ-6.2, REQ-6.3, REQ-6.4, REQ-6.5, REQ-6.6, REQ-6.7, REQ-6.9

use crate::cli::ReportArgs;
use crate::counter;
use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;

/// REQ-6.6: Report format version
const REPORT_FORMAT_VERSION: &str = "1.0";

/// REQ-6.4: File statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub path: PathBuf,
    pub language: String,
    pub total_lines: usize,
    pub logical_lines: usize,
    pub empty_lines: usize,
}

/// REQ-6.4: Language summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageStats {
    pub language: String,
    pub file_count: usize,
    pub total_lines: usize,
    pub logical_lines: usize,
    pub empty_lines: usize,
}

/// REQ-6.4, REQ-6.5, REQ-6.6, REQ-6.7: Report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// REQ-6.6: Report format version
    pub report_format_version: String,
    
    /// REQ-6.5: Generation timestamp (RFC 3339 / ISO 8601)
    pub generated_at: DateTime<Utc>,
    
    /// REQ-6.4: Per-file statistics
    pub files: Vec<FileStats>,
    
    /// Language summaries
    pub languages: Vec<LanguageStats>,
    
    /// Global summary
    pub summary: GlobalSummary,
    
    /// REQ-6.9: Optional checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSummary {
    pub total_files: usize,
    pub total_lines: usize,
    pub logical_lines: usize,
    pub empty_lines: usize,
    pub languages_count: usize,
}

impl Report {
    /// Create a new report from file statistics
    pub fn new(files: Vec<FileStats>) -> Self {
        let languages = Self::calculate_language_stats(&files);
        let summary = Self::calculate_summary(&files, &languages);
        
        Report {
            report_format_version: REPORT_FORMAT_VERSION.to_string(),
            generated_at: Utc::now(),
            files,
            languages,
            summary,
            checksum: None,
        }
    }
    
    /// Calculate language statistics
    fn calculate_language_stats(files: &[FileStats]) -> Vec<LanguageStats> {
        let mut lang_map: HashMap<String, LanguageStats> = HashMap::new();
        
        for file in files {
            let entry = lang_map.entry(file.language.clone()).or_insert(LanguageStats {
                language: file.language.clone(),
                file_count: 0,
                total_lines: 0,
                logical_lines: 0,
                empty_lines: 0,
            });
            
            entry.file_count += 1;
            entry.total_lines += file.total_lines;
            entry.logical_lines += file.logical_lines;
            entry.empty_lines += file.empty_lines;
        }
        
        let mut languages: Vec<LanguageStats> = lang_map.into_values().collect();
        // REQ-9.3: Deterministic output
        languages.sort_by(|a, b| a.language.cmp(&b.language));
        languages
    }
    
    /// Calculate global summary
    fn calculate_summary(files: &[FileStats], languages: &[LanguageStats]) -> GlobalSummary {
        GlobalSummary {
            total_files: files.len(),
            total_lines: files.iter().map(|f| f.total_lines).sum(),
            logical_lines: files.iter().map(|f| f.logical_lines).sum(),
            empty_lines: files.iter().map(|f| f.empty_lines).sum(),
            languages_count: languages.len(),
        }
    }
    
    /// REQ-6.9: Calculate SHA256 checksum
    pub fn calculate_checksum(&mut self) {
        let mut hasher = Sha256::new();
        
        // Hash all file stats in deterministic order
        let mut sorted_files = self.files.clone();
        sorted_files.sort_by(|a, b| a.path.cmp(&b.path));
        
        for file in &sorted_files {
            hasher.update(file.path.to_string_lossy().as_bytes());
            hasher.update(file.language.as_bytes());
            hasher.update(file.total_lines.to_string().as_bytes());
            hasher.update(file.logical_lines.to_string().as_bytes());
            hasher.update(file.empty_lines.to_string().as_bytes());
        }
        
        let result = hasher.finalize();
        self.checksum = Some(hex::encode(result));
    }
    
    /// Load report from file
    pub fn from_file(path: &PathBuf, format: crate::cli::OutputFormat) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        
        let report = match format {
            crate::cli::OutputFormat::Json => {
                serde_json::from_str(&content)
                    .map_err(|e| crate::error::SlocError::Deserialization(e.to_string()))?
            }
            crate::cli::OutputFormat::Xml => {
                serde_xml_rs::from_str(&content)
                    .map_err(|e| crate::error::SlocError::Deserialization(e.to_string()))?
            }
            crate::cli::OutputFormat::Csv => {
                // CSV requires special handling
                Self::from_csv(&content)?
            }
        };
        
        Ok(report)
    }
    
    /// Load report from CSV
    fn from_csv(content: &str) -> Result<Self> {
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        let mut files = Vec::new();
        
        for result in reader.deserialize() {
            let file: FileStats = result
                .map_err(|e| crate::error::SlocError::Deserialization(e.to_string()))?;
            files.push(file);
        }
        
        Ok(Self::new(files))
    }
}

/// Execute report generation command
pub fn execute_report(args: ReportArgs) -> Result<()> {
    // Convert ReportArgs to CountArgs for reuse
    let count_args = crate::cli::CountArgs {
        paths: args.paths,
        recursive: args.recursive,
        stdin: false,
        format: Some(args.format),
        output: Some(args.output.clone()),
        sort: None,
        language_override: vec![],
        config: args.config,
        no_progress: false,
        threads: args.threads,
        checksum: args.checksum,
        ignore_preprocessor: false,
    };
    
    // Reuse count logic
    counter::execute_count(count_args)?;
    
    println!("Report generated successfully: {}", args.output.display());
    Ok(())
}