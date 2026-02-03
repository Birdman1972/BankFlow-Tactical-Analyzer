# Integration Tests Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a core integration test that exercises the parse -> match -> process -> export pipeline using real fixtures.

**Architecture:** Add a new Rust test file under `crates/bankflow-core/tests/` that reads fixture XLSX files, runs the full pipeline, and validates that results and exported workbook structure are sane. Keep the test self-contained with a small helper to load fixture bytes.

**Tech Stack:** Rust, `calamine` (read workbook), `rust_xlsxwriter` (export), core modules (`Parser`, `IpMatcher`, `Processor`, `Exporter`).

### Task 1: Add integration pipeline test

**Files:**
- Create: `crates/bankflow-core/tests/integration_flow.rs`

**Step 1: Write the failing test**

```rust
use bankflow_core::{Exporter, IpMatcher, Parser, Processor};
use calamine::{open_workbook_auto_from_rs, Reader};
use std::io::Cursor;

fn read_fixture_bytes(_name: &str) -> Vec<u8> {
    unimplemented!("fixture loader");
}

#[test]
fn integration_pipeline_exports_workbook() {
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
    let exported = Exporter::export_to_bytes(&transactions, &income, &expense).expect("export");

    let cursor = Cursor::new(exported);
    let mut workbook = open_workbook_auto_from_rs(cursor).expect("open export");
    let sheets = workbook.sheet_names().to_vec();
    assert!(sheets.contains(&"Summary".to_string()));
    assert!(sheets.contains(&"Income".to_string()));
    assert!(sheets.contains(&"Expense".to_string()));
    assert!(sheets.contains(&"Counterparty".to_string()));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test integration_flow`
Expected: FAIL with `unimplemented!` panic or missing fixture loader.

**Step 3: Write minimal implementation**

```rust
fn read_fixture_bytes(name: &str) -> Vec<u8> {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("fixtures")
        .join(name);
    std::fs::read(base).expect("read fixture")
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test --test integration_flow`
Expected: PASS (1 test).

**Step 5: Commit**

```bash
git add crates/bankflow-core/tests/integration_flow.rs
git commit -m "test: add core integration pipeline test"
```

### Task 2: Update roadmap status (optional but recommended)

**Files:**
- Modify: `docs/plans/ROADMAP.md`

**Step 1: Update Phase 5 progress line**

Change Phase 5 status note to reflect integration test coverage started.

**Step 2: Verify diff is correct**

Run: `git diff -- docs/plans/ROADMAP.md`
Expected: Only Phase 5 status line adjusted.

**Step 3: Commit**

```bash
git add docs/plans/ROADMAP.md
git commit -m "docs: note integration test coverage started"
```
