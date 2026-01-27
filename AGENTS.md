## AGENTS.md（給自動化 coding agents）

## 全域偏好
- 回覆請使用繁體中文（台灣用語）。
- 不要自行 `git commit` / `git push`，除非使用者明確要求。

## 專案速覽
- 前端：Svelte 5 + TypeScript + Vite + TailwindCSS（`src/`）
- 桌面：Tauri 2（Rust，`src-tauri/`）
- 核心引擎：Rust crate `bankflow-core`（`crates/bankflow-core/`）
- Web/WASM：`bankflow-core` 透過 wasm-pack 編譯輸出到 `src/lib/wasm/bankflow-core-wasm/`
- Serverless API（Vercel）：`api/`（例：`api/feedback.ts`）

## Cursor / Copilot 規則
- 未找到 `.cursor/rules/`、`.cursorrules` 或 `.github/copilot-instructions.md`（若後續新增，請更新本檔）。

## 常用指令（以 repo 內現有腳本/CI 為準）

## 環境需求（開發/CI 一致性）
- Node.js：CI 用 Node 20（見 `.github/workflows/ci.yml`）
- Rust：stable toolchain；CI 會安裝 `wasm32-unknown-unknown` target
- wasm-pack：`npm run build:wasm` 會呼叫 `scripts/build-wasm.sh`；若缺少會提示 `cargo install wasm-pack`
- Linux（CI/打包）：Tauri 依賴 WebKit/GTK 等系統套件（見 `.github/workflows/ci.yml`）

### Node / 前端
- 安裝依賴（CI）：`npm ci`
- 安裝依賴（本機）：`npm install`
- Web 開發：`npm run dev`
- Tauri 開發：`npm run dev:tauri`
- 型別檢查（Svelte/TS）：`npm run check`
- Web build（只 build 前端）：`npm run build`
- WASM build（由 Rust 產出 wasm-bindgen pkg）：`npm run build:wasm`
- Web build（WASM + 前端）：`npm run build:web`
- Tauri build（桌面打包）：`npm run build:tauri`
- 預覽：`npm run preview`

備註：README 提到 `pnpm`，但 repo 目前有 `package-lock.json` 且 CI 用 `npm ci`。除非你要改成 pnpm 並同步調整鎖檔/CI，否則請優先跟 CI 一致用 npm。

### Web/WASM 建置注意事項
- `npm run build:wasm` 會整個刪掉並重建 `src/lib/wasm/bankflow-core-wasm/`（見 `scripts/build-wasm.sh`）。
- `scripts/build-wasm.sh` 支援 `--debug`（開發模式）；未帶參數為 release（最佳化）。
- Web production 會從 `/public/wasm/` 取 `.wasm`（見 `src/lib/stores/wasm-impl.ts`），所以改 Rust/WASM 後請確保 `npm run build:web` 能成功。

### Rust / Core（bankflow-core）
- 單元測試（穩定作法）：`(cd crates/bankflow-core && cargo test)`
- 跑單一測試（以名稱過濾）：`(cd crates/bankflow-core && cargo test <test_name_substring>)`
- 跑單一測試（精確比對）：`(cd crates/bankflow-core && cargo test -- --exact <test_name>)`

備註：CI 目前寫的是 `cargo test -p bankflow-core`（見 `.github/workflows/ci.yml`）。若你在本機從 repo root 執行遇到 workspace/package 相關錯誤，請改用上面「進到 crate 目錄」的方式。

### Rust / Tauri（桌面後端）
- 測試（依 `src-tauri` crate）：`(cd src-tauri && cargo test)`

### WASM 測試（wasm-pack）
- 全部 WASM tests（Node）：`(cd crates/bankflow-core && wasm-pack test --node --features wasm)`
- 全部 WASM tests（無頭瀏覽器）：`(cd crates/bankflow-core && wasm-pack test --firefox --headless --features wasm)`
- 跑特定測試檔（本 repo：`crates/bankflow-core/tests/wasm.rs` -> 測試名為 `wasm`）：
  - `wasm-pack test --node --features wasm --test wasm`
