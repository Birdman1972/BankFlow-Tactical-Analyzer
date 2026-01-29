# BankFlow Project Codex (最高指導原則)

此文件定義了 **BankFlow 金流分析器** 的開發公約與強制性協定。所有參與者 (User & Agent) 必須嚴格遵守此處定義的標準運作程序 (SOP)。

> **Standardization Note**: 本法典 (Codex) 及其定義的 `TRIALS.md` 機制為所有專案的標準配備。任何新專案必須在 `docs/` 目錄下建立對應的法典與試錯紀錄庫。

---

## Ⅰ. 核心哲學 (Core Philosophy)

1.  **Logic First, Code Later**: 在敲下任何一行程式碼前，必須先釐清邏輯。
2.  **No Guessing**: 禁止通靈 (Vibe Coding)。遇到錯誤必須查 Log、看 Context。
3.  **Fail-Safe**: 系統必須假設輸入會出錯，並提供明確的錯誤回饋 (Resilient Validation)。

---

## Ⅱ. 八大守護神協議 (The 8 Superpowers Protocol)

Agent 必須在特定情境下強制調用以下思維模式：

| 模式               | 觸發時機            | 行動準則                                              |
| :----------------- | :------------------ | :---------------------------------------------------- |
| **`[BRAINSTORM]`** | 架構抉擇 (A vs B)   | 提出 3 種方案並分析優缺點。                           |
| **`[PLAN]`**       | 複雜任務/跨檔案修改 | 先寫 Roadmap 與驗收標準，獲准後才執行。               |
| **`[DEBUG]`**      | 遇到 Bug/報錯       | **拒絕暴力試錯**。先查 Log -> 找 Root Cause -> 修復。 |
| **`[EXECUTE]`**    | 方案已定            | 逐步執行並進行 Verify-As-You-Go。                     |
| **`[REFLECT]`**    | 任務結束/自省       | 檢查是否違反 Codex？文件是否同步？                    |
| **`[TEACH]`**      | 知識落差            | 主動解釋概念或提供 Skill Tip。                        |
| **`[AUDIT]`**      | Session 啟動/結束   | 檢查檔案一致性與版控狀態。                            |
| **`[TEST]`**       | 部署前/修改後       | **No Test, No Pass**。                                |

---

## Ⅲ. 任務管理協議 (Task Management Protocol)

本專案採用 **Session-Based R-P-I Loop** 開發模型。

### 1. 文件體系

- **`task.md`**: **動態清單**。反映當下的 Todo List 狀態。
- **`docs/sessions/*.md`**: **歷史紀錄**。每個工作階段 (Sessions) 結束前必須撰寫。
- **`README.md`**: **專案入口**。包含安裝、核心功能與協定連結。

### 2. R-P-I 循環

每個功能實作必須經歷：

1.  **Research**: 讀取現有代碼，確認影響範圍。
2.  **Plan**: 更新 `task.md`，列出細部步驟。
3.  **Implement**: 執行開發，並在完成後將 `task.md` 項目打勾 (`[x]`)。

---

## Ⅳ. 錯誤處理協議 (Error Handling Protocol)

### 1. 紀錄失敗 (Knowledge Base)

- 所有非預期的技術障礙、架構錯誤或嘗試失敗，**必須** 記錄在 [`docs/TRIALS.md`](TRIALS.md)。
- **格式**: Error -> Cause -> Solution/Workaround -> Learning。

### 2. 防禦模式 (Guardian Recursion)

若 `[TEST]` 連續失敗 **5 次**，強制觸發防禦模式：

1.  **STOP**: 停止無效的修復嘗試。
2.  **CALL `[DEBUG]`**: 深度分析 Log。
3.  **CALL `[BRAINSTORM]`**: 尋找替代方案。
4.  **ROLLBACK**: 若無法解決，回滾至上一個穩定狀態。

### 3. 使用者回饋 (UI Feedback)

- **禁止靜默失敗 (No Silent Failures)**: 任何錯誤 (如檔案格式不符、網路超時) 必須在 UI 上給予明確的視覺回饋。
  - _Good_: 紅色外框 + "缺少欄位: 交易時間"。
  - _Bad_: 按了按鈕沒反應，或只在 Console 報錯。

---

## Ⅴ. 開發標準 (Coding Standards)

- **Language**: 繁體中文 (台灣)。
- **Frontend**: Svelte 5 + Tailwind CSS (Cyberpunk Theme)。
- **Backend**: Rust (Tauri Commands)。
- **Git Flow**:
  - Session 結束前必須 Commit。
  - 重大版本更新 (e.g., v0.3.9) 必須打 Tag。

---

> _此法典由 NetOps Architect Ken 與 Agent Antigravity 共同維護。_
