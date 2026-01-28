# Fixtures Spec (Canonical XLSX Schema)

This document defines the canonical Excel schemas used for local test fixtures and feature verification.

## Defaults (current app behavior)

- Hide sensitive columns: OFF
- Split income/expense: ON
- IP cross-reference: ON
- Whois lookup: OFF (desktop only)

## File A (Transactions)

Accepted: `.xlsx` / `.xls`

### Sheet

- The first sheet is used.

### Header row

- Row 1 is the header row.
- Data starts at Row 2.

### Columns (canonical)

| Col | Index | Header | Required | Used for |
|-----|-------|--------|----------|----------|
| A | 0 | 交易時間 | yes | IP matching window anchor time |
| B | 1 | 帳號 | yes | Join key with File B |
| C | 2 | 身分證字號 | no | Sensitive (removable) |
| D | 3 | 戶名 | no | Display / raw context |
| E | 4 | 幣別 | no | Display / raw context |
| F | 5 | 交易序號 | no | Sensitive (removable) |
| G | 6 | 摘要 | no | Display / raw context |
| H | 7 | 備註 | no | Display / raw context |
| I | 8 | 支出 | no | Split (expense > 0) |
| J | 9 | 存入 | no | Split (income > 0) |
| K | 10 | 餘額 | no | Display / raw context |
| L | 11 | 對方帳號 | no | Counterparty unique list (pre-mask) / Sensitive (removable) |
| M | 12 | 對方戶名 | no | Sensitive (removable) |

### Sensitive columns (C/F/L/M)

If `hideSensitive` is ON, remove the following columns from all exported sheets:

- Indices: 2, 5, 11, 12
- Headers: 身分證字號, 交易序號, 對方帳號, 對方戶名

Notes:

- Any feature that needs counterparty accounts MUST derive its results BEFORE removing these columns.

## File B (IP login records)

Accepted: `.xlsx` / `.xls`

### Sheet

- The first sheet is used.

### Header row

- Row 1 is the header row.
- Data starts at Row 2.

### Columns (canonical)

| Col | Index | Header | Required | Used for |
|-----|-------|--------|----------|----------|
| A | 0 | 登入時間 | yes | IP matching window comparison |
| B | 1 | 帳號 | yes | Join key with File A |
| C | 2 | IP位址 | yes | Matched IP output |

## IP matching window

- For each File A transaction time T, match File B records where:
  - T - 1s <= login_time <= T + 2s

## Verification matrix (fixtures)

| Fixture | Purpose | Must validate |
|--------|---------|---------------|
| `tests/fixtures/test_transactions_1000.xlsx` + `tests/fixtures/test_ip_records_1000.xlsx` | Functional baseline | split, masking, IP match formatting |
| `tmp_test_FileA_20mb.xlsx` + `tmp_test_FileB_20mb.xlsx` | Performance / stress | load, analyze, export completes without crash |
| `tmp_test_FileA_50mb.xlsx` + `tmp_test_FileB_50mb.xlsx` | Performance / stress | same as above at larger size |

### Generating fresh fixtures

Use the helper script to regenerate canonical XLSX pairs that reflect the latest business logic:

```bash
python3 scripts/generate-sample-fixtures.py \
  --rows-a 1000 \
  --rows-b 1000 \
  --out-a tests/fixtures/test_transactions_latest.xlsx \
  --out-b tests/fixtures/test_ip_records_latest.xlsx
```

- The script reuses `scripts/generate-fixtures.py` builders and automatically runs `cargo run --bin verify_fixtures` (pass `--skip-verify` if you only need the files).
- Resulting files live under `tests/fixtures/` so they are versioned despite the global `*.xlsx` ignore rule.

## Local verification entrypoint

Preferred (no Python deps):

- From repo root:
  - `cargo run --manifest-path crates/bankflow-core/Cargo.toml --bin verify_fixtures`
  - or specify custom paths:
    - `cargo run --manifest-path crates/bankflow-core/Cargo.toml --bin verify_fixtures -- --file-a tests/fixtures/test_transactions_latest.xlsx --file-b tests/fixtures/test_ip_records_latest.xlsx`
