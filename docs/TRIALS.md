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

### [2026-01-29] Fixture Filenames

- **Context**: Using `generate_tc_fixtures.py`.
- **Error**: `os error 2` due to guessing filenames like `_FileA_small.xlsx`.
- **Fact**: The script outputs `[prefix]_A.xlsx` and `[prefix]_B.xlsx`. It does NOT include `_small` or `_File` in the filename.
- **Solution**: Always check `ls` or script output before running dependent commands.

### [2026-01-29] Vercel Runtime Error (null is not an object)

- **Context**: Cyberpunk Light theme implementation.
- **Attempt**: Used `<svelte:head><html ... /></svelte:head>` in `App.svelte` to set data-theme.
- **Error**: Runtime crash: `null is not an object (evaluating 's.cloneNode')`.
- **Why it failed**: Svelte template engine cannot safely clone/hydrate the root `<html>` tag if injected via component template.
- **Solution**: Use reactive script logic: `$: document.documentElement.setAttribute(...)`.

### [2026-01-29] Cargo Run in Monorepo & CWD

- **Context**: Running internal tools (`generate_report`) from root.
- **Attempt**: `cargo run --manifest-path ... --bin generate_report -- --file-a ../../test.xlsx`
- **Error**: `os error 2` (File not found).
- **Why it failed**: using `--manifest-path` does NOT change the Current Working Directory (CWD). It remains at the root where `cargo` was invoked.
- **Solution**: Use paths relative to the _invocation directory_ (e.g., just `test.xlsx`), not relative to the crate directory.

### [2026-01-29] Git Worktree Branch Creation Failed

- **Context**: Creating a new worktree for `feature/feedback-ui`.
- **Attempt**: `git worktree add .worktrees/feedback-ui -b feature/feedback-ui`
- **Error**: `fatal: cannot lock ref 'refs/heads/feature/feedback-ui': unable to create directory for .git/refs/heads/feature/feedback-ui`
- **Why it failed**: Git could not create the nested ref directory under `.git/refs/heads/feature/`. This usually indicates missing parent directory or a permission/lock issue in `.git/refs`.
- **Solution**: Create the branch namespace first or choose a flat branch name (e.g., `feedback-ui`), then retry `git worktree add`.

### [2026-01-29] Git Worktree Lock Permission Error

- **Context**: Creating a new worktree on macOS with a flat branch name.
- **Attempt**: `git worktree add .worktrees/feedback-ui -b feedback-ui`
- **Error**: `fatal: cannot lock ref 'refs/heads/feedback-ui': ... .git/refs/heads/feedback-ui.lock: Operation not permitted`
- **Why it failed**: The repository’s `.git/refs` directory is not writable in this environment, blocking branch creation.
- **Solution**: Create the branch manually outside this environment or fix permissions on `.git/refs`; then re-run `git worktree add` without `-b` to attach to the existing branch.

### [2026-01-29] Git Worktree Lock Still Failing After chmod

- **Context**: Retried worktree creation after granting write permission to `.git/refs`.
- **Attempt**: `chmod -R u+w .git/refs` then `git worktree add .worktrees/feedback-ui -b feedback-ui`
- **Error**: `fatal: cannot lock ref 'refs/heads/feedback-ui': ... .git/refs/heads/feedback-ui.lock: Operation not permitted`
- **Why it failed**: Permission issue persists even after chmod, suggesting filesystem restrictions beyond repo permissions (e.g., mount flags or sandbox limits).
- **Solution**: Create the branch outside this environment or move repo to a writable location, then create worktree.

### [2026-01-29] Git Index Lock Permission Error

- **Context**: Staging changes before commit.
- **Attempt**: `git add ...`
- **Error**: `fatal: Unable to create '.git/index.lock': Operation not permitted`
- **Why it failed**: `.git` directory is not writable in this environment (lockfile creation blocked).
- **Solution**: Fix `.git` permissions (including `.git/index` and parent) or move repo to a writable location, then retry staging.

---

## 📝 Pending Hypotheses (待驗證假說)

- [ ] **Windows Path Separator**: 目前假設 Rust `PathBuf` 能完全處理 `\` 與 `/`，但尚未在真實 Windows 機器上驗證深層中文路徑的相容性。

---

> **Note to Agent**: If you try something and it fails, **LOG IT HERE** before you rollback or try a new approach.
