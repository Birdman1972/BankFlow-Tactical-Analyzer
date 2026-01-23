# BankFlow Tactical Analyzer - Claude 指引

---

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

**階段**：Phase 7 WASM 架構 - 完成 ✅

**已完成**：
- [x] 專案文件 (README, docs/)
- [x] 架構設計 (Tauri + Rust + Svelte)
- [x] 開發計畫 (docs/plans/)
- [x] **Phase 1：專案初始化** (2026-01-21)
- [x] **Phase 2：Rust 核心引擎** (2026-01-21)
- [x] **Phase 3：Tauri 命令層** (2026-01-21)
- [x] **Phase 4：Svelte 前端** (2026-01-21)
- [x] **Phase 7：WASM 雙平台架構** (2026-01-23)
  - [x] bankflow-core crate (WASM 相容)
  - [x] src-tauri 使用 bankflow-core
  - [x] 前端平台切換機制 (platform.ts)
  - [x] WASM 建置腳本與編譯測試

**跳過的階段**：
- Phase 5 (測試) - 待功能完成後補做
- Phase 6 (打包) - 待功能完成後執行

---

## 雙平台架構

```
┌─────────────────────────────────────────────────────────┐
│                    Svelte Frontend                       │
│              (src/lib/stores/platform.ts)               │
└──────────────────────┬──────────────────────────────────┘
                       │
         ┌─────────────┴─────────────┐
         │                           │
         ▼                           ▼
┌─────────────────┐         ┌─────────────────┐
│  Tauri Desktop  │         │    Web/WASM     │
│  (tauri-impl)   │         │  (wasm-impl)    │
└────────┬────────┘         └────────┬────────┘
         │                           │
         ▼                           ▼
┌─────────────────┐         ┌─────────────────┐
│  bankflow-core  │         │  bankflow-core  │
│   (native Rust) │         │    (WASM)       │
└─────────────────┘         └─────────────────┘
```

---

## 檔案結構

```
crates/bankflow-core/        # 共用核心 (WASM 相容)
├── src/
│   ├── lib.rs, models.rs, parser.rs, matcher.rs
│   ├── processor.rs, exporter.rs, error.rs
│   └── wasm.rs              # WASM bindings

src-tauri/                   # Tauri 桌面版
├── src/
│   ├── core/mod.rs          # 重新導出 bankflow-core
│   ├── core/whois.rs        # Tauri 專用 (網路請求)
│   └── commands/, models/, state.rs

src/lib/stores/              # 前端平台抽象
├── platform.ts              # 統一 API + 偵測
├── tauri-impl.ts            # Tauri 實現
├── wasm-impl.ts             # WASM 實現
└── app.ts                   # 應用狀態

src/lib/wasm/                # WASM 輸出
└── bankflow-core-wasm/      # wasm-pack build 輸出
```

---

## 快速指令

**開發**：
- `npm run dev` - Vite 開發伺服器 (Web)
- `npm run dev:tauri` - Tauri 開發模式

**建置**：
- `npm run build:wasm` - 編譯 WASM 模組
- `npm run build:web` - 編譯 Web 版本
- `npm run build:tauri` - 打包桌面應用

**檢查**：
- `npm run check` - TypeScript 型別檢查
