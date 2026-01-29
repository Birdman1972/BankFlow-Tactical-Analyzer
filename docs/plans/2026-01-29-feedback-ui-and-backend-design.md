# Feedback UI + Backend Storage Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 在 Classic/Modern UI 提供一致的意見回饋入口，並由後端同時寫入 GitHub Issues 與 DB 作備援。

**Architecture:** 前端以單一 `feedbackService.submitFeedback` 封裝送出與離線佇列，Classic/Modern 透過統一的 Feedback 頁面/區塊呈現。後端 `api/feedback` 同步寫入 GitHub Issues 與 DB（或 KV），並做輸入驗證與 rate limit。

**Tech Stack:** Svelte 5, TypeScript, Vercel Serverless (`@vercel/node`), GitHub Issues API, DB/KV（待選）。

---

### Task 1: 明確 UI 路由與入口（Classic + Modern）

**Files:**
- Modify: `src/ClassicPage.svelte`
- Modify: `src/ModernPage.svelte`
- Modify: `src/lib/stores/router.ts`
- Modify: `src/lib/components/Sidebar.svelte`

**Step 1: 定義 UI 入口與導航行為**

- Classic：Header 加入「意見回饋」按鈕或區塊（顯示於下載旁）。
- Modern：Sidebar 導航啟用 `feedback`，顯示 Feedback 頁面。

**Step 2: 寫一個最小的切換展示**

- 使用 `currentPage`/`navigate` 切換頁面內容
- Classic 先用條件顯示 `FeedbackForm` 區塊

**Step 3: 檢查 i18n 標籤**

- 確認 `nav.feedback` 與 `feedbackForm.*` 已存在

**Step 4: 手動驗證**

- 入口可點擊，頁面切換正確

---

### Task 2: 串接 FeedbackForm → feedbackService.submitFeedback

**Files:**
- Modify: `src/lib/components/FeedbackForm.svelte`
- Modify: `src/lib/services/feedbackService.ts`
- Modify: `src/lib/i18n/locales/en.ts`
- Modify: `src/lib/i18n/locales/zh-TW.ts`

**Step 1: 寫失敗場景（手動）**

- 模擬 API 錯誤，確認 UI 顯示 error

**Step 2: 改用 service 提交**

- 在 `FeedbackForm` 引入 `submitFeedback` 並傳入 `payload`
- 顯示 queued / success / error 狀態

**Step 3: 調整錯誤訊息 i18n**

- 增加 queued / offline / network 文字

---

### Task 3: 後端儲存雙軌（GitHub Issues + DB）

**Files:**
- Modify: `api/feedback.ts`
- Create: `api/_lib/feedbackStore.ts`（若需要）
- Update: `README.md`（若條款需要補充）

**Step 1: 定義輸出格式**

- Issue 標題：`[Feedback/<type>] <title>`
- Issue 內容包含：version, platform, createdAt, description

**Step 2: 建立 GitHub Issues**

- 以 GitHub token（Serverless env）送出
- 失敗時 fallback DB-only（仍回 200/202）

**Step 3: 寫入 DB/KV**

- 寫入 payload + server timestamp

**Step 4: 手動測試**

- 模擬 API POST，確認 Issue 與 DB 都收到

---

### Task 4: 文件同步

**Files:**
- Modify: `docs/ARCHITECTURE.md`
- Modify: `docs/PROJECT.md`
- Modify: `docs/USER_GUIDE.md`
- Modify: `task.md`
- Modify: `docs/sessions/YYYY-MM-DD-*.md`

**Step 1: 更新架構圖與流程圖**

- 補上 `FeedbackForm -> /api/feedback -> GitHub Issues + DB`

**Step 2: 更新使用者指南**

- 加入回饋入口、隱私提醒

**Step 3: 更新 task 與 session**

- 將完成項目勾選

---

Plan complete.
