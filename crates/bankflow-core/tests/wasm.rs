//! WASM integration tests
//!
//! Run with: wasm-pack test --node --features wasm
//! Or: wasm-pack test --headless --chrome --features wasm

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

// Test the core types work in WASM environment
#[wasm_bindgen_test]
fn test_transaction_serialization() {
    use bankflow_core::Transaction;

    let tx = Transaction::new(
        "2024-01-15 10:30:00".to_string(),
        "ACC001".to_string(),
        Some(1000.0),
        None,
        vec!["col1".to_string(), "col2".to_string()],
        1,
    );

    // Test serialization works
    let json = serde_json::to_string(&tx).unwrap();
    assert!(json.contains("ACC001"));
    assert!(json.contains("1000"));
}

#[wasm_bindgen_test]
fn test_ip_record_serialization() {
    use bankflow_core::IpRecord;

    let record = IpRecord::new(
        "2024-01-15 10:30:00".to_string(),
        "ACC001".to_string(),
        "192.168.1.1".to_string(),
        1,
    );

    let json = serde_json::to_string(&record).unwrap();
    assert!(json.contains("192.168.1.1"));
}

#[wasm_bindgen_test]
fn test_file_metadata_serialization() {
    use bankflow_core::FileMetadata;

    let meta = FileMetadata {
        path: None,
        filename: "test.xlsx".to_string(),
        row_count: 100,
        column_count: 10,
        file_type: "xlsx".to_string(),
    };

    let json = serde_json::to_string(&meta).unwrap();
    assert!(json.contains("test.xlsx"));
    assert!(json.contains("100"));
}

#[wasm_bindgen_test]
fn test_processor_split_income_expense() {
    use bankflow_core::{Processor, Transaction};

    let transactions = vec![
        Transaction::new(
            "2024-01-15 10:30:00".to_string(),
            "ACC001".to_string(),
            Some(1000.0),
            None,
            vec![],
            1,
        ),
        Transaction::new(
            "2024-01-15 10:31:00".to_string(),
            "ACC002".to_string(),
            None,
            Some(500.0),
            vec![],
            2,
        ),
    ];

    let (income, expense) = Processor::split_income_expense(&transactions);

    assert_eq!(income.len(), 1);
    assert_eq!(expense.len(), 1);
}

#[wasm_bindgen_test]
fn test_matcher_basic() {
    use bankflow_core::{IpMatcher, IpRecord, Transaction};

    let ip_records = vec![IpRecord::new(
        "2024-01-15 10:30:00".to_string(),
        "ACC001".to_string(),
        "192.168.1.1".to_string(),
        1,
    )];

    let mut transactions = vec![Transaction::new(
        "2024-01-15 10:30:00".to_string(),
        "ACC001".to_string(),
        Some(1000.0),
        None,
        vec![],
        1,
    )];

    let matcher = IpMatcher::with_default_window(&ip_records);
    matcher.match_all(&mut transactions);

    assert_eq!(transactions[0].matched_ip, Some("192.168.1.1".to_string()));
}
