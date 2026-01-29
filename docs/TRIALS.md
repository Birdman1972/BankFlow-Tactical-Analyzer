# 🚫 Trials & Mistakes Knowledge Base (試錯紀錄庫)

> **Protocol**: Before attempting any complex fix or architectural change, **YOU MUST READ THIS FILE**.
> **Purpose**: To prevent "Groundhog Day" loops where different AI models repeat the same failed strategies.

---

## 🛑 Architectural Constraints (絕對限制)

這些是經過驗證的硬限制，**不要嘗試挑戰**：

### 1. WASM 內的 SystemTime

- **限制**: 瀏覽器 WASM 環境 **不支援** Rust 標準庫的 `SystemTime::now()`。
- **後果**: 呼叫它會導致 WASM 在執行時期 Panic (`RuntimeError: unreachable`)。
- **正確解法**:
  - 使用 `js-sys::Date::new_0()`。
  - 或在 `Cargo.toml` 中啟用相關 crate 的 `wasm` feature (例如 `rust_xlsxwriter` 的 `wasm` feature)。
  - **不要** 嘗試自己寫 conditional compilation 去硬繞 `SystemTime`，除非你是寫底層 crate。

### 2. Tauri IPC & Browser Mode

- **限制**: 在純瀏覽器環境 (`npm run dev`)，`window.__TAURI_INTERNALS__` 不存在。
- **錯誤**: 直接呼叫 `invoke` 會導致 `undefined is not an object`。
- **正確解法**: 必須檢查環境並提供 **Mock Mode** (回傳假資料)，而不是讓 UI 崩潰。

---

## 🧪 Failed Attempts Log (失敗嘗試紀錄)

記錄那些「看起來可行但實際上會失敗」的方法。

### [2026-01-27] WASM Excel Export Panic

- **Context**: 嘗試在瀏覽器端匯出 Excel 報表。
- **Attempt**: 直接編譯 `rust_xlsxwriter` 並 deploy 到 Vercel。
- **Error**: `panicked at library/std/src/sys/unsupported/time.rs: SystemTime not implemented on this platform`.
- **Why it failed**: `rust_xlsxwriter` 預設依賴 std time 用於寫入 Excel metadata (建立時間)，這在 WASM (unknown-unknown) 是不支援的。
- **Solution**: 在 `crates/bankflow-core/Cargo.toml` 開啟 `rust_xlsxwriter = { version = "...", features = ["wasm"] }`。

### [2026-01-26] Drag & Drop Crash

- **Context**: 實作檔案拖曳功能。
- **Attempt**: 直接在 `DropZone.svelte` 使用 Tauri 的 `listen` API 來監聽拖曳。
- **Error**: 在瀏覽器測試時直接白屏 Crash。
- **Why it failed**: 沒有做環境判斷 (Tauri Guard)。
- **Solution**: 實作 `handleTauriError` wrapper，在非 Tauri 環境自動切換為 Mock 行為。

---

## 📝 Pending Hypotheses (待驗證假說)

- [ ] **Windows Path Separator**: 目前假設 Rust `PathBuf` 能完全處理 `\` 與 `/`，但尚未在真實 Windows 機器上驗證深層中文路徑的相容性。

---

> **Note to Agent**: If you try something and it fails, **LOG IT HERE** before you rollback or try a new approach.
