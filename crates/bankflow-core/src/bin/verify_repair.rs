use bankflow_core::parser::header_map;
use std::collections::HashMap;

fn main() {
    println!("🧪 Verifying Smart Repair (Dynamic Mapping) Logic...");

    // 1. Simulate "Broken" Headers (Non-standard names)
    let broken_headers = vec![
        "CustomTime".to_string(), // Should be "交易時間"
        "帳號".to_string(), 
        "支出金額".to_string(), 
        "存入金額".to_string()
    ];

    println!("\n--- Test 1: Default Validation (Expect Failure) ---");
    match header_map::validate_file_a_headers(&broken_headers, None) {
        Ok(_) => println!("❌ Test 1 Failed: Should have rejected 'CustomTime'"),
        Err(e) => {
            println!("✅ Test 1 Passed: Correctly flagged missing columns: {:?}", e);
            assert!(e.contains(&"交易時間/timestamp".to_string()));
        }
    }

    // 2. Simulate "Repair" (Providing Mapping)
    println!("\n--- Test 2: Repaired Validation (Expect Success) ---");
    let mut mapping = HashMap::new();
    mapping.insert("timestamp".to_string(), "CustomTime".to_string());

    match header_map::validate_file_a_headers(&broken_headers, Some(&mapping)) {
        Ok(cols) => {
            println!("✅ Test 2 Passed: Successfully mapped 'CustomTime' to 'timestamp'");
            println!("   - Resolved Column Indices: {:?}", cols);
            assert_eq!(cols.timestamp, 0); // Should resolve to index 0
        },
        Err(e) => println!("❌ Test 2 Failed: Mapping did not work! Errors: {:?}", e),
    }

    println!("\n🔍 Smart Repair Verification Complete.");
}
