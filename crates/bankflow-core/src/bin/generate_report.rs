use bankflow_core::exporter::Exporter;
use bankflow_core::matcher::IpMatcher;
use bankflow_core::parser::Parser;
use bankflow_core::processor::Processor;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let (file_a_path, file_b_path, out_path, enable_whois, hide_sensitive) = parse_args(&args)?;

    println!("\n=== BankFlow Report Generator Audit ===");
    println!("File A Input:    {:?}", file_a_path);
    println!("File B Input:    {:?}", file_b_path);
    println!("Output Target:   {:?}", out_path);
    println!("---------------------------------------");
    println!("Feature: Whois Lookup    [{}]", if enable_whois { "ON" } else { "OFF" });
    println!("Feature: Hide Sensitive  [{}]", if hide_sensitive { "ON" } else { "OFF" });
    println!("Feature: IP Matching     [ON] (Always)");
    println!("Feature: Split Sheets    [ON] (Always)");
    println!("---------------------------------------");

    let start = Instant::now();

    // 1. Parse File A
    print!("> Parsing File A... ");
    let bytes_a = fs::read(&file_a_path)
        .map_err(|e| format!("Failed to read File A: {}", e))?;
    let (mut transactions, _meta_a) =
        Parser::parse_transactions_from_bytes(&bytes_a, "FileA.xlsx", None)
        .map_err(|e| e.to_string())?;
    println!("OK ({} tx)", transactions.len());

    // 2. Parse File B
    print!("> Parsing File B... ");
    let bytes_b = fs::read(&file_b_path)
        .map_err(|e| format!("Failed to read File B: {}", e))?;
    let (ip_records, _meta_b) = Parser::parse_ip_records_from_bytes(&bytes_b, "FileB.xlsx", None)
        .map_err(|e| e.to_string())?;
    println!("OK ({} rec)", ip_records.len());

    // 3. Match IPs
    print!("> Matching IPs...   ");
    let matcher = IpMatcher::with_default_window(&ip_records);
    matcher.match_all(&mut transactions);
    let stats = matcher.get_stats(&transactions);
    println!("OK (Matched: {}, Multi-IP: {})", stats.matched, stats.multi_ip);

    // 4. Whois Simulation (Verify: Data population)
    if enable_whois {
        print!("> Simulating Whois... ");
        let mut lookup_count = 0;
        for tx in transactions.iter_mut() {
            if let Some(ip_str) = &tx.matched_ip {
                // Skip Whois for "N/A" (No Match)
                if ip_str == "N/A" || ip_str.starts_with("N/A") {
                    continue;
                }

                lookup_count += 1;
                // Basic mock logic for verification
                let (country, isp) = if ip_str.contains("192.168.") || ip_str.contains("10.") || ip_str.contains("172.") {
                     ("Private".to_string(), "Local Network".to_string())
                } else if ip_str.contains("210.") || ip_str.contains("220.") || ip_str.contains("111.") || ip_str.contains("59.") { 
                     ("TW".to_string(), "Chunghwa Telecom".to_string()) 
                } else if ip_str.contains("8.8.8.8") || ip_str.contains("1.1.1.1") {
                     ("US".to_string(), "Google LLC".to_string())
                } else {
                     ("Unknown".to_string(), "Unknown ISP".to_string())
                };
                tx.ip_country = Some(country);
                tx.ip_isp = Some(isp);
            }
        }
        println!("OK ({} lookups performed)", lookup_count);
    } else {
        println!("> Whois Disabled.   (Skipped)");
    }

    // 5. Transform (Verify: Hide Sensitive)
    if hide_sensitive {
        print!("> Masking Columns...  ");
        let original_cols = transactions[0].raw_columns.len();
        Processor::new(true).process(&mut transactions);
        let new_cols = transactions[0].raw_columns.len();
        println!("OK (Cols: {} -> {})", original_cols, new_cols);
        if new_cols >= original_cols {
            eprintln!("WARNING: Column count did not decrease!");
        }
    } else {
        println!("> Masking Disabled.   (Skipped)");
    }

    // 6. Split Income/Expense
    print!("> Splitting Sheets... ");
    let (income, expense) = Processor::split_income_expense(&transactions);
    println!("OK (Income: {}, Expense: {})", income.len(), expense.len());

    // 7. Export
    print!("> Exporting XLSX...   ");
    let xlsx_bytes = Exporter::export_to_bytes(&transactions, &income, &expense)
        .map_err(|e| e.to_string())?;

    fs::write(&out_path, xlsx_bytes)
        .map_err(|e| format!("Failed to write output file: {}", e))?;
    println!("OK ({})", out_path.display());

    println!("---------------------------------------");
    println!("Total Time: {:.2?}", start.elapsed());
    Ok(())
}

fn parse_args(args: &[String]) -> Result<(PathBuf, PathBuf, PathBuf, bool, bool), String> {
    let mut file_a: Option<PathBuf> = None;
    let mut file_b: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut whois = false;
    let mut hide_sensitive = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file-a" => {
                i += 1;
                if i < args.len() { file_a = Some(PathBuf::from(&args[i])); }
            }
            "--file-b" => {
                i += 1;
                if i < args.len() { file_b = Some(PathBuf::from(&args[i])); }
            }
            "--out" => {
                i += 1;
                if i < args.len() { out = Some(PathBuf::from(&args[i])); }
            }
            "--whois" => {
                whois = true;
            }
            "--hide-sensitive" => {
                hide_sensitive = true;
            }
            _ => {}
        }
        i += 1;
    }

    match (file_a, file_b, out) {
        (Some(a), Some(b), Some(o)) => Ok((a, b, o, whois, hide_sensitive)),
        _ => Err("Usage: generate_report --file-a <path> --file-b <path> --out <path> [--whois] [--hide-sensitive]".to_string()),
    }
}
