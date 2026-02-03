use bankflow_core::parser::Parser;
use rust_xlsxwriter::{Workbook, XlsxError};

fn build_file_a_bytes() -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    let ws = workbook.add_worksheet();
    ws.write_string(0, 0, "交易序號")?;
    ws.write_string(0, 1, "帳號")?;
    ws.write_string(0, 2, "客戶姓名")?;
    ws.write_string(0, 3, "交易時間")?;
    ws.write_string(0, 4, "交易類型")?;
    ws.write_string(0, 5, "身分證/統編")?;
    ws.write_string(0, 6, "交易摘要")?;
    ws.write_string(0, 7, "交易後餘額")?;
    ws.write_string(0, 8, "支出金額")?;
    ws.write_string(0, 9, "存入金額")?;

    ws.write_number(1, 0, 1.0)?;
    ws.write_string(1, 1, "ACC123")?;
    ws.write_string(1, 3, "2024-01-15 10:30:00")?;
    ws.write_number(1, 8, 500.0)?;
    ws.write_number(1, 9, 0.0)?;

    workbook.save_to_buffer()
}

fn build_file_b_bytes() -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    let ws = workbook.add_worksheet();
    ws.write_string(0, 0, "登入序號")?;
    ws.write_string(0, 1, "帳號")?;
    ws.write_string(0, 2, "登入時間")?;
    ws.write_string(0, 3, "IP位址")?;
    ws.write_string(0, 4, "裝置資訊")?;
    ws.write_string(0, 5, "登入地區")?;

    ws.write_number(1, 0, 1.0)?;
    ws.write_string(1, 1, "ACC123")?;
    ws.write_string(1, 2, "2024-01-15 10:30:01")?;
    ws.write_string(1, 3, "203.0.113.1")?;

    workbook.save_to_buffer()
}

#[test]
fn parse_transactions_uses_header_mapping() {
    let bytes = build_file_a_bytes().expect("build file a");
    let (transactions, _meta) =
        Parser::parse_transactions_from_bytes(&bytes, "a.xlsx", None).expect("parse");
    assert_eq!(transactions.len(), 1);
    let tx = &transactions[0];
    assert_eq!(tx.account, "ACC123");
    assert_eq!(tx.timestamp, "2024-01-15 10:30:00");
    assert_eq!(tx.expense, Some(500.0));
}

#[test]
fn parse_ip_records_uses_header_mapping() {
    let bytes = build_file_b_bytes().expect("build file b");
    let (records, _meta) =
        Parser::parse_ip_records_from_bytes(&bytes, "b.xlsx", None).expect("parse");
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.account, "ACC123");
    assert_eq!(record.timestamp, "2024-01-15 10:30:01");
    assert_eq!(record.ip_address, "203.0.113.1");
}
