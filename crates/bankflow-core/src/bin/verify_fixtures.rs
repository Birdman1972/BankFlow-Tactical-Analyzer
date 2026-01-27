use bankflow_core::matcher::IpMatcher;
use bankflow_core::parser::Parser;
use bankflow_core::processor::{ProcessingStats, Processor};
use std::fs;
use std::path::PathBuf;

fn main() {
    if let Err(msg) = run() {
        eprintln!("ERROR: {msg}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let (file_a, file_b) = parse_args();

    let bytes_a =
        fs::read(&file_a).map_err(|e| format!("Failed to read File A ({file_a:?}): {e}"))?;
    let bytes_b =
        fs::read(&file_b).map_err(|e| format!("Failed to read File B ({file_b:?}): {e}"))?;

    let (mut transactions, _meta_a) =
        Parser::parse_transactions_from_bytes(&bytes_a, "FileA.xlsx").map_err(|e| e.to_string())?;
    let (ip_records, _meta_b) =
        Parser::parse_ip_records_from_bytes(&bytes_b, "FileB.xlsx").map_err(|e| e.to_string())?;

    if transactions.is_empty() {
        return Err("File A parsed to 0 transactions".to_string());
    }
    if ip_records.is_empty() {
        return Err("File B parsed to 0 IP records".to_string());
    }

    // Ensure split logic is exercised.
    let processing_stats = ProcessingStats::from_transactions(&transactions);
    if processing_stats.income_count == 0 || processing_stats.expense_count == 0 {
        return Err(format!(
            "Fixture does not exercise split logic (income_count={}, expense_count={})",
            processing_stats.income_count, processing_stats.expense_count
        ));
    }

    // Ensure counterparty column (canonical index 11) exists and is non-empty for at least one row.
    let counterparty_nonempty = transactions
        .iter()
        .filter(|tx| {
            tx.raw_columns
                .get(11)
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
        })
        .count();
    if counterparty_nonempty == 0 {
        return Err(
            "No non-empty counterparty accounts found in canonical column L (index 11)".to_string(),
        );
    }

    // Ensure IP matching produces both matched and multi-IP cases.
    let matcher = IpMatcher::with_default_window(&ip_records);
    matcher.match_all(&mut transactions);
    let match_stats = matcher.get_stats(&transactions);
    if match_stats.matched == 0 {
        return Err("No transactions matched any IP records within [-1s, +2s]".to_string());
    }
    if match_stats.multi_ip == 0 {
        return Err("No transactions produced multi-IP matches".to_string());
    }

    // Ensure masking actually removes the 4 canonical sensitive columns from raw_columns.
    let original_len = transactions[0].raw_columns.len();
    let mut masked = transactions.clone();
    Processor::new(true).process(&mut masked);
    let masked_len = masked[0].raw_columns.len();
    if original_len >= 13 && masked_len != original_len.saturating_sub(4) {
        return Err(format!(
            "Masking did not remove 4 columns as expected (original_len={original_len}, masked_len={masked_len})"
        ));
    }

    // Ensure split results are non-empty.
    let (income, expense) = Processor::split_income_expense(&transactions);
    if income.is_empty() || expense.is_empty() {
        return Err(
            "Split results are empty (expected both Income and Expense sheets to be non-empty)"
                .to_string(),
        );
    }

    println!("OK: fixtures verified");
    println!("- File A: {file_a:?} (rows={})", transactions.len());
    println!("- File B: {file_b:?} (rows={})", ip_records.len());
    println!(
        "- Match stats: total={}, matched={}, multi_ip={}, unmatched={} ",
        match_stats.total, match_stats.matched, match_stats.multi_ip, match_stats.unmatched
    );
    println!(
        "- Split stats: income_count={}, expense_count={} ",
        processing_stats.income_count, processing_stats.expense_count
    );
    println!("- Counterparty non-empty rows (sampled via parsed tx list): {counterparty_nonempty}");
    Ok(())
}

fn parse_args() -> (PathBuf, PathBuf) {
    let mut file_a: Option<PathBuf> = None;
    let mut file_b: Option<PathBuf> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--file-a" => file_a = args.next().map(PathBuf::from),
            "--file-b" => file_b = args.next().map(PathBuf::from),
            _ => {}
        }
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let default_a = manifest_dir.join("../../tests/fixtures/test_transactions_1000.xlsx");
    let default_b = manifest_dir.join("../../tests/fixtures/test_ip_records_1000.xlsx");
    (file_a.unwrap_or(default_a), file_b.unwrap_or(default_b))
}
