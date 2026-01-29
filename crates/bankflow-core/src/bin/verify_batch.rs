use bankflow_core::batch::BatchProcessor;
use std::path::Path;

fn main() {
    // Exact absolute path to avoid environment-dependent relative path issues
    let root_path = "/Volumes/Samsung SSD 990 PRO/Downloads/Project-BoB/BankFlow-Tactical-Analyzer/fixtures/batch_test";
    let root = Path::new(root_path);
    
    println!("Scanning folder: {:?}", root);

    let result = BatchProcessor::scan_folder(root, 3);
    
    println!("Scan verification result:");
    println!("{}", serde_json::to_string_pretty(&result).unwrap());

    let count_valid = result.pairs.len();
    println!("Found {} valid pairs.", count_valid);

    // 2025-01 and 2025-02 are designed to be valid.
    // 2025-03 is incomplete (A only).
    // 2025-04 is ambiguous (multiple A's).
    if count_valid == 2 {
        println!("✅ SUCCESS: Found correctly paired folders.");
    } else {
        println!("❌ FAILURE: Expected 2 valid pairs, found {}.", count_valid);
        std::process::exit(1);
    }
}