- 跑單一測試（官方語法：在 `--test <name>` 後面加過濾字串）：
  - `wasm-pack test --node --features wasm --test wasm test_transaction`
- 需要 `println!` 輸出：`wasm-pack test --node --features wasm --test wasm -- --nocapture`

參考：https://drager.github.io/wasm-pack/book/commands/test.html

### 其他常用（非 CI 固定，但通常可用）
- Rust 格式化：`(cd crates/bankflow-core && cargo fmt --all)`、`(cd src-tauri && cargo fmt --all)`
- Rust 靜態檢查：`(cd crates/bankflow-core && cargo clippy --all-targets --all-features)`（視本機 toolchain 而定）

### 建議的驗證順序（模擬 CI）
1. `npm ci`
2. `npm run check`
3. `npm run build:wasm`
4. `(cd crates/bankflow-core && cargo test)`
5. `npm run build:web`

## 重要檔案（常見修改點）
- `src/main.ts`：前端入口；平台初始化失敗時會顯示 fallback UI。
- `src/lib/stores/platform.ts`：平台抽象層（Tauri vs Web/WASM）。UI 層盡量只呼叫這裡的 API。
- `src/lib/stores/tauri-impl.ts`、`src/lib/stores/wasm-impl.ts`：各平台具體實作。
- `src/lib/stores/app.ts`：全域狀態、型別、log store、actions。
- `src-tauri/src/commands/*.rs`：Tauri command 入口（多為 `Result<..., String>`）。
- `crates/bankflow-core/src/*`：核心引擎（以 `CoreError` 做結構化錯誤）。

## 生成檔 / 不要手改
- `src/lib/wasm/bankflow-core-wasm/**`：wasm-pack 產物；請修改 Rust（`crates/bankflow-core/`）後用 `npm run build:wasm` 重新產出。
- `src/lib/wasm/bankflow-core-wasm/*.d.ts`：由 wasm-bindgen/wasm-pack 產生。
- `public/wasm/**`：部署用的 WASM 靜態資源（若有更新產物，請用建置流程產生）。
- `package-lock.json`：若不是刻意更換套件管理策略，請保持與 npm 一致。

## 開發原則（避免 AI slop）
- 變更要小、可回溯：避免混入無關重排（特別是全檔格式化/引號統一）。
- 先找既有 pattern：新增功能前先在 repo 內找相似做法（stores、services、Tauri commands）。
- 不要新增 suppression：避免新增 `as any`、`@ts-ignore`；若真有必要，限定範圍並註明原因。
- 不要偷偷改建置策略：例如從 npm 換 pnpm、從 Vite 換其他 bundler，需明確需求才做。

## 程式碼風格（依現有 codebase 慣例）

### TypeScript / Svelte
- 型別：專案 `tsconfig.json` 是 `strict: true`；新增/修改程式碼要維持型別正確，避免鬆綁。
- 引用：優先使用 `$lib/*` alias（見 `tsconfig.json` 的 paths），避免深層相對路徑。
- import type：型別 import 使用 `import type { ... } from '...'`（例：`src/lib/stores/platform.ts`）。
- 檔案結構（stores）：照 `src/lib/stores/app.ts` 的順序：Types -> Stores -> Derived Stores -> Actions。
- Store/Service 分工：
  - 狀態集中在 `src/lib/stores/*`；純邏輯/外部互動放 `src/lib/services/*`。
  - Rust 回傳 snake_case 時，TS 端先用 snake_case 型別承接，再轉成 camelCase（例：`src/lib/stores/tauri.ts`）。
- 平台隔離：UI 元件不要直接呼叫 Tauri API；透過 `src/lib/stores/platform.ts` 的統一介面。
- 命名：
  - 介面/型別/類別：PascalCase（例：`FileInfo`, `WasmPlatform`）
  - 變數/函式：camelCase（例：`addLog`, `checkForUpdates`）
  - 檔名：小寫；多字用 kebab-case（例：`tauri-impl.ts`, `wasm-impl.ts`）
  - Svelte 元件：PascalCase（例：`DropZone.svelte`）
