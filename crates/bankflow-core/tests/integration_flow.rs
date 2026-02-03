use bankflow_core::{Exporter, IpMatcher, Parser, Processor};
use calamine::{open_workbook_auto_from_rs, Reader};
use std::io::Cursor;

fn read_fixture_bytes(name: &str) -> Vec<u8> {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("fixtures")
        .join(name);
    std::fs::read(base).expect("read fixture")
}

#[test]
fn integration_pipeline_exports_workbook() {
    let bytes_a = read_fixture_bytes("tc_small_A.xlsx");
    let bytes_b = read_fixture_bytes("tc_small_B.xlsx");

    let (mut transactions, _meta_a) =
        Parser::parse_transactions_from_bytes(&bytes_a, "FileA.xlsx", None).expect("parse a");
    let (ip_records, _meta_b) =
        Parser::parse_ip_records_from_bytes(&bytes_b, "FileB.xlsx", None).expect("parse b");

    let matcher = IpMatcher::with_default_window(&ip_records);
    matcher.match_all(&mut transactions);

    let processor = Processor::new(true);
    processor.process(&mut transactions);

    let (income, expense) = Processor::split_income_expense(&transactions);
    let exported = Exporter::export_to_bytes(&transactions, &income, &expense).expect("export");

    let cursor = Cursor::new(exported);
    let workbook = open_workbook_auto_from_rs(cursor).expect("open export");
    let sheets = workbook.sheet_names().to_vec();
    assert!(sheets.contains(&"Summary".to_string()));
    assert!(sheets.contains(&"Income".to_string()));
    assert!(sheets.contains(&"Expense".to_string()));
    assert!(sheets.contains(&"Counterparty".to_string()));
}
