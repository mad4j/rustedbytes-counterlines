// config.rs - Configuration management
// Implements: REQ-3.3, REQ-9.7

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Language configuration that can be loaded from TOML
/// REQ-3.3: Language definitions via configuration files
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub languages: HashMap<String, LanguageDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageDefinition {
    pub name: String,
    pub extensions: Vec<String>,
    pub single_line_comment: Vec<String>,
    pub multi_line_comment: Vec<MultiLineComment>,
    #[serde(default)]
    pub nested_comments: bool,
    pub preprocessor_prefix: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiLineComment {
    pub start: String,
    pub end: String,
}

/// Application configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub performance: PerformanceConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
}

/// REQ-9.7: Performance metrics configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceConfig {
    #[serde(default = "default_threads")]
    pub default_threads: usize,
    #[serde(default = "default_chunk_size")]
    pub chunk_size: usize,
    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,
    #[serde(default = "default_metrics_file")]
    pub metrics_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultsConfig {
    #[serde(default = "default_recursive")]
    pub recursive: bool,
    #[serde(default = "default_no_progress")]
    pub no_progress: bool,
    #[serde(default = "default_format")]
    pub output_format: String,
    #[serde(default = "default_output_file")]
    pub output_file: String, // base name (without extension) for auto-generated report files
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            default_threads: default_threads(),
            chunk_size: default_chunk_size(),
            enable_metrics: default_enable_metrics(),
            metrics_file: default_metrics_file(),
        }
    }
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            recursive: default_recursive(),
            no_progress: default_no_progress(),
            output_format: default_format(),
            output_file: default_output_file(),
        }
    }
}

fn default_threads() -> usize {
    0
}
fn default_chunk_size() -> usize {
    1000
}
fn default_enable_metrics() -> bool {
    false
}
fn default_metrics_file() -> String {
    "sloc_metrics.log".to_string()
}
fn default_recursive() -> bool {
    false
}
fn default_no_progress() -> bool {
    false
} // progress enabled by default
fn default_format() -> String {
    "json".to_string()
}
fn default_output_file() -> String { // new default base report name
    DEFAULT_OUTPUT_FILE_BASE.to_string()
}

/// Public constant for the default base name of auto-generated report files
pub const DEFAULT_OUTPUT_FILE_BASE: &str = "sloc-report";

impl AppConfig {
    pub fn from_file(path: &Path) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| crate::error::SlocError::InvalidConfig(e.to_string()))
    }

    pub fn default() -> Self {
        Self {
            performance: PerformanceConfig::default(),
            defaults: DefaultsConfig::default(),
        }
    }

    /// Create AppConfig with CLI overrides
    pub fn with_cli_overrides(
        config_path: Option<&Path>,
        enable_metrics: bool,
        metrics_file: Option<&PathBuf>,
    ) -> crate::error::Result<Self> {
        let mut config = if let Some(path) = config_path {
            Self::from_file(path).unwrap_or_else(|_| {
                eprintln!("Warning: Could not load config file, using defaults");
                Self::default()
            })
        } else {
            Self::default()
        };

        // Override with CLI arguments
        if enable_metrics {
            config.performance.enable_metrics = true;
        }

        if let Some(file_path) = metrics_file {
            config.performance.metrics_file = file_path.to_string_lossy().to_string();
        }

        Ok(config)
    }
}

/// REQ-9.7: Performance metrics logger
pub struct MetricsLogger {
    enabled: bool,
    start_time: std::time::Instant,
    file_path: String,
}

impl MetricsLogger {
    pub fn new(config: &PerformanceConfig) -> Self {
        Self {
            enabled: config.enable_metrics,
            start_time: std::time::Instant::now(),
            file_path: config.metrics_file.clone(),
        }
    }

    /// Create MetricsLogger from CLI arguments
    pub fn _from_cli(enable_metrics: bool, metrics_file: Option<&PathBuf>) -> Self {
        let file_path = metrics_file
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "sloc_metrics.log".to_string());

        Self {
            enabled: enable_metrics,
            start_time: std::time::Instant::now(),
            file_path,
        }
    }

    /// Initialize metrics with session info
    pub fn init_session(&self, operation: &str, args_summary: &str) {
        if !self.enabled {
            return;
        }

        self.log_raw_message(&format!(
            "\n=== SLOC Metrics Session Started ===\nOperation: {}\nTimestamp: {}\nArgs: {}\n",
            operation,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            args_summary
        ));
    }

    /// Log a raw message without timestamp prefix
    pub fn log_raw_message(&self, message: &str) {
        if !self.enabled {
            return;
        }

        if let Err(e) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(message.as_bytes())
            })
        {
            eprintln!("Failed to log message: {}", e);
        }
    }

    pub fn log_metric(&self, metric_name: &str, value: f64) {
        if !self.enabled {
            return;
        }

        let elapsed = self.start_time.elapsed().as_secs_f64();
        let log_entry = format!("[{:.3}s] {}: {:.3}\n", elapsed, metric_name, value);

        if let Err(e) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(log_entry.as_bytes())
            })
        {
            eprintln!("Failed to log metric: {}", e);
        }
    }

    /// Log a metric with additional context
    pub fn _log_metric_with_context(&self, metric_name: &str, value: f64, context: &str) {
        if !self.enabled {
            return;
        }

        let elapsed = self.start_time.elapsed().as_secs_f64();
        let log_entry = format!(
            "[{:.3}s] {} ({}): {:.3}\n",
            elapsed, metric_name, context, value
        );

        if let Err(e) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(log_entry.as_bytes())
            })
        {
            eprintln!("Failed to log metric: {}", e);
        }
    }

    /// Log system information
    pub fn log_system_info(&self) {
        if !self.enabled {
            return;
        }

        let cpu_count = num_cpus::get();
        let available_parallelism = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(cpu_count);

        self.log_metric("system_cpu_count", cpu_count as f64);
        self.log_metric("system_available_parallelism", available_parallelism as f64);

        // Log Rust version if available
        if let Some(version) = option_env!("CARGO_PKG_VERSION") {
            self.log_raw_message(&format!("Tool version: {}\n", version));
        }
    }

    pub fn log_completion(&self, files_processed: usize, total_lines: usize) {
        if !self.enabled {
            return;
        }

        let elapsed = self.start_time.elapsed();
        let throughput = if elapsed.as_secs() > 0 {
            total_lines as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        self.log_metric("total_files", files_processed as f64);
        self.log_metric("total_lines", total_lines as f64);
        self.log_metric("elapsed_seconds", elapsed.as_secs_f64());
        self.log_metric("lines_per_second", throughput);

        // Log session end
        self.log_raw_message("=== Session Completed ===\n\n");
    }

    /// Check if metrics logging is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the metrics file path
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}
