//! Core business logic modules
//!
//! Re-exports from bankflow-core for shared functionality.
//! Tauri-specific modules (whois) are kept here.

// Re-export core modules from bankflow-core
pub use bankflow_core::exporter;
pub use bankflow_core::matcher;
pub use bankflow_core::parser;
pub use bankflow_core::processor;

// Tauri-specific modules (requires network access, not available in WASM)
pub mod whois;
