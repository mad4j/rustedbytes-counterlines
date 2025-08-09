// error.rs - Error handling definitions
// Implements: REQ-2.5, REQ-9.3

use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum SlocError {
    /// REQ-2.5: Error messages for invalid/inaccessible paths
    #[error("Invalid path: {path}")]
    InvalidPath { path: PathBuf },
    
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    #[error("Invalid report format: {0}")]
    InvalidReportFormat(String),
    
    #[error("Encoding error: {0}")]
    Encoding(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, SlocError>;