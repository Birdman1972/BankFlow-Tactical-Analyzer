# Tauri + Rust 遷移設計計畫

**日期**：2026-01-20
**版本**：2.0.0
**狀態**：規劃中

---

## 目標

將 BankFlow Tactical Analyzer 從 Python/Streamlit Web 版本遷移至 Tauri + Rust + Svelte 桌面應用程式，以實現：

1. **更高效能**：Rust 原生運算，處理大量資料
2. **更好的部署體驗**：單一 .exe 檔案，免安裝 Python 環境
3. **更佳的擴充性**：Svelte 元件化架構，便於新增功能
4. **更強的安全性**：Rust 記憶體安全保證

---

## 架構決策

### 技術選型

| 決策項目 | 選擇 | 理由 |
|----------|------|------|
| 桌面框架 | Tauri 2.0 | 輕量、原生效能、Rust 整合 |
| 前端框架 | Svelte 5 | 編譯時優化、元件化、擴充性佳 |
| 後端語言 | Rust | 高效能、記憶體安全 |
| Excel 讀取 | calamine | Rust 原生、效能優異 |
| Excel 寫入 | rust_xlsxwriter | Rust 原生、功能完整 |

### 排除方案

| 方案 | 排除理由 |
|------|---------|
| Electron | 打包體積過大（~150MB vs Tauri ~15MB） |
| Python + PyO3 | 仍需 Python runtime，部署複雜 |
| 純 Rust GUI (egui) | GUI 生態不夠成熟，UI 彈性不足 |

---

## 實作階段

### 階段 1：專案初始化（Week 1）

```
任務清單：
□ 建立 Tauri 2.0 專案結構
□ 設定 Svelte 5 + TypeScript + Vite
□ 設定 TailwindCSS（賽博龐克主題）
□ 建立基本目錄結構
□ 設定 CI/CD pipeline
```

### 階段 2：Rust 核心引擎（Week 2-3）

```
任務清單：
□ 實作 Excel 解析器（calamine）
  □ Transaction 結構解析
  □ IpRecord 結構解析
  □ 時間格式處理
  □ 錯誤處理

□ 實作 IP 比對引擎
  □ 建立帳號索引 HashMap
  □ 時間窗口比對邏輯
  □ 結果格式化
  □ 平行運算最佳化（Rayon）

□ 實作資料處理器
  □ 敏感欄位過濾
  □ 收支分流邏輯

□ 實作 Whois 查詢
  □ HTTP 客戶端（reqwest）
  □ 結果快取
  □ 錯誤處理

□ 實作 Excel 匯出器（rust_xlsxwriter）
  □ 多 Sheet 產生
  □ 格式設定
```

### 階段 3：Tauri 命令層（Week 3）

```
任務清單：
□ 實作 load_file 命令
□ 實作 run_analysis 命令
□ 實作 export_excel 命令
□ 實作進度事件發送
□ 設定 App State 管理
```

### 階段 4：Svelte 前端（Week 4-5）

```
任務清單：
□ 實作 UI 元件
  □ DropZone（檔案拖曳區）
  □ ControlPanel（控制面板）
  □ ProgressBar（進度條）
  □ LogConsole（日誌區）
  □ ResultTable（結果表格）
  □ WarningBanner（OpSec 警告）

□ 實作狀態管理
  □ filesStore
  □ settingsStore
  □ analysisStore
  □ logsStore

□ 實作 Tauri API 封裝
□ 實作賽博龐克主題 CSS
```

### 階段 5：測試與最佳化（Week 6）

```
任務清單：
□ 撰寫 Rust 單元測試
□ 撰寫整合測試
□ 產生測試用 Excel 檔案
□ 效能測試與最佳化
□ 記憶體使用測試
```

### 階段 6：打包與發布（Week 7）

```
任務清單：
□ Windows MSI 打包設定
□ 可攜版 ZIP 打包
□ 程式碼簽署（如需要）
□ 撰寫發布說明
□ GitHub Release 發布
```

---

## 測試檔案規格

### 測試用 Excel 檔案

#### `tests/fixtures/sample_transactions.xlsx`

產生邏輯：
- 1000 筆測試交易紀錄
- 時間範圍：2024-01-01 ~ 2024-01-31
- 帳號：10 個測試帳號循環使用
- 存入/支出：隨機分配

#### `tests/fixtures/sample_ip_records.xlsx`

產生邏輯：
- 5000 筆 IP 登入紀錄
- 時間範圍：與交易對應（±3 秒）
- 部分紀錄故意不匹配（測試 N/A 情況）
- 部分紀錄多 IP（測試多 IP 格式化）

---

## 風險評估

| 風險 | 等級 | 緩解措施 |
|------|------|---------|
| Rust 學習曲線 | 中 | 提供完整文件與範例 |
| Tauri 2.0 相容性 | 低 | 使用穩定版 API |
| Windows 打包問題 | 中 | 提前測試 CI/CD |
| 效能不如預期 | 低 | 預留最佳化時間 |

---

## 成功標準

| 指標 | 目標值 |
|------|--------|
| 檔案載入速度 | 10 萬筆 < 2 秒 |
| IP 比對速度 | 10 萬筆 < 5 秒 |
| 記憶體使用 | 10 萬筆 < 500 MB |
| 打包體積 | < 20 MB |
| 啟動時間 | < 1 秒 |

---

## 附錄：Rust 核心模組虛擬碼

### IP 比對演算法

```rust
pub fn match_ips(
    transactions: &[Transaction],
    ip_records: &[IpRecord],
) -> Vec<MatchResult> {
    // 1. 建立帳號索引
    let index: HashMap<String, Vec<&IpRecord>> = ip_records
        .iter()
        .fold(HashMap::new(), |mut acc, record| {
            acc.entry(record.account.clone())
                .or_default()
                .push(record);
            acc
        });

    // 2. 平行比對
    transactions
        .par_iter()
        .map(|tx| {
            let matches = index
                .get(&tx.account)
                .map(|records| {
                    records
                        .iter()
                        .filter(|r| {
                            let diff = (r.timestamp - tx.timestamp).num_seconds();
                            diff >= -1 && diff <= 2
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            format_match_result(&matches, tx.timestamp)
        })
        .collect()
}
```

---

*此設計計畫將隨開發進度更新*
