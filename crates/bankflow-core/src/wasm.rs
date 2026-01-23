//! WASM bindings using wasm-bindgen
//!
//! Provides JavaScript-friendly API for the browser.

#![cfg(feature = "wasm")]

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use crate::{Exporter, IpMatcher, Parser, Processor};

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "wasm")]
    console_error_panic_hook::set_once();
}

/// Analyze Excel files and return results as JSON
#[wasm_bindgen]
pub fn analyze(
    file_a_bytes: &[u8],
    file_a_name: &str,
    file_b_bytes: &[u8],
    file_b_name: &str,
    hide_sensitive: bool,
    ip_cross_reference: bool,
) -> Result<JsValue, JsError> {
    // Parse files
    let (mut transactions, meta_a) = Parser::parse_transactions_from_bytes(file_a_bytes, file_a_name)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let (ip_records, meta_b) = Parser::parse_ip_records_from_bytes(file_b_bytes, file_b_name)
        .map_err(|e| JsError::new(&e.to_string()))?;

    // Preprocess
    let processor = Processor::new(hide_sensitive);
    processor.process(&mut transactions);

    // IP matching
    if ip_cross_reference {
        let matcher = IpMatcher::with_default_window(&ip_records);
        matcher.match_all(&mut transactions);
    }

    // Split income/expense
    let (income, expense) = Processor::split_income_expense(&transactions);

    // Build result
    let result = serde_json::json!({
        "fileA": meta_a,
        "fileB": meta_b,
        "totalRecords": transactions.len(),
        "incomeCount": income.len(),
        "expenseCount": expense.len(),
        "transactions": transactions,
        "income": income,
        "expense": expense,
    });

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}

/// Export analysis results to Excel bytes
#[wasm_bindgen]
pub fn export_excel(transactions_json: &str) -> Result<Vec<u8>, JsError> {
    #[derive(serde::Deserialize)]
    struct ExportInput {
        transactions: Vec<crate::Transaction>,
        income: Vec<crate::Transaction>,
        expense: Vec<crate::Transaction>,
    }

    let input: ExportInput = serde_json::from_str(transactions_json)
        .map_err(|e| JsError::new(&format!("Invalid JSON: {}", e)))?;

    Exporter::export_to_bytes(&input.transactions, &input.income, &input.expense)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Get file metadata without full parsing
#[wasm_bindgen]
pub fn get_file_info(bytes: &[u8], filename: &str) -> Result<JsValue, JsError> {
    let metadata = Parser::get_metadata_from_bytes(bytes, filename)
        .map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&metadata).map_err(|e| JsError::new(&e.to_string()))
}
