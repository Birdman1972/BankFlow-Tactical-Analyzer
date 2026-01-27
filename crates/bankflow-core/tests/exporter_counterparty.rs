use bankflow_core::exporter::Exporter;
use bankflow_core::models::Transaction;
use calamine::{open_workbook_auto_from_rs, Data, Reader};
use std::io::Cursor;

fn make_raw_columns(counterparty: &str) -> Vec<String> {
    // Canonical raw columns length 13 (A..M). Counterparty account is index 11 (L).
    let mut cols = vec!["".to_string(); 13];
    cols[11] = counterparty.to_string();
    cols
}

fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::String(s) => s.clone(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(dt) => dt.to_string(),
        Data::DateTimeIso(s) => s.clone(),
        Data::DurationIso(s) => s.clone(),
        Data::Empty => "".to_string(),
        Data::Error(e) => format!("#ERROR: {:?}", e),
    }
}

#[test]
fn export_includes_counterparty_sheet() {
    let income_tx = Transaction::new(
        "2024-01-15 10:30:00".to_string(),
        "ACC001".to_string(),
        Some(1000.0),
        None,
        make_raw_columns("CP-IN-001"),
        2,
    );
    let expense_tx = Transaction::new(
        "2024-01-15 10:30:01".to_string(),
        "ACC002".to_string(),
        None,
        Some(500.0),
        make_raw_columns("CP-EX-001"),
        3,
    );

    let bytes = Exporter::export_to_bytes(
        &[income_tx.clone(), expense_tx.clone()],
        &[income_tx],
        &[expense_tx],
    )
    .expect("export");

    let cursor = Cursor::new(bytes);
    let mut workbook = open_workbook_auto_from_rs(cursor).expect("open workbook");
    let sheet_names = workbook.sheet_names().to_vec();
    assert!(sheet_names.iter().any(|n| n == "Counterparty"));

    let range = workbook.worksheet_range("Counterparty").expect("range");
    let header_row = range.rows().next().expect("header row");
    let headers: Vec<String> = header_row.iter().map(cell_to_string).collect();
    assert_eq!(headers[0], "Income Counterparty Account");
    assert_eq!(headers[1], "Expense Counterparty Account");

    // First data row should include at least one of the values.
    let first_data_row = range.rows().nth(1).expect("first data row");
    let v0 = cell_to_string(&first_data_row[0]);
    let v1 = cell_to_string(&first_data_row[1]);
    assert!(v0 == "CP-IN-001" || v1 == "CP-EX-001");
}
