use bankflow_core::exporter::Exporter;
use bankflow_core::models::Transaction;
use calamine::{open_workbook_auto_from_rs, Reader};
use std::io::Cursor;

fn build_sample_transaction() -> Transaction {
    Transaction::new(
        "2024-01-15 10:30:00".to_string(),
        "ACC123".to_string(),
        Some(1000.0),
        Some(0.0),
        vec!["RAW1".to_string(), "RAW2".to_string(), "RAW3".to_string()],
        2,
    )
}

fn cell_to_string(cell: &calamine::Data) -> String {
    match cell {
        calamine::Data::String(s) => s.clone(),
        calamine::Data::Float(f) => f.to_string(),
        calamine::Data::Int(i) => i.to_string(),
        calamine::Data::Bool(b) => b.to_string(),
        calamine::Data::DateTime(dt) => dt.to_string(),
        calamine::Data::DateTimeIso(s) => s.clone(),
        calamine::Data::DurationIso(s) => s.clone(),
        calamine::Data::Empty => "".to_string(),
        calamine::Data::Error(e) => format!("#ERROR: {:?}", e),
    }
}

#[test]
fn export_headers_match_user_manual() {
    let tx = build_sample_transaction();
    let bytes = Exporter::export_to_bytes(&[tx.clone()], &[tx.clone()], &[tx]).expect("export");

    let cursor = Cursor::new(bytes);
    let mut workbook = open_workbook_auto_from_rs(cursor).expect("open workbook");
    let sheet_name = workbook.sheet_names().first().cloned().expect("sheet name");
    let range = workbook.worksheet_range(&sheet_name).expect("range");

    let header_row = range.rows().next().expect("header row");
    let headers: Vec<String> = header_row.iter().map(cell_to_string).collect();

    let expected = vec![
        "Timestamp",
        "Account",
        "Income",
        "Expense",
        "Matched IP",
        "IP Country",
        "IP ISP",
        "Raw Column 1",
        "Raw Column 2",
        "Raw Column 3",
    ];

    assert_eq!(headers, expected);
}
