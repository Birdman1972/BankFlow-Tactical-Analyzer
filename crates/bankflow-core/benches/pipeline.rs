use bankflow_core::{Exporter, IpMatcher, Parser, Processor};
use criterion::{criterion_group, criterion_main, Criterion};

fn read_fixture_bytes(name: &str) -> Vec<u8> {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("fixtures")
        .join(name);
    std::fs::read(base).expect("read fixture")
}

fn bench_pipeline(c: &mut Criterion) {
    let bytes_a = read_fixture_bytes("tc_small_A.xlsx");
    let bytes_b = read_fixture_bytes("tc_small_B.xlsx");

    c.bench_function("pipeline_small", |b| {
        b.iter(|| {
            let (mut transactions, _meta_a) =
                Parser::parse_transactions_from_bytes(&bytes_a, "FileA.xlsx", None).expect("parse a");
            let (ip_records, _meta_b) =
                Parser::parse_ip_records_from_bytes(&bytes_b, "FileB.xlsx", None).expect("parse b");

            let matcher = IpMatcher::with_default_window(&ip_records);
            matcher.match_all(&mut transactions);

            let processor = Processor::new(true);
            processor.process(&mut transactions);

            let (income, expense) = Processor::split_income_expense(&transactions);
            let _exported =
                Exporter::export_to_bytes(&transactions, &income, &expense).expect("export");
        });
    });
}

criterion_group!(benches, bench_pipeline);
criterion_main!(benches);