- 分號：`.ts` 檔多數使用分號，請維持一致。
- 引號：repo 內單/雙引號混用；修改既有檔案時請跟隨該檔案現有風格，不要做全檔重排。
- Import 排序（建議）：外部套件 -> `$lib/*` -> 相對路徑；型別用 `import type`；必要時用同一行 `import { listen, type UnlistenFn } ...`（見 `src/lib/stores/tauri.ts`）。
- 瀏覽器/桌面差異：
  - 偵測平台請用 `src/lib/stores/platform.ts`；不要在 UI 直接依賴 `window.__TAURI__`。
  - UI 元件若需桌面限定功能，透過 platform 層動態 import（例：`src/App.svelte` 的 file drop）。
- 錯誤處理：
  - 前端流程性錯誤/使用者可見事件：用 `addLog(level, message)` 記錄（見 `src/lib/stores/app.ts`）。
  - 只在需要除錯時使用 `console.*`；避免把大量 debug log 留在 production path。
  - 存取 `localStorage` / `indexedDB` 前先做環境判斷（例：`src/lib/stores/router.ts`、`src/lib/services/feedbackService.ts`）。
- API（Vercel `api/`）慣例：
  - 請做輸入驗證（型別、必填欄位、長度限制），並回傳合適的 HTTP status（見 `api/feedback.ts`）。
  - CORS/Preflight：若跨域使用，需處理 `OPTIONS`（見 `api/feedback.ts`）。
- `@ts-ignore`：目前用於 `import pkg from '../../../package.json'`（見 `src/lib/services/versionService.ts`、`src/lib/components/AboutPage.svelte`）；除非必要，避免新增更多 suppression；若必須，請註明原因。

### Rust（bankflow-core / Tauri）
- 命名：型別 CamelCase、函式/模組/檔名 snake_case（Rust 慣例）。
- 核心錯誤：核心 crate 使用 `thiserror` 的 `CoreError`（`crates/bankflow-core/src/error.rs`）；核心層以 `Result<_, CoreError>` 傳遞錯誤。
- Tauri 指令錯誤：`src-tauri/src/commands/*.rs` 多以 `Result<_, String>` 回傳；把底層錯誤轉成使用者可理解的訊息（見 `src-tauri/src/commands/file_ops.rs`）。
- 錯誤轉換：核心層用結構化錯誤，進到 command 層再轉為字串訊息；避免在 command 層洩漏底層 debug 細節（除非是開發模式）。
- Serde 欄位：對外 API 若遇到 snake/camel 不一致，用 `#[serde(rename = "...")]` 明確對應（例：`src-tauri/src/core/whois.rs`）。
- 測試：
  - 單元測試通常放在同檔 `#[cfg(test)] mod tests { ... }`（例：`crates/bankflow-core/src/matcher.rs`、`src-tauri/src/core/whois.rs`）。
  - async 測試使用 `#[tokio::test]`（例：`src-tauri/src/core/whois.rs`）。
- 外部連線：Whois 會打外部 API（見 `src-tauri/src/core/whois.rs`）；避免在預設流程中偷偷啟用/新增更多外連。

## 安全與隱私
- 本工具設計為離線分析；如需外連（Whois），請保持「明確開啟」的使用者同意流程（參考 README 與 `src-tauri/src/core/whois.rs` 的 OpSec 註解）。
- 不要把使用者檔案路徑、帳號、IP、交易內容寫入永久儲存或外部服務（除非功能需求明確要求且有遮罩/同意）。

## PR / 變更策略（給 agent）
- 小步、最小改動；避免夾帶無關重排（尤其是引號/格式全檔變更）。
- 新增依賴前先確認必要性；優先使用既有依賴與現成 pattern。
- Web/WASM 相關改動：記得同步檢查 `npm run build:web` 與 core tests。
