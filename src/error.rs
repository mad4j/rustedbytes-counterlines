// error.rs - Error handling definitions
// Implements: REQ-2.5, REQ-9.3

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlocError {
    /// REQ-2.5: Error messages for invalid/inaccessible paths
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Parse error: {0}")]
    Parse(String),

    // Varianti che potrebbero essere usate in futuro
    // Usa l'attributo allow per silenziare i warning
    #[allow(dead_code)]
    #[error("Invalid path: {path}")]
    InvalidPath { path: PathBuf },

    #[allow(dead_code)]
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },

    #[allow(dead_code)]
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),

    #[allow(dead_code)]
    #[error("Invalid report format: {0}")]
    InvalidReportFormat(String),

    #[allow(dead_code)]
    #[error("Encoding error: {0}")]
    Encoding(String),
}

pub type Result<T> = std::result::Result<T, SlocError>;
