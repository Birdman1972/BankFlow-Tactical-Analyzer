# Export Format + Parsing Fix Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Align Excel export format with `docs/USER_MANUAL_zh-TW.md`, fix header-based parsing to restore IP matching stats, and generate 20MB/50MB test fixtures for performance validation.

**Architecture:** Add header-aware column mapping in `parser.rs` (File A/B) with safe fallbacks. Update `exporter.rs` to emit the documented columns (Timestamp, Account, Income, Expense, Matched IP, IP Country, IP ISP, Raw Columns...). Add tests that read the exported workbook to validate headers and raw-column inclusion. Provide a fixture generator script to produce large test files.

**Tech Stack:** Rust (calamine, rust_xlsxwriter), Node/Python for fixture generation, Vite/WASM runtime.

---

### Task 1: Define header mapping behavior (tests first)

**Files:**
- Modify: `crates/bankflow-core/src/parser.rs`
- Test: `crates/bankflow-core/tests/parser_headers.rs`

**Step 1: Write failing tests**

```rust
use bankflow_core::parser::header_map::{map_file_a_columns, map_file_b_columns};

#[test]
fn file_a_header_mapping_prefers_named_columns() {
    let headers = vec![
        "交易序號", "帳號", "客戶姓名", "交易時間", "交易類型",
        "身分證/統編", "交易摘要", "交易後餘額", "支出金額", "存入金額",
    ];
    let map = map_file_a_columns(&headers);
    assert_eq!(map.timestamp, 3);
    assert_eq!(map.account, 1);
    assert_eq!(map.expense, 8);
    assert_eq!(map.income, 9);
}

#[test]
fn file_b_header_mapping_prefers_named_columns() {
    let headers = vec!["登入序號", "帳號", "登入時間", "IP位址", "裝置資訊", "登入地區"];
    let map = map_file_b_columns(&headers);
    assert_eq!(map.timestamp, 2);
    assert_eq!(map.account, 1);
    assert_eq!(map.ip_address, 3);
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test -p bankflow-core --test parser_headers -v`
Expected: FAIL (functions not found or incorrect mapping)

**Step 3: Implement minimal mapping code**

- Add a small `header_map` module in `parser.rs` (or new `parser_headers.rs`) that:
  - normalizes header text (trim, lowercase)
  - finds index by known aliases
  - falls back to current default indices if not found

**Step 4: Run tests to verify pass**

Run: `cargo test -p bankflow-core --test parser_headers -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/bankflow-core/src/parser.rs crates/bankflow-core/tests/parser_headers.rs
git commit -m "test: add header mapping tests for File A/B"
```

---

### Task 2: Use header mapping in actual parsing (tests first)

**Files:**
- Modify: `crates/bankflow-core/src/parser.rs`
- Test: `crates/bankflow-core/tests/parser_parsing.rs`

**Step 1: Write failing tests**

Create a tiny in-memory XLSX (rust_xlsxwriter) with header row + 1 data row for File A and File B, then verify:
- parsed timestamp/account/ip are correct based on header names

**Step 2: Run tests to verify failure**

Run: `cargo test -p bankflow-core --test parser_parsing -v`
Expected: FAIL

**Step 3: Implement parsing using mapped indices**

- Read first row headers
- Use mapping for indices
- Keep existing `skip(1)` behavior

**Step 4: Run tests to verify pass**

Run: `cargo test -p bankflow-core --test parser_parsing -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/bankflow-core/src/parser.rs crates/bankflow-core/tests/parser_parsing.rs
git commit -m "fix: parse File A/B by header mapping"
```

---

### Task 3: Align export format with USER_MANUAL (tests first)

**Files:**
- Modify: `crates/bankflow-core/src/exporter.rs`
- Test: `crates/bankflow-core/tests/exporter_format.rs`
- Modify (if needed): `docs/USER_MANUAL_zh-TW.md` (only if mismatch found)

**Step 1: Write failing tests**

Test that the first row headers in exported workbook match:
`Timestamp, Account, Income, Expense, Matched IP, IP Country, IP ISP, Raw Columns...`
And that raw columns are appended (count = tx.raw_columns.len()).

**Step 2: Run tests to verify failure**

Run: `cargo test -p bankflow-core --test exporter_format -v`
Expected: FAIL (headers mismatch, missing raw columns)

**Step 3: Implement exporter changes**

- Replace header set to match USER_MANUAL order and names
- Append raw columns dynamically
- Ensure Income/Expense columns align to spec (Income before Expense)

**Step 4: Run tests to verify pass**

Run: `cargo test -p bankflow-core --test exporter_format -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/bankflow-core/src/exporter.rs crates/bankflow-core/tests/exporter_format.rs
git commit -m "fix: align Excel export format with user manual"
```

---

### Task 4: Generate 20MB/50MB test fixtures (script + verify)

**Files:**
- Create: `scripts/generate-fixtures.py`
- Output: `tmp_test_FileA_20mb.xlsx`, `tmp_test_FileB_20mb.xlsx`, `tmp_test_FileA_50mb.xlsx`, `tmp_test_FileB_50mb.xlsx`

**Step 1: Write generator script**

- Create File A with required headers (per user manual)
- Create File B with required headers
- Use repeated rows to reach target sizes
- Ensure some matching accounts + timestamps within ±3 seconds for IP matching

**Step 2: Run script to generate files**

Run: `python3 scripts/generate-fixtures.py`
Expected: files created with sizes near 20MB/50MB

**Step 3: Quick sanity check**

- Read first row headers via a small Python snippet
- Confirm row counts and headers

**Step 4: Commit (optional, if script only)**

```bash
git add scripts/generate-fixtures.py
git commit -m "chore: add fixture generator for performance testing"
```

---

### Task 5: Web end-to-end test (manual + browser)

**Steps:**
1. Build web (`npm run build:web`) and deploy (or local preview)
2. Load File A/B fixtures
3. Run analysis with:
   - IP cross-reference ON
   - WHOIS ON (if supported in web)
4. Export Excel and verify:
   - IP match count
   - Multi-IP count
   - WHOIS count

---

Plan complete.
