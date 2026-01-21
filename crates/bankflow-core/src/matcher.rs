//! IP time-window matching engine
//!
//! Uses NaiveDateTime for WASM compatibility.
//! Parallel processing via rayon is only available on native targets.

use crate::models::{IpRecord, Transaction};
use chrono::NaiveDateTime;
use std::collections::HashMap;

/// Time window configuration (in seconds)
#[derive(Debug, Clone, Copy)]
pub struct TimeWindow {
    pub before: i64,
    pub after: i64,
}

impl Default for TimeWindow {
    fn default() -> Self {
        Self { before: 1, after: 2 }
    }
}

/// A single IP match with time offset
#[derive(Debug, Clone)]
pub struct IpMatch {
    pub ip: String,
    pub offset_seconds: i64,
    pub row_index: usize,
}

/// Reference to an IP record for the index
#[derive(Debug, Clone)]
struct IpRecordRef {
    datetime: NaiveDateTime,
    ip_address: String,
    row_index: usize,
}

/// IP Matcher engine
pub struct IpMatcher {
    window: TimeWindow,
    account_index: HashMap<String, Vec<IpRecordRef>>,
}

impl IpMatcher {
    pub fn new(ip_records: &[IpRecord], window: TimeWindow) -> Self {
        let mut account_index: HashMap<String, Vec<IpRecordRef>> = HashMap::new();

        for record in ip_records {
            if let Some(dt) = record.datetime {
                account_index
                    .entry(record.account.clone())
                    .or_default()
                    .push(IpRecordRef {
                        datetime: dt,
                        ip_address: record.ip_address.clone(),
                        row_index: record.row_index,
                    });
            }
        }

        for records in account_index.values_mut() {
            records.sort_by_key(|r| r.datetime);
        }

        Self { window, account_index }
    }

    pub fn with_default_window(ip_records: &[IpRecord]) -> Self {
        Self::new(ip_records, TimeWindow::default())
    }

    /// Match a single transaction
    fn match_single(&self, tx: &Transaction) -> Option<String> {
        let tx_datetime = tx.datetime?;

        let ip_records = self.account_index.get(&tx.account)?;

        let matches: Vec<IpMatch> = ip_records
            .iter()
            .filter_map(|record| {
                let diff = record.datetime.signed_duration_since(tx_datetime);
                let offset_seconds = diff.num_seconds();

                if offset_seconds >= -(self.window.before) && offset_seconds <= self.window.after {
                    Some(IpMatch {
                        ip: record.ip_address.clone(),
                        offset_seconds,
                        row_index: record.row_index,
                    })
                } else {
                    None
                }
            })
            .collect();

        if matches.is_empty() {
            return Some("N/A".to_string());
        }

        Some(format_matches(&matches))
    }

    /// Match all transactions (sequential, WASM compatible)
    pub fn match_all(&self, transactions: &mut [Transaction]) {
        for tx in transactions.iter_mut() {
            tx.matched_ip = self.match_single(tx).or(Some("N/A".to_string()));
        }
    }

    /// Get statistics
    pub fn get_stats(&self, transactions: &[Transaction]) -> MatchStats {
        let mut matched_count = 0;
        let mut multi_ip_count = 0;

        for tx in transactions {
            if let Some(ref ip) = tx.matched_ip {
                if ip != "N/A" && !ip.starts_with("N/A") {
                    matched_count += 1;
                    if ip.contains(" | ") {
                        multi_ip_count += 1;
                    }
                }
            }
        }

        MatchStats {
            total: transactions.len(),
            matched: matched_count,
            multi_ip: multi_ip_count,
            unmatched: transactions.len() - matched_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchStats {
    pub total: usize,
    pub matched: usize,
    pub multi_ip: usize,
    pub unmatched: usize,
}

fn format_matches(matches: &[IpMatch]) -> String {
    let mut seen_ips: Vec<&str> = vec![];
    let mut unique_matches: Vec<&IpMatch> = vec![];

    for m in matches {
        if !seen_ips.contains(&m.ip.as_str()) {
            seen_ips.push(&m.ip);
            unique_matches.push(m);
        }
    }

    if unique_matches.len() == 1 {
        return unique_matches[0].ip.clone();
    }

    unique_matches
        .iter()
        .map(|m| {
            let offset_str = if m.offset_seconds == 0 {
                "0s".to_string()
            } else if m.offset_seconds > 0 {
                format!("+{}s", m.offset_seconds)
            } else {
                format!("{}s", m.offset_seconds)
            };
            format!("{}:{}", offset_str, m.ip)
        })
        .collect::<Vec<_>>()
        .join(" | ")
}
