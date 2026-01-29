use bankflow_core::parser::Parser;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    println!("🧪 Verifying Smart Repair with REAL FILE (broken_A.xlsx)...");

    let path = Path::new("../../fixtures/broken_A.xlsx");
    if !path.exists() {
        eprintln!("❌ Error: broken_A.xlsx not found. Run generate_broken_fixture.py first.");
        std::process::exit(1);
    }

    // 1. Attempt Default Parse (Should Fail)
    println!("\n--- Test 1: Default Parse (Expect Failure) ---");
    match Parser::parse_transactions(path, None) {
        Ok(_) => println!("❌ Test 1 Failed: Should have rejected the broken file."),
        Err(e) => {
            println!("✅ Test 1 Passed: Rejected file as expected.");
            println!("   Error: {:?}", e);
        }
    }

    // 2. Attempt Repaired Parse (Should Succeed)
    println!("\n--- Test 2: Repaired Parse (Expect Success) ---");
    let mut mapping = HashMap::new();
    mapping.insert("timestamp".to_string(), "CustomTime".to_string());
    mapping.insert("account".to_string(), "UserID".to_string());

    match Parser::parse_transactions(path, Some(&mapping)) {
        Ok(txs) => {
            println!("✅ Test 2 Passed: Successfully parsed file with mapping.");
            println!("   - Record Count: {}", txs.len());
            // Verify count (should be 50 based on generator default)
            if txs.len() == 50 {
                println!("   - Count Verified: 50 records.");
            } else {
                println!("   ⚠️ Count Mismatch: Expected 50, got {}", txs.len());
            }
        },
        Err(e) => println!("❌ Test 2 Failed: Repair unsuccessful. Error: {:?}", e),
    }

    println!("\n🔍 Real File Verification Complete.");
}
