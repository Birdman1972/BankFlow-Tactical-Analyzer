# Feedback Link Attachments Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 在意見回饋表單加入多行連結欄位，允許使用者附上檔案連結（不做檔案上傳），並在 GitHub Issue/DB 中保存。

**Architecture:** 前端表單新增多行 URL 欄位並傳入 `feedbackService.submitFeedback`；後端 `api/feedback` 驗證並將連結寫入 Issue 內容與 DB payload。僅接受 `http/https` 連結。

**Tech Stack:** Svelte 5, TypeScript, Vercel Serverless (`@vercel/node`), GitHub Issues API

---

### Task 1: 前端 UI 欄位與 Payload 擴充

**Files:**
- Modify: `src/lib/components/FeedbackForm.svelte`
- Modify: `src/lib/services/feedbackService.ts`
- Modify: `src/lib/i18n/types.ts`
- Modify: `src/lib/i18n/locales/en.ts`
- Modify: `src/lib/i18n/locales/zh-TW.ts`

**Step 1: 寫失敗驗證（手動）**

- 空白連結欄位應允許送出
- 有內容時，若含非 http/https 連結則顯示錯誤

**Step 2: UI 新增多行欄位**

- 新增 `attachments` 多行輸入（以換行分隔）
- 驗證每行是 `http://` 或 `https://`

**Step 3: Payload 擴充**

- `FeedbackPayload` 增加 `attachments?: string[]`
- `feedbackService.validatePayload` 加入連結驗證

**Step 4: i18n 文案**

- 標籤、Placeholder、錯誤訊息（含「僅支援連結，不支援上傳」）

---

### Task 2: 後端 API 儲存連結

**Files:**
- Modify: `api/feedback.ts`

**Step 1: Payload 驗證**

- `attachments` 可選
- 每個連結必須為 `http/https`

**Step 2: Issue 內容加入連結**

- Issue body 新增「Attachments」段落

**Step 3: DB payload 加入連結**

- DB 寫入時包含 `attachments`

---

### Task 3: 文件同步

**Files:**
- Modify: `README.md`
- Modify: `docs/USER_GUIDE.md`
- Modify: `docs/ARCHITECTURE.md`
- Modify: `task.md`
- Modify: `docs/sessions/YYYY-MM-DD-*.md`

**Step 1: 使用者指南補充**

- 說明回饋僅支援連結，不支援檔案上傳

**Step 2: 架構圖補充**

- 註記「Attachments (links only)」

**Step 3: 更新任務與 Session Log**

- 勾選完成項目

---

Plan complete.
