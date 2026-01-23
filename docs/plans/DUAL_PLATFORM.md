# BankFlow Tactical Analyzer - 雙平台架構規劃

**建立日期**：2026-01-21
**更新日期**：2026-01-21
**目標**：提供網頁版 + 桌面版雙軌並行
**技術決策**：使用 Rust → WebAssembly，共用核心引擎

---

## 架構總覽（WASM 版本）

```
                    crates/bankflow-core/
                    (共用 Rust 核心引擎)
                           │
           ┌───────────────┼───────────────┐
           ▼                               ▼
      src-tauri/                      src-wasm/
      (Tauri 桌面版)                  (wasm-bindgen)
           │                               │
           ▼                               ▼
      原生執行檔                      .wasm 檔案
      (.exe/.dmg)                          │
                                          ▼
                              ┌─────────────────────┐
                              │   Vercel 網頁版     │
                              │  (Rust WASM 效能)   │
                              │                     │
                              │  [下載桌面版按鈕]   │
                              └─────────────────────┘
```

## 優點

- **效能**：網頁版使用 Rust WASM，接近原生速度
- **維護**：只維護一份 Rust 核心程式碼
- **一致性**：網頁版與桌面版行為完全一致
- **開發效率**：不需重寫 JavaScript 版本

---

## 舊架構（已棄用）

~~```
┌─────────────────────────────────────────────────────────────────┐
│                     Vercel 部署                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    網頁版 (Web App)                       │  │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐     │  │
│  │  │DropZone │  │ Control │  │  Log    │  │ Result  │     │  │
│  │  │         │  │  Panel  │  │ Console │  │ Summary │     │  │
│  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘     │  │
│  │                      │                                    │  │
│  │              ┌───────▼───────┐                           │  │
│  │              │ JS 分析引擎   │  ← 純前端 JavaScript      │  │
│  │              │ (xlsx.js)     │                           │  │
│  │              └───────────────┘                           │  │
│  │                                                          │  │
│  │  ┌──────────────────────────────────────────────────┐   │  │
│  │  │  下載區塊                                         │   │  │
│  │  │  [Windows .exe]  [macOS .dmg]  [Linux .AppImage] │   │  │
│  │  └──────────────────────────────────────────────────┘   │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```~~
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                   GitHub Releases                                │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐    │
│  │ Windows MSI    │  │ macOS DMG      │  │ Linux AppImage │    │
│  │ (.exe)         │  │ (.dmg)         │  │ (.AppImage)    │    │
│  └────────────────┘  └────────────────┘  └────────────────┘    │
│                                                                  │
│  桌面版功能：                                                    │
│  - Rust 高效能引擎                                              │
│  - 完全離線運作                                                  │
│  - 大檔案支援 (100MB+)                                          │
│  - Whois 查詢 (需網路)                                          │
└─────────────────────────────────────────────────────────────────┘
```

---

## 功能比較

| 功能 | 網頁版 | 桌面版 |
|------|--------|--------|
| Excel 解析 | ✅ xlsx.js | ✅ calamine (Rust) |
| IP 比對 | ✅ JavaScript | ✅ Rust + rayon |
| 時間窗口 | ✅ [-1s, +2s] | ✅ [-1s, +2s] |
| 敏感欄位隱藏 | ✅ | ✅ |
| 收支分流 | ✅ | ✅ |
| Excel 匯出 | ✅ xlsx.js | ✅ rust_xlsxwriter |
| Whois 查詢 | ❌ (CORS 限制) | ✅ reqwest |
| 大檔案支援 | ⚠️ 有限 (~10MB) | ✅ 100MB+ |
| 離線使用 | ❌ | ✅ |
| 安裝需求 | 無 | 需下載安裝 |

---

## 目錄結構

```
BankFlow-Tactical-Analyzer/
├── src/                      # 共用 Svelte 元件
│   ├── lib/
│   │   ├── components/       # UI 元件 (共用)
│   │   └── stores/
│   │       ├── app.ts        # 狀態管理 (共用)
│   │       ├── tauri.ts      # 桌面版 API
│   │       └── web.ts        # 網頁版 API (新增)
│   └── App.svelte
│
├── src-tauri/                # 桌面版 Rust 後端
│   └── src/
│       ├── core/             # Rust 分析引擎
│       └── commands/         # Tauri 命令
│
├── src-web/                  # 網頁版專用 (新增)
│   ├── core/                 # JavaScript 分析引擎
│   │   ├── parser.ts         # Excel 解析 (xlsx.js)
│   │   ├── matcher.ts        # IP 比對
│   │   ├── processor.ts      # 資料處理
│   │   └── exporter.ts       # Excel 匯出
│   └── download/             # 下載區塊元件
│       └── DownloadSection.svelte
│
├── dist/                     # Vite 建置輸出 (網頁版)
└── src-tauri/target/         # Tauri 建置輸出 (桌面版)
```

---

## 開發階段

### Phase 7A：網頁版 JavaScript 引擎

- [ ] 安裝 xlsx.js 依賴
- [ ] 實作 `src-web/core/parser.ts` - Excel 解析
- [ ] 實作 `src-web/core/matcher.ts` - IP 比對
- [ ] 實作 `src-web/core/processor.ts` - 資料處理
- [ ] 實作 `src-web/core/exporter.ts` - Excel 匯出
- [ ] 實作 `src/lib/stores/web.ts` - 網頁版 API 封裝

### Phase 7B：平台切換機制

- [ ] 建立環境偵測 (Tauri vs Web)
- [ ] 建立統一 API 介面
- [ ] 修改 stores 根據環境載入不同實作

### Phase 7C：下載區塊

- [ ] 實作 `DownloadSection.svelte` 元件
- [ ] 整合 GitHub Releases API
- [ ] 自動偵測使用者作業系統
- [ ] 顯示版本號與檔案大小

### Phase 7D：Vercel 部署

- [ ] 設定 `vercel.json`
- [ ] 設定建置指令 (排除 Tauri)
- [ ] 部署測試
- [ ] 自訂域名 (選用)

### Phase 7E：桌面版打包

- [ ] Windows MSI 打包
- [ ] macOS DMG 打包 (需 Apple 開發者帳號簽署)
- [ ] Linux AppImage 打包
- [ ] 上傳至 GitHub Releases
- [ ] 設定 CI/CD 自動打包 (選用)

---

## 技術細節

### 環境偵測

```typescript
// src/lib/stores/platform.ts
export const isTauri = typeof window !== 'undefined'
  && window.__TAURI_INTERNALS__ !== undefined;

