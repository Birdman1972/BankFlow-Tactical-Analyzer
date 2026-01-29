//! File operations commands
//!
//! Handles loading Excel files and exporting analysis results.

use bankflow_core::{Exporter, Parser};
use crate::models::FileMetadata;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

/// Status of loaded files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadStatus {
    pub file_a_loaded: bool,
    pub file_b_loaded: bool,
    pub file_a_records: usize,
    pub file_b_records: usize,
    pub file_a_name: Option<String>,
    pub file_b_name: Option<String>,
}

/// Load and parse File A (Transaction file)
use std::collections::HashMap;

/// Load and parse File A (Transaction file)
#[tauri::command]
pub async fn load_file(path: String, mapping: Option<HashMap<String, String>>, state: State<'_, AppState>) -> Result<FileMetadata, String> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(format!("File not found: {}", path_buf.display()));
    }

    // Get file metadata
    let (row_count, column_count) = Parser::get_file_metadata(&path_buf)
        .map_err(|e| format!("Failed to read file metadata: {}", e))?;

    // Parse transactions
    let transactions = Parser::parse_transactions(&path_buf, mapping.as_ref())
        .map_err(|e| format!("Failed to parse transactions: {}", e))?;

    let _record_count = transactions.len();

    // Store in state
    {
        let mut tx_store = state.transactions.write().await;
        *tx_store = transactions;
    }

    // Clear previous analysis results since data changed
    {
        let mut results = state.results.write().await;
        results.is_complete = false;
    }

    Ok(FileMetadata {
        path: Some(path_buf.to_string_lossy().to_string()),
        filename: path_buf
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        row_count,
        column_count,
        file_type: path_buf
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_else(|| "xlsx".to_string()),
    })
}

/// Load and parse File B (IP log file)
#[tauri::command]
pub async fn load_ip_file(path: String, mapping: Option<HashMap<String, String>>, state: State<'_, AppState>) -> Result<FileMetadata, String> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(format!("File not found: {}", path_buf.display()));
    }

    // Get file metadata
    let (row_count, column_count) = Parser::get_file_metadata(&path_buf)
        .map_err(|e| format!("Failed to read file metadata: {}", e))?;

    // Parse IP records
    let ip_records = Parser::parse_ip_records(&path_buf, mapping.as_ref())
        .map_err(|e| format!("Failed to parse IP records: {}", e))?;

    // Store in state
    {
        let mut ip_store = state.ip_records.write().await;
        *ip_store = ip_records;
    }

    // Clear previous analysis results since data changed
    {
        let mut results = state.results.write().await;
        results.is_complete = false;
    }

    Ok(FileMetadata {
        path: Some(path_buf.to_string_lossy().to_string()),
        filename: path_buf
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        row_count,
        column_count,
        file_type: path_buf
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_else(|| "xlsx".to_string()),
    })
}

/// Get current load status
#[tauri::command]
pub async fn get_load_status(state: State<'_, AppState>) -> Result<LoadStatus, String> {
    let transactions = state.transactions.read().await;
    let ip_records = state.ip_records.read().await;

    Ok(LoadStatus {
        file_a_loaded: !transactions.is_empty(),
        file_b_loaded: !ip_records.is_empty(),
        file_a_records: transactions.len(),
        file_b_records: ip_records.len(),
        file_a_name: None, // Could store filename in state if needed
        file_b_name: None,
    })
}

/// Clear all loaded files and reset state
#[tauri::command]
pub async fn clear_files(state: State<'_, AppState>) -> Result<(), String> {
    state.clear().await;
    Ok(())
}

/// Export analysis results to Excel file
#[tauri::command]
pub async fn export_excel(output_path: String, state: State<'_, AppState>) -> Result<String, String> {
    let results = state.results.read().await;

    if !results.is_complete {
        return Err("No analysis results to export. Please run analysis first.".to_string());
    }

    let path_buf = PathBuf::from(&output_path);

    Exporter::export_to_excel(&path_buf, &results.summary, &results.income, &results.expense)
        .map_err(|e| format!("Failed to export Excel: {}", e))?;

    Ok(format!(
        "Successfully exported {} records to {}",
        results.summary.len(),
        output_path
    ))
}

use bankflow_core::batch::{BatchProcessor, BatchScanResult};

/// Get headers from an Excel file
#[tauri::command]
pub async fn get_file_headers(path: String) -> Result<Vec<String>, String> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(format!("File not found: {}", path_buf.display()));
    }

    Parser::get_file_headers(&path_buf)
        .map_err(|e| format!("Failed to read headers: {}", e))
}

/// Scan a folder for A/B pairs
#[tauri::command]
pub async fn scan_folder(path: String, max_depth: usize) -> Result<BatchScanResult, String> {
    let path_buf = PathBuf::from(&path);
    
    if !path_buf.exists() || !path_buf.is_dir() {
        return Err(format!("Folder not found or invalid: {}", path));
    }

    Ok(BatchProcessor::scan_folder(&path_buf, max_depth))
}
