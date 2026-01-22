//! Data models for the application
//!
//! Re-exports core types from bankflow-core and adds Tauri-specific extensions.

// Re-export core types from bankflow-core
pub use bankflow_core::{
    excel_date_to_datetime, AnalysisResult, AnalysisSettings, FileMetadata, IpRecord, Transaction,
    WhoisResult,
};

// Note: bankflow-core uses NaiveDateTime for WASM compatibility.
// For display purposes, convert to local timezone as needed.
