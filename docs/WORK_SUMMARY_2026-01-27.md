# BankFlow Tactical Analyzer - 工作摘要

**日期**: 2026-01-27  
**Commit**: `df0c4d2`  
**Branch**: `main`  

---

## 今日完成事項總覽

| # | 任務 | 狀態 |
|---|------|------|
| 1 | 收集 context：fixtures/generators、masking 行為、版本更新流程、桌面打包配置 | ✅ 完成 |
| 2 | 凍結 XLSX schema + masking 規則、定義成功標準 + 測試矩陣 | ✅ 完成 |
| 3 | 建立 canonical fixture generator + golden expected outputs + 本地驗證入口 | ✅ 完成 |
| 4 | 修復 Streamlit main.py：masking 邏輯、IP 匹配健壯性、counterparty 匯出 | ✅ 完成 |
| 5 | 審計 v2 Rust core masking vs header mapping | ✅ 完成 |
| 6 | 審計 desktop (Tauri) macOS/Windows 程式碼路徑及打包配置 | ✅ 完成 |
| 7 | 統一版本來源 (web + desktop)，實作更新檢查 + ignore-version 持久化 | ✅ 完成 |
| 8 | 打包 desktop builds + 新增 web 下載頁面 | ✅ 完成 |
| 9 | 執行驗證：tests/builds + fixture-based regression | ✅ 完成 |

---

## 詳細變更紀錄

### 1. Rust Core (`crates/bankflow-core/`)

#### `src/exporter.rs` (+97 行)
- **新增 Counterparty Unique List 功能**
  - 新增 `export_counterparty_list()` 函數
  - 從所有交易中提取唯一交易對手名稱
  - 支援匯出為獨立 sheet 或 CSV

#### `src/bin/verify_fixtures.rs` (新檔案)
- 本地驗證入口點
- 可執行 fixture 驗證而不需完整 build

#### `tests/exporter_counterparty.rs` (新檔案)
- Counterparty 功能的單元測試
- 驗證唯一性和排序邏輯

### 2. Streamlit Python (`main.py`) (+214/-60 行)

#### Masking 修正
- 收入/支出 sheets 現在正確套用 masking
- 修復 C/F/L/M 欄位語義對應

#### IP 匹配健壯性
- 移除 blanket "Invalid Data" 錯誤
- 改為逐筆檢查，提供精確錯誤訊息
- 支援多種 IP 格式（純數字、帶前綴等）

#### Counterparty 匯出
- 新增「交易對手清單」匯出選項
- 整合至現有下載流程

### 3. Web UI (Svelte)

#### `src/lib/components/DownloadsDialog.svelte` (新檔案，152 行)
- 全新下載對話框元件
- 連結 GitHub Releases 頁面
- 顯示最新版本資訊 + changelog
- 直接下載 Windows MSI / Portable ZIP
- macOS 說明（目前無自動簽章）

#### `src/ClassicPage.svelte` & `src/ModernPage.svelte`
- 整合 DownloadsDialog 觸發按鈕
- 統一兩種 UI 模式的下載體驗

#### `src/lib/services/versionService.ts`
- 統一版本來源：`/public/version.json`
- 支援 web 和 desktop 共用同一版本資訊
- 新增 `getLatestVersion()` API

### 4. i18n 翻譯

#### `src/lib/i18n/locales/en.ts` & `zh-TW.ts`
新增以下 key：
```
downloadsDialog.title
downloadsDialog.latestVersion
downloadsDialog.loading
downloadsDialog.loadError
downloadsDialog.openReleases
downloadsDialog.downloadWindowsMsi
downloadsDialog.downloadPortableZip
downloadsDialog.macosNote
downloadsDialog.releaseNotes
downloadsDialog.close
```

#### `src/lib/i18n/types.ts`
- 更新 TranslationKeys 類型定義

### 5. Desktop (Tauri)

#### `src-tauri/Cargo.toml`
- 版本更新至 `0.3.4`

#### `src-tauri/src/lib.rs`
- 微調初始化流程
- 確保 macOS/Windows 路徑一致性

### 6. 文件

#### `docs/desktop-audit.md` (新檔案)
- Tauri 桌面應用程式審計報告
- macOS/Windows 程式碼路徑分析
- 打包配置說明
- 已知問題與建議

#### `docs/fixtures-spec.md` (新檔案)
- XLSX 測試檔案規格書
- Masking 規則定義 (C/F/L/M)
- 欄位語義對照表
- 測試矩陣

### 7. Fixture 系統

#### `scripts/generate-fixtures.py` (重構)
- 產生標準測試 XLSX 檔案
- 包含各種邊界案例

#### `scripts/verify-fixtures.py` (新檔案)
- 驗證處理結果與預期輸出
- 可作為 CI 驗證步驟

### 8. 其他

#### `public/version.json` (新檔案)
```json
{
  "version": "0.3.4",
  "releaseDate": "2026-01-27",
  "changelog": [...]
}
```

#### `pyrightconfig.json` (新檔案)
- Python 類型檢查配置
- 改善 IDE 支援

---

## 明日可接續工作

### 優先順序 1 - 驗證 Vercel 部署
- [ ] 確認 Vercel 已自動部署最新 commit
- [ ] 測試 DownloadsDialog 在線上環境正常運作
- [ ] 確認 `/version.json` 可正確載入

### 優先順序 2 - Desktop 發布
- [ ] 建立 GitHub Release `v0.3.4`
- [ ] 上傳 Windows MSI 安裝檔
- [ ] 上傳 Portable ZIP
- [ ] 測試下載連結是否正確指向 artifacts

### 優先順序 3 - macOS 簽章（可選）
- [ ] 評估 Apple Developer Program 費用
- [ ] 若決定簽章，設定 CI/CD 自動化

### 優先順序 4 - 功能增強（待討論）
- [ ] Counterparty 清單在 Web UI 的視覺化
- [ ] 批次處理多檔案
- [ ] 歷史紀錄功能

---

## 技術債 / 已知問題

| 問題 | 嚴重度 | 說明 |
|------|--------|------|
| macOS 無簽章 | 中 | 使用者需手動允許執行 |
| WASM 檔案較大 (~1.6MB) | 低 | 可考慮 wasm-opt 優化 |
| Python 版本相依 | 低 | Streamlit 需 Python 3.9+ |

---

## 相關檔案快速連結

- **下載對話框**: `src/lib/components/DownloadsDialog.svelte`
- **版本服務**: `src/lib/services/versionService.ts`
- **Rust 匯出器**: `crates/bankflow-core/src/exporter.rs`
- **Fixture 規格**: `docs/fixtures-spec.md`
- **桌面審計**: `docs/desktop-audit.md`

---

*此文件由 AI 助理自動產生，供明日檢視與接續工作使用。*
