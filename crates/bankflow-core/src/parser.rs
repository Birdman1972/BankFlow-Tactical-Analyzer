//! Excel file parser using calamine
//!
//! Supports both file path (native) and byte array (WASM) inputs.

use crate::error::CoreError;
use crate::models::{excel_date_to_datetime, FileMetadata, IpRecord, Transaction};
use calamine::{open_workbook_auto_from_rs, Data, DataType, Reader, Sheets};
use std::io::Cursor;

/// Column indices for File A (Transaction file)
mod file_a_columns {
    pub const TIMESTAMP: usize = 0;
    pub const ACCOUNT: usize = 1;
    pub const EXPENSE: usize = 8;
    pub const INCOME: usize = 9;
}

/// Column indices for File B (IP log file)
mod file_b_columns {
    pub const TIMESTAMP: usize = 0;
    pub const ACCOUNT: usize = 1;
    pub const IP_ADDRESS: usize = 2;
}

/// Header-based column mapping helpers
pub mod header_map {
    #[derive(Debug, Clone, Copy)]
    pub struct FileAColumns {
        pub timestamp: usize,
        pub account: usize,
        pub expense: usize,
        pub income: usize,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct FileBColumns {
        pub timestamp: usize,
        pub account: usize,
        pub ip_address: usize,
    }

    fn normalize_header(value: &str) -> String {
        value.trim().to_lowercase()
    }

    fn find_index(headers: &[String], candidates: &[&str]) -> Option<usize> {
        for (idx, header) in headers.iter().enumerate() {
            let normalized = normalize_header(header);
            if candidates.iter().any(|c| normalized == normalize_header(c)) {
                return Some(idx);
            }
        }
        None
    }

    pub fn map_file_a_columns(raw_headers: &[&str]) -> FileAColumns {
        let headers: Vec<String> = raw_headers.iter().map(|h| h.to_string()).collect();
        map_file_a_columns_from_strings(&headers)
    }

    pub fn map_file_a_columns_from_strings(headers: &[String]) -> FileAColumns {
        let timestamp = find_index(headers, &["交易時間", "時間", "timestamp", "交易日期"]).unwrap_or(super::file_a_columns::TIMESTAMP);
        let account = find_index(headers, &["帳號", "account", "account_id"]).unwrap_or(super::file_a_columns::ACCOUNT);
        let expense = find_index(headers, &["支出金額", "expense", "支出"]).unwrap_or(super::file_a_columns::EXPENSE);
        let income = find_index(headers, &["存入金額", "收入金額", "income", "存入"]).unwrap_or(super::file_a_columns::INCOME);
        FileAColumns { timestamp, account, expense, income }
    }

    pub fn map_file_b_columns(raw_headers: &[&str]) -> FileBColumns {
        let headers: Vec<String> = raw_headers.iter().map(|h| h.to_string()).collect();
        map_file_b_columns_from_strings(&headers)
    }

    pub fn map_file_b_columns_from_strings(headers: &[String]) -> FileBColumns {
        let timestamp = find_index(headers, &["登入時間", "時間", "timestamp"]).unwrap_or(super::file_b_columns::TIMESTAMP);
        let account = find_index(headers, &["帳號", "account", "account_id"]).unwrap_or(super::file_b_columns::ACCOUNT);
        let ip_address = find_index(headers, &["ip位址", "ip地址", "ip", "ip address"]).unwrap_or(super::file_b_columns::IP_ADDRESS);
        FileBColumns { timestamp, account, ip_address }
    }
}

/// Excel parser that works with both native and WASM
pub struct Parser;

impl Parser {
    /// Parse transactions from File A bytes
    pub fn parse_transactions_from_bytes(
        data: &[u8],
        filename: &str,
    ) -> Result<(Vec<Transaction>, FileMetadata), CoreError> {
        let cursor = Cursor::new(data);
        let mut workbook: Sheets<_> = open_workbook_auto_from_rs(cursor).map_err(|e| {
            CoreError::ExcelParseError(format!("Failed to open file: {}", e))
        })?;

        let sheet_name = workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| CoreError::ExcelParseError("No sheets found".to_string()))?;

        let range = workbook
            .worksheet_range(&sheet_name)
            .map_err(|e| CoreError::ExcelParseError(format!("Failed to read sheet: {}", e)))?;

        let row_count = range.height();
        let col_count = range.width();

        let mut transactions = Vec::new();
        let headers: Vec<String> = range
            .rows()
            .next()
            .map(|row| row.iter().map(cell_to_string).collect())
            .unwrap_or_default();
        let columns = header_map::map_file_a_columns_from_strings(&headers);

        for (row_idx, row) in range.rows().enumerate().skip(1) {
            if row.is_empty() || row.iter().all(|c| c.is_empty()) {
                continue;
            }

            let timestamp = extract_cell_as_string(row.get(columns.timestamp));
            let account = extract_cell_as_string(row.get(columns.account));

            if timestamp.is_empty() || account.is_empty() {
                continue;
            }

            let expense = extract_cell_as_f64(row.get(columns.expense));
            let income = extract_cell_as_f64(row.get(columns.income));
            let raw_columns: Vec<String> = row.iter().map(cell_to_string).collect();

            let transaction = Transaction::new(
                timestamp,
                account,
                income,
                expense,
                raw_columns,
                row_idx + 1,
            );

            transactions.push(transaction);
        }

        let metadata = FileMetadata {
            path: None,
            filename: filename.to_string(),
            row_count,
            column_count: col_count,
            file_type: "xlsx".to_string(),
        };

        Ok((transactions, metadata))
    }

