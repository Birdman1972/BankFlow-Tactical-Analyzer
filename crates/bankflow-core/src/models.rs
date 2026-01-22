//! Data models for the application
//!
//! Uses NaiveDateTime for WASM compatibility (no timezone support in browsers)

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Metadata about a loaded file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Full path (optional, only available on native)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub filename: String,
    pub row_count: usize,
    pub column_count: usize,
    pub file_type: String,
}

/// A single transaction record from File A
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Parsed timestamp for comparison (NaiveDateTime for WASM compat)
    #[serde(skip)]
    pub datetime: Option<NaiveDateTime>,
    /// Original timestamp string for display
    pub timestamp: String,
    /// Account identifier
    pub account: String,
    /// Income amount (Column J)
    pub income: Option<f64>,
    /// Expense amount (Column I)
    pub expense: Option<f64>,
    /// Matched IP address(es) after analysis
    pub matched_ip: Option<String>,
    /// Country from Whois lookup
    pub ip_country: Option<String>,
    /// ISP from Whois lookup
    pub ip_isp: Option<String>,
    /// All raw column values for export
    pub raw_columns: Vec<String>,
    /// Row index in original file (1-based)
    pub row_index: usize,
}

impl Transaction {
    /// Create a new transaction with parsed datetime
    pub fn new(
        timestamp: String,
        account: String,
        income: Option<f64>,
        expense: Option<f64>,
        raw_columns: Vec<String>,
        row_index: usize,
    ) -> Self {
        let datetime = parse_datetime_string(&timestamp);
        Self {
            datetime,
            timestamp,
            account,
            income,
            expense,
            matched_ip: None,
            ip_country: None,
            ip_isp: None,
            raw_columns,
            row_index,
        }
    }
}

/// An IP login record from File B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRecord {
    /// Parsed timestamp for comparison
    #[serde(skip)]
    pub datetime: Option<NaiveDateTime>,
    /// Original timestamp string
    pub timestamp: String,
    /// Account identifier
    pub account: String,
    /// IP address
    pub ip_address: String,
    /// Row index in original file (1-based)
    pub row_index: usize,
}

impl IpRecord {
    /// Create a new IP record with parsed datetime
    pub fn new(timestamp: String, account: String, ip_address: String, row_index: usize) -> Self {
        let datetime = parse_datetime_string(&timestamp);
        Self {
            datetime,
            timestamp,
            account,
            ip_address,
            row_index,
        }
    }
}

/// Parse datetime from various string formats
fn parse_datetime_string(s: &str) -> Option<NaiveDateTime> {
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%d/%m/%Y %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y/%m/%d %H:%M",
        "%m/%d/%Y %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
    ];

    for fmt in formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(s.trim(), fmt) {
            return Some(naive);
        }
    }
    None
}

/// Excel serial date to NaiveDateTime conversion
pub fn excel_date_to_datetime(serial: f64) -> Option<NaiveDateTime> {
    // Excel epoch is 1899-12-30 (accounting for the 1900 leap year bug)
    let days = serial.floor() as i64;
    let time_fraction = serial - serial.floor();

    // Seconds in a day
    let seconds = (time_fraction * 86400.0).round() as u32;
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    // Excel epoch: 1899-12-30
    let epoch = NaiveDateTime::parse_from_str("1899-12-30 00:00:00", "%Y-%m-%d %H:%M:%S").ok()?;
    Some(
        epoch
            + chrono::Duration::days(days)
            + chrono::Duration::hours(hours as i64)
            + chrono::Duration::minutes(minutes as i64)
            + chrono::Duration::seconds(secs as i64),
    )
}

/// Analysis result summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub total_records: usize,
    pub matched_count: usize,
    pub multi_ip_count: usize,
    pub whois_queried: usize,
    pub settings: AnalysisSettings,
}

/// Analysis settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSettings {
    pub hide_sensitive: bool,
    pub split_income_expense: bool,
    pub ip_cross_reference: bool,
    pub whois_lookup: bool,
}

impl Default for AnalysisSettings {
    fn default() -> Self {
        Self {
            hide_sensitive: false,
            split_income_expense: true,
            ip_cross_reference: true,
            whois_lookup: false,
        }
    }
}

/// Whois query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhoisResult {
    pub ip: String,
    pub country: Option<String>,
    pub isp: Option<String>,
    pub query_success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_datetime() {
        let dt = parse_datetime_string("2024-01-15 10:30:00");
        assert!(dt.is_some());
        let dt = dt.unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
    }

    #[test]
    fn test_excel_date_conversion() {
        // 45306.5 should be approximately 2024-01-15 12:00:00
        let dt = excel_date_to_datetime(45306.5);
        assert!(dt.is_some());
    }
}
