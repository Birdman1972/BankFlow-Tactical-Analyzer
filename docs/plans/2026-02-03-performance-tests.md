# Performance Tests Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a lightweight performance benchmark for the core parse -> match -> process -> export pipeline.

**Architecture:** Add a Criterion benchmark under `crates/bankflow-core/benches/` that loads existing fixtures and measures end-to-end pipeline duration on stable Rust.

**Tech Stack:** Rust, `criterion`, core modules (`Parser`, `IpMatcher`, `Processor`, `Exporter`).

### Task 1: Add Criterion benchmark

**Files:**
- Modify: `crates/bankflow-core/Cargo.toml`
- Create: `crates/bankflow-core/benches/pipeline.rs`

**Step 1: Add criterion dev-dependency and bench target**

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "pipeline"
harness = false
```

**Step 2: Add benchmark file**

```rust
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
            let _exported = Exporter::export_to_bytes(&transactions, &income, &expense)
                .expect("export");
        });
    });
}

criterion_group!(benches, bench_pipeline);
criterion_main!(benches);
```

**Step 3: Run benchmark to verify it works**

Run: `cargo bench --bench pipeline`
Expected: Benchmark runs and prints timing summary.

**Step 4: Commit**

```bash
git add crates/bankflow-core/Cargo.toml crates/bankflow-core/benches/pipeline.rs
git commit -m "bench: add core pipeline performance benchmark"
```

### Task 2: Update roadmap status note

**Files:**
- Modify: `docs/plans/ROADMAP.md`

**Step 1: Update Phase 5 status note**

Append a short note indicating perf benchmark added.

**Step 2: Verify diff**

Run: `git diff -- docs/plans/ROADMAP.md`
Expected: Only Phase 5 status line adjusted.

**Step 3: Commit**

```bash
git add docs/plans/ROADMAP.md
git commit -m "docs: note perf benchmark added"
```
