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

## 目前狀態

**階段**：文件完成 ✅，準備開始實作

**已完成**：
- [x] 專案文件 (README, docs/)
- [x] 架構設計 (Tauri + Rust + Svelte)
- [x] 開發計畫 (docs/plans/)

**下一步**：
- [ ] 建立 Tauri + Svelte 專案結構
- [ ] 實作 Rust 核心引擎
- [ ] 產生測試用 Excel 檔案
- [ ] 實作前端 UI 元件

---

## 快速指令

繼續開發請說：「繼續 BankFlow 專案」
