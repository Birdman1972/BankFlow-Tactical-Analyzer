# Session Log: Feedback UI + Backend Storage

**Date**: 2026-01-29
**Focus**: Feedback UI 整合（Classic/Modern）與後端雙軌儲存

---

## 1. 完成事項

### UI
- Classic UI：Header 加入「意見回饋」切換，顯示 FeedbackForm。
- Modern UI：Sidebar 加入「Home / Feedback」導覽，回饋頁面顯示 FeedbackForm。

### 前端送出流程
- `FeedbackForm` 直接串接 `feedbackService.submitFeedback`。
- 追加 queued 成功提示（離線暫存）與 i18n 文案。

### 後端儲存
- `api/feedback` 同步寫入 GitHub Issues 與 DB（可選 endpoint）。
- GitHub Issues 標題格式與標籤規則固定。

---

## 2. 文件同步

- `docs/PROJECT.md`：心智圖新增「意見回饋」。
- `docs/ARCHITECTURE.md`：新增回饋流程圖。
- `docs/plans/ROADMAP.md`：新增 Feedback 任務條目。
- `docs/USER_GUIDE.md`：補上回饋入口說明。
- `README.md`：補上回饋功能描述與隱私提醒。

---

## 3. 環境變數（後端）

- `FEEDBACK_GITHUB_TOKEN`：GitHub token
- `FEEDBACK_GITHUB_REPO`：目標 repo（預設 `Birdman1972/BankFlow-Tactical-Analyzer`）
- `FEEDBACK_DB_ENDPOINT`：DB 收件 endpoint
- `FEEDBACK_DB_TOKEN`：DB 授權 token（可選）

---

## 4. 錯誤嘗試紀錄

- 本次沒有新增失敗嘗試，因此 `docs/TRIALS.md` 未更新。

---

## 5. 下一步

- 補上實際 DB/KV 服務端整合（若有具體平台）。
- 驗證 GitHub Issues/DB 寫入權限與錯誤處理行為。

