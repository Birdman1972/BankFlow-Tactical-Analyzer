//! Application state management
//!
//! Manages loaded files and analysis results across Tauri commands.

use crate::models::{IpRecord, Transaction};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state holding loaded data and analysis results
#[derive(Default)]
pub struct AppState {
    /// Loaded transactions from File A
    pub transactions: Arc<RwLock<Vec<Transaction>>>,
    /// Loaded IP records from File B
    pub ip_records: Arc<RwLock<Vec<IpRecord>>>,
    /// Processed results ready for export
    pub results: Arc<RwLock<AnalysisState>>,
}

/// Analysis state holding processed data
#[derive(Default, Clone)]
pub struct AnalysisState {
    /// All processed transactions (summary)
    pub summary: Vec<Transaction>,
    /// Income transactions only
    pub income: Vec<Transaction>,
    /// Expense transactions only
    pub expense: Vec<Transaction>,
    /// Whether analysis has been run
    pub is_complete: bool,
    /// Total records processed
    pub total_records: usize,
    /// Records with matched IPs
    pub matched_count: usize,
    /// Records with multiple IPs
    pub multi_ip_count: usize,
    /// Whois queries made
    pub whois_queried: usize,
}

impl AppState {
    /// Create a new empty application state
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all loaded data
    pub async fn clear(&self) {
        let mut transactions = self.transactions.write().await;
        transactions.clear();

        let mut ip_records = self.ip_records.write().await;
        ip_records.clear();

        let mut results = self.results.write().await;
        *results = AnalysisState::default();
    }

    /// Check if File A is loaded
    pub async fn has_transactions(&self) -> bool {
        let transactions = self.transactions.read().await;
        !transactions.is_empty()
    }

    /// Check if File B is loaded
    pub async fn has_ip_records(&self) -> bool {
        let ip_records = self.ip_records.read().await;
        !ip_records.is_empty()
    }

    /// Check if analysis is complete
    pub async fn is_analysis_complete(&self) -> bool {
        let results = self.results.read().await;
        results.is_complete
    }
}
