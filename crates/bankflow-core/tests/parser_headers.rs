use bankflow_core::parser::header_map::{map_file_a_columns, map_file_b_columns};

#[test]
fn file_a_header_mapping_prefers_named_columns() {
    let headers = vec![
        "交易序號", "帳號", "客戶姓名", "交易時間", "交易類型",
        "身分證/統編", "交易摘要", "交易後餘額", "支出金額", "存入金額",
    ];
    let map = map_file_a_columns(&headers);
    assert_eq!(map.timestamp, 3);
    assert_eq!(map.account, 1);
    assert_eq!(map.expense, 8);
    assert_eq!(map.income, 9);
}

#[test]
fn file_b_header_mapping_prefers_named_columns() {
    let headers = vec!["登入序號", "帳號", "登入時間", "IP位址", "裝置資訊", "登入地區"];
    let map = map_file_b_columns(&headers);
    assert_eq!(map.timestamp, 2);
    assert_eq!(map.account, 1);
    assert_eq!(map.ip_address, 3);
}