export const platform = isTauri ? 'desktop' : 'web';
```

### 統一 API 介面

```typescript
// src/lib/stores/api.ts
import * as tauriApi from './tauri';
import * as webApi from './web';
import { isTauri } from './platform';

export const api = isTauri ? tauriApi : webApi;

// 使用方式（元件中）
import { api } from '$lib/stores/api';
await api.selectAndLoadFileA();
await api.runAnalysis();
await api.exportReport();
```

### Vercel 建置設定

```json
// vercel.json
{
  "buildCommand": "npm run build:web",
  "outputDirectory": "dist",
  "framework": "svelte"
}
```

```json
// package.json (新增指令)
{
  "scripts": {
    "build:web": "vite build",
    "build:desktop": "tauri build"
  }
}
```

---

## 預估工時

| 階段 | 預估時間 | 說明 |
|------|----------|------|
| Phase 7A | 2-3 小時 | JS 引擎實作 |
| Phase 7B | 1 小時 | 平台切換 |
| Phase 7C | 1 小時 | 下載區塊 |
| Phase 7D | 30 分鐘 | Vercel 部署 |
| Phase 7E | 1-2 小時 | 桌面版打包 |

**總計**：約 5-7 小時

---

## 安全考量

### 網頁版

- 所有處理在瀏覽器端完成
- 檔案不會上傳到伺服器
- 無 Whois 功能（避免 CORS 問題）
- 建議加上隱私聲明

### 桌面版

- 完全離線可用
- Whois 功能會對外連線（顯示警告）
- 可選擇不啟用網路功能

---

## 下一步

確認此規劃後，從 Phase 7A 開始實作網頁版 JavaScript 引擎。

---

*此文件將隨開發進度更新*
