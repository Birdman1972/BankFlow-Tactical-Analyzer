//! Error types for the core library

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Excel parse error: {0}")]
    ExcelParseError(String),

    #[error("Export error: {0}")]
    ExportError(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for CoreError {
    fn from(e: std::io::Error) -> Self {
        CoreError::IoError(e.to_string())
    }
}
