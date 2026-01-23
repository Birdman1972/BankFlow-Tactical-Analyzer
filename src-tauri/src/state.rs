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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        // Default state should have empty vectors
        assert!(state.transactions.try_read().is_ok());
        assert!(state.ip_records.try_read().is_ok());
        assert!(state.results.try_read().is_ok());
    }

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert!(state.transactions.try_read().is_ok());
    }

    #[test]
    fn test_analysis_state_default() {
        let state = AnalysisState::default();
        assert!(state.summary.is_empty());
        assert!(state.income.is_empty());
        assert!(state.expense.is_empty());
        assert!(!state.is_complete);
        assert_eq!(state.total_records, 0);
        assert_eq!(state.matched_count, 0);
        assert_eq!(state.multi_ip_count, 0);
        assert_eq!(state.whois_queried, 0);
    }

    #[test]
    fn test_analysis_state_clone() {
        let state = AnalysisState {
            summary: vec![],
            income: vec![],
            expense: vec![],
            is_complete: true,
            total_records: 100,
            matched_count: 80,
            multi_ip_count: 5,
            whois_queried: 10,
        };

        let cloned = state.clone();
        assert_eq!(cloned.is_complete, state.is_complete);
        assert_eq!(cloned.total_records, state.total_records);
        assert_eq!(cloned.matched_count, state.matched_count);
    }

    #[tokio::test]
    async fn test_app_state_has_transactions_empty() {
        let state = AppState::new();
        assert!(!state.has_transactions().await);
    }

    #[tokio::test]
    async fn test_app_state_has_transactions_with_data() {
        let state = AppState::new();

        // Add a transaction
        {
            let mut transactions = state.transactions.write().await;
            transactions.push(Transaction {
                datetime: None,
                timestamp: "2024-01-15 10:30:00".to_string(),
                account: "ACC001".to_string(),
                income: Some(1000.0),
                expense: None,
                matched_ip: None,
                ip_country: None,
                ip_isp: None,
                raw_columns: vec![],
                row_index: 1,
            });
        }

        assert!(state.has_transactions().await);
    }

    #[tokio::test]
    async fn test_app_state_has_ip_records() {
        let state = AppState::new();
        assert!(!state.has_ip_records().await);

        // Add an IP record
        {
            let mut ip_records = state.ip_records.write().await;
            ip_records.push(IpRecord::new(
                "2024-01-15 10:30:00".to_string(),
                "ACC001".to_string(),
                "192.168.1.1".to_string(),
                1,
            ));
        }

        assert!(state.has_ip_records().await);
    }

    #[tokio::test]
    async fn test_app_state_is_analysis_complete() {
        let state = AppState::new();
        assert!(!state.is_analysis_complete().await);

        // Mark analysis as complete
        {
            let mut results = state.results.write().await;
            results.is_complete = true;
        }

        assert!(state.is_analysis_complete().await);
    }

    #[tokio::test]
    async fn test_app_state_clear() {
        let state = AppState::new();

        // Add data
        {
            let mut transactions = state.transactions.write().await;
            transactions.push(Transaction {
                datetime: None,
                timestamp: "2024-01-15 10:30:00".to_string(),
                account: "ACC001".to_string(),
                income: Some(1000.0),
                expense: None,
                matched_ip: None,
                ip_country: None,
                ip_isp: None,
                raw_columns: vec![],
                row_index: 1,
            });
        }
        {
            let mut ip_records = state.ip_records.write().await;
            ip_records.push(IpRecord::new(
                "2024-01-15 10:30:00".to_string(),
                "ACC001".to_string(),
                "192.168.1.1".to_string(),
                1,
            ));
        }
        {
            let mut results = state.results.write().await;
            results.is_complete = true;
            results.total_records = 100;
        }

        // Verify data exists
        assert!(state.has_transactions().await);
        assert!(state.has_ip_records().await);
        assert!(state.is_analysis_complete().await);

        // Clear
        state.clear().await;

        // Verify cleared
        assert!(!state.has_transactions().await);
        assert!(!state.has_ip_records().await);
        assert!(!state.is_analysis_complete().await);
    }
}
