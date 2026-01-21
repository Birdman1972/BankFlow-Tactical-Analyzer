//! BankFlow Core - Shared analysis engine for desktop and WASM
//!
//! This crate provides the core business logic that can be compiled
//! for both native (Tauri desktop) and WebAssembly targets.

pub mod error;
pub mod exporter;
pub mod matcher;
pub mod models;
pub mod parser;
pub mod processor;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use error::CoreError;
pub use exporter::Exporter;
pub use matcher::{IpMatcher, MatchStats, TimeWindow};
pub use models::{
    AnalysisResult, AnalysisSettings, FileMetadata, IpRecord, Transaction, WhoisResult,
};
pub use parser::Parser;
pub use processor::{ProcessingStats, Processor};
