# Memory Usage Tests Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a simple memory-usage verification tool for the core pipeline and document it in the roadmap.

**Architecture:** Add a small Rust binary `verify_memory` that runs the parse -> match -> process -> export pipeline on fixtures and reports peak RSS via `getrusage`. Keep it informational (no brittle assertions).

**Tech Stack:** Rust, `libc` (rusage), core modules (`Parser`, `IpMatcher`, `Processor`, `Exporter`).

### Task 1: Add verify_memory binary

**Files:**
- Modify: `crates/bankflow-core/Cargo.toml`
- Create: `crates/bankflow-core/src/bin/verify_memory.rs`

**Step 1: Add libc dependency**

```toml
[dependencies]
libc = "0.2"
```

**Step 2: Add verify_memory binary**

```rust
use bankflow_core::{Exporter, IpMatcher, Parser, Processor};
use std::io::Cursor;

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
```

**Step 3: Run binary to verify it works**

Run: `cargo run --bin verify_memory`
Expected: Prints `Peak RSS: ... KB`.

**Step 4: Commit**

```bash
git add crates/bankflow-core/Cargo.toml crates/bankflow-core/src/bin/verify_memory.rs
 git commit -m "tool: add memory usage verifier"
```

### Task 2: Update roadmap status note

**Files:**
- Modify: `docs/plans/ROADMAP.md`

**Step 1: Update Phase 5 status note**

Append a short note indicating memory usage check added.

**Step 2: Commit**

```bash
git add docs/plans/ROADMAP.md
git commit -m "docs: note memory check added"
```
