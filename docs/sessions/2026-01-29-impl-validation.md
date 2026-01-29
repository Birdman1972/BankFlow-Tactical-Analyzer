# Session Log: Implement Resilient File Validation

Date: 2026-01-29
Time: Morning Session

## 1. Objective

實作「強固檔案驗證 (Resilient File Validation)」機制，防止使用者載入格式錯誤的檔案導致應用程式崩潰或行為異常。

## 2. Changes

### Core (Rust)

- **File**: `crates/bankflow-core/src/parser.rs`
- **Action**:
  - 移除原本寬鬆的 `unwrap_or` 邏輯。
  - 實作 `header_map::validate_file_a_headers` 與 `validate_file_b_headers`。
  - 若缺少必要欄位 (如 `交易時間`, `帳號`)，直接回傳 `Result::Err` 並列出缺少的欄位名稱。

### State (TypeScript)

- **Files**:
  - `src/lib/stores/app.ts`: 擴充 `FileInfo` 介面，新增 `isValid: boolean` 與 `validationError?: string`。
  - `src/lib/stores/tauri-impl.ts` & `src/lib/stores/wasm-impl.ts`: 在載入檔案時捕捉 Rust 回傳的錯誤，並將其填入 `validationError`，而非僅是 console.log。

### UI (Svelte)

- **File**: `src/lib/components/DropZone.svelte`
- **Action**:
  - 新增錯誤狀態樣式 (Red Border + Background)。
  - 顯示明確的錯誤訊息 (e.g., "缺少欄位: 交易時間/timestamp")。
  - 修正了 HTML 結構 (Nested Button 問題)。

## 3. Results

- **Pass**: 成功攔截無效檔案，UI 顯示紅色警告。
- **Pass**: 有效檔案正常載入。
- **Pass**: `ControlPanel` 在檔案無效時鎖定「執行分析」按鈕。

## 4. Next Steps

- [ ] 準備 v0.3.9 發布 (Changelog, Tag)。
- [ ] (v0.4.0) 實作 "Smart Repair" 功能，允許使用者在 UI 上手動映射欄位。

## 5. Protocol Compliance

- [x] Task Updated (`task.md`)
- [x] Protocol Documented (`README.md`)