    /// Parse IP records from File B bytes
    pub fn parse_ip_records_from_bytes(
        data: &[u8],
        filename: &str,
    ) -> Result<(Vec<IpRecord>, FileMetadata), CoreError> {
        let cursor = Cursor::new(data);
        let mut workbook: Sheets<_> = open_workbook_auto_from_rs(cursor).map_err(|e| {
            CoreError::ExcelParseError(format!("Failed to open file: {}", e))
        })?;

        let sheet_name = workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| CoreError::ExcelParseError("No sheets found".to_string()))?;

        let range = workbook
            .worksheet_range(&sheet_name)
            .map_err(|e| CoreError::ExcelParseError(format!("Failed to read sheet: {}", e)))?;

        let row_count = range.height();
        let col_count = range.width();

        let mut records = Vec::new();
        let headers: Vec<String> = range
            .rows()
            .next()
            .map(|row| row.iter().map(cell_to_string).collect())
            .unwrap_or_default();
        let columns = header_map::map_file_b_columns_from_strings(&headers);

        for (row_idx, row) in range.rows().enumerate().skip(1) {
            if row.is_empty() || row.iter().all(|c| c.is_empty()) {
                continue;
            }

            let timestamp = extract_cell_as_string(row.get(columns.timestamp));
            let account = extract_cell_as_string(row.get(columns.account));
            let ip_address = extract_cell_as_string(row.get(columns.ip_address));

            if timestamp.is_empty() || account.is_empty() || ip_address.is_empty() {
                continue;
            }

            let record = IpRecord::new(timestamp, account, ip_address, row_idx + 1);
            records.push(record);
        }

        let metadata = FileMetadata {
            path: None,
            filename: filename.to_string(),
            row_count,
            column_count: col_count,
            file_type: "xlsx".to_string(),
        };

        Ok((records, metadata))
    }

    /// Get file metadata from bytes
    pub fn get_metadata_from_bytes(data: &[u8], filename: &str) -> Result<FileMetadata, CoreError> {
        let cursor = Cursor::new(data);
        let mut workbook: Sheets<_> = open_workbook_auto_from_rs(cursor)
            .map_err(|e| CoreError::ExcelParseError(format!("Failed to open file: {}", e)))?;

        let sheet_name = workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| CoreError::ExcelParseError("No sheets found".to_string()))?;

        let range = workbook
            .worksheet_range(&sheet_name)
            .map_err(|e| CoreError::ExcelParseError(format!("Failed to read sheet: {}", e)))?;

        Ok(FileMetadata {
            path: None,
            filename: filename.to_string(),
            row_count: range.height(),
            column_count: range.width(),
            file_type: "xlsx".to_string(),
        })
    }
}

fn extract_cell_as_string(cell: Option<&Data>) -> String {
    match cell {
        Some(data) => cell_to_string(data),
        None => String::new(),
    }
}

fn cell_to_string(data: &Data) -> String {
    match data {
        Data::Int(i) => i.to_string(),
        Data::Float(f) => {
            if *f > 1.0 && *f < 2958466.0 && f.fract() != 0.0 {
                if let Some(dt) = excel_date_to_datetime(*f) {
                    return dt.format("%Y-%m-%d %H:%M:%S").to_string();
                }
            }
            f.to_string()
        }
        Data::String(s) => s.clone(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(excel_dt) => {
            let serial = excel_dt.as_f64();
            if let Some(dt) = excel_date_to_datetime(serial) {
                dt.format("%Y-%m-%d %H:%M:%S").to_string()
            } else {
                excel_dt.to_string()
            }
        }
        Data::Error(e) => format!("#ERROR: {:?}", e),
        Data::Empty => String::new(),
        _ => String::new(),
    }
}

fn extract_cell_as_f64(cell: Option<&Data>) -> Option<f64> {
    match cell {
        Some(Data::Float(f)) => {
            if *f != 0.0 {
                Some(*f)
            } else {
                None
            }
        }
        Some(Data::Int(i)) => {
            if *i != 0 {
                Some(*i as f64)
            } else {
                None
            }
        }
        Some(Data::String(s)) => s.trim().parse().ok().filter(|v: &f64| *v != 0.0),
        _ => None,
    }
}

// Native-only functions (not available in WASM)
#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;
    use std::fs;
    use std::path::Path;

    impl Parser {
        /// Parse transactions from file path (native only)
        pub fn parse_transactions(path: &Path) -> Result<Vec<Transaction>, CoreError> {
            let data = fs::read(path).map_err(|e| {
                CoreError::ExcelParseError(format!("Failed to read file: {}", e))
            })?;
            let filename = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let (transactions, _) = Parser::parse_transactions_from_bytes(&data, &filename)?;
            Ok(transactions)
        }

        /// Parse IP records from file path (native only)
        pub fn parse_ip_records(path: &Path) -> Result<Vec<IpRecord>, CoreError> {
            let data = fs::read(path).map_err(|e| {
                CoreError::ExcelParseError(format!("Failed to read file: {}", e))
            })?;
            let filename = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let (records, _) = Parser::parse_ip_records_from_bytes(&data, &filename)?;
            Ok(records)
        }

        /// Get file metadata from path (native only)
        pub fn get_file_metadata(path: &Path) -> Result<(usize, usize), CoreError> {
            let data = fs::read(path).map_err(|e| {
                CoreError::ExcelParseError(format!("Failed to read file: {}", e))
            })?;
            let filename = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let metadata = Parser::get_metadata_from_bytes(&data, &filename)?;
            Ok((metadata.row_count, metadata.column_count))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
pub use native::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_to_string() {
        assert_eq!(cell_to_string(&Data::Int(42)), "42");
        assert_eq!(cell_to_string(&Data::String("test".to_string())), "test");
        assert_eq!(cell_to_string(&Data::Empty), "");
    }
}
