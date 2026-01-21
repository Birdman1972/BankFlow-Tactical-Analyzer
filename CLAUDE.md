# BankFlow Tactical Analyzer - Claude 指引

## Guardian Recursion Protocol (守護神連鎖)

**規則**：禁止暴力試錯 (No Brute-Force Retry)

若測試/建構連續失敗 **5 次**，必須強制進入 **Defense Mode**：

1. **Stop** - 立即停止目前的修復嘗試
2. **Debug** - 執行深度分析找出根本原因
3. **Brainstorm** - 列出替代方案
4. **Re-Test** - 修復後再次測試
5. **Rollback** - 若上述皆失效，`git reset --hard` 並回報

---

## Documentation First Protocol (文件優先原則)

**規則**：專案變動時，文件更新為首要任務

任何程式碼變更後，**必須**同步更新：

1. **心智圖** - `docs/PROJECT.md` 中的 Mermaid mindmap
2. **架構圖** - `docs/ARCHITECTURE.md` 中的流程圖與模組圖
3. **時程規劃** - `docs/plans/ROADMAP.md` 中的 Gantt chart
4. **狀態追蹤** - 本檔案 (CLAUDE.md) 的進度狀態

```
程式碼變更 → 文件更新 → Git Commit
     ❌ 不可跳過文件更新步驟
```

---

## 目前狀態

**階段**：Phase 7 WASM 架構 - 進行中 🔄

**已完成**：
- [x] 專案文件 (README, docs/)
- [x] 架構設計 (Tauri + Rust + Svelte)
- [x] 開發計畫 (docs/plans/)
- [x] **Phase 1：專案初始化** (2026-01-21)
- [x] **Phase 2：Rust 核心引擎** (2026-01-21)
- [x] **Phase 3：Tauri 命令層** (2026-01-21)
- [x] **Phase 4：Svelte 前端** (2026-01-21)
- [x] **Phase 7A：bankflow-core crate** (2026-01-21) - 部分完成
  - [x] 建立 `crates/bankflow-core/` 目錄結構
  - [x] Cargo.toml (含 wasm feature flag)
  - [x] models.rs - NaiveDateTime (WASM 相容)
  - [x] parser.rs - bytes 輸入 (WASM 相容)
  - [x] matcher.rs - 移除 rayon (WASM 相容)
  - [x] processor.rs - 資料前處理
  - [x] exporter.rs - bytes 輸出 (WASM 相容)
  - [x] wasm.rs - wasm-bindgen 封裝
  - [x] error.rs - 錯誤類型
  - [x] lib.rs - 模組匯出

**Phase 7 剩餘工作**：
- [ ] 更新 src-tauri/Cargo.toml 依賴 bankflow-core
- [ ] 修改 src-tauri 使用 bankflow-core (移除重複程式碼)
- [ ] 建立前端平台切換機制 (src/lib/stores/platform.ts)
- [ ] 建立 WASM 建置腳本 (wasm-pack)
- [ ] 測試 WASM 編譯

**跳過的階段**：
- Phase 5 (測試) - 待 WASM 架構完成後補做
- Phase 6 (打包) - 待功能完成後執行

---

## 檔案結構 (Phase 7 新增)

```
crates/bankflow-core/
├── Cargo.toml          # 含 [features] wasm
└── src/
    ├── lib.rs          # 模組匯出
    ├── error.rs        # CoreError
    ├── models.rs       # Transaction, IpRecord (NaiveDateTime)
    ├── parser.rs       # Parser::parse_*_from_bytes()
    ├── matcher.rs      # IpMatcher (無 rayon)
    ├── processor.rs    # Processor
    ├── exporter.rs     # Exporter::export_to_bytes()
    └── wasm.rs         # #[wasm_bindgen] analyze(), export_excel()
```

---

## 快速指令

繼續 WASM 架構：「繼續 Phase 7 WASM」

完成剩餘步驟：
1. 更新 src-tauri 依賴
2. 前端平台切換
3. WASM 編譯測試
