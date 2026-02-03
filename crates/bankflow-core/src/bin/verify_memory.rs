use bankflow_core::{Exporter, IpMatcher, Parser, Processor};

fn read_fixture_bytes(name: &str) -> Vec<u8> {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("fixtures")
        .join(name);
    std::fs::read(base).expect("read fixture")
}

#[cfg(unix)]
fn peak_rss_kb() -> i64 {
    use libc::{getrusage, rusage, RUSAGE_SELF};
    unsafe {
        let mut usage: rusage = std::mem::zeroed();
        if getrusage(RUSAGE_SELF, &mut usage) == 0 {
            usage.ru_maxrss as i64
        } else {
            -1
        }
    }
}

#[cfg(not(unix))]
fn peak_rss_kb() -> i64 {
    -1
}

fn main() {
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
    let _exported = Exporter::export_to_bytes(&transactions, &income, &expense).expect("export");

    let peak = peak_rss_kb();
    if peak >= 0 {
        println!("Peak RSS: {} KB", peak);
    } else {
        println!("Peak RSS: unsupported on this platform");
    }
}
