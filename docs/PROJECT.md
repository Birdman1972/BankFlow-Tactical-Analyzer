# BankFlow Tactical Analyzer

## 專案總覽

```
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║    ██████╗  █████╗ ███╗   ██╗██╗  ██╗███████╗██╗      ██████╗ ██╗    ██╗    ║
║    ██╔══██╗██╔══██╗████╗  ██║██║ ██╔╝██╔════╝██║     ██╔═══██╗██║    ██║    ║
║    ██████╔╝███████║██╔██╗ ██║█████╔╝ █████╗  ██║     ██║   ██║██║ █╗ ██║    ║
║    ██╔══██╗██╔══██║██║╚██╗██║██╔═██╗ ██╔══╝  ██║     ██║   ██║██║███╗██║    ║
║    ██████╔╝██║  ██║██║ ╚████║██║  ██╗██║     ███████╗╚██████╔╝╚███╔███╔╝    ║
║    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝     ╚══════╝ ╚═════╝  ╚══╝╚══╝     ║
║                                                                              ║
║                    T A C T I C A L   A N A L Y Z E R                        ║
║                        數 位 鑑 識 戰 術 分 析 系 統                          ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

---

## 目錄

- [專案簡介](#專案簡介)
- [系統心智圖](#系統心智圖)
- [核心功能](#核心功能)
- [系統架構](#系統架構)
- [技術堆疊](#技術堆疊)
- [文件導覽](#文件導覽)

---

## 專案簡介

**BankFlow Tactical Analyzer** 是一款專為**執法單位**與**數位鑑識人員**設計的離線分析工具。透過直覺的拖曳式介面，自動清洗、整理並交叉比對「銀行存款往來明細」與「網銀 IP 登入紀錄」，協助調查人員快速識別可疑交易來源。

### 設計理念

```
┌─────────────────────────────────────────────────────────────────┐
│                        核 心 原 則                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   🔒 離線優先        所有運算在本機完成，不上傳任何資料            │
│   Offline-First                                                 │
│                                                                 │
│   💾 記憶體運算      不產生中間暫存檔，處理完畢自動清除             │
│   In-Memory                                                     │
│                                                                 │
│   🛡️ 行動安全       OpSec 設計，Whois 查詢需明確授權              │
│   OpSec                                                         │
│                                                                 │
│   🎯 直覺操作        拖曳即用，零學習成本                          │
│   Intuitive                                                     │
│                                                                 │
│   ⚡ 高效能          Rust 核心引擎，處理百萬筆資料無壓力           │
│   High Performance                                              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 系統心智圖

```mermaid
mindmap
  root((BankFlow<br/>Tactical<br/>Analyzer))
    技術架構
      前端 Svelte
        元件化 UI
        狀態管理 Store
        TypeScript
      橋接 Tauri
        IPC Commands
        事件系統
        單一執行檔
      後端 Rust
        calamine 解析
        rust_xlsxwriter 輸出 (wasm)
        reqwest HTTP
    功能模組
      輸入處理
        檔案 A 存款明細
        檔案 B IP 紀錄
        拖曳上傳
      資料處理
        敏感欄位隱藏
        收支分流
        時間標準化
      核心分析
        IP 時間窗口比對
        Whois 反查
        多 IP 處理
      輸出匯出
        Excel 多 Sheet
        即時預覽
        記憶體清理
    安全設計
      離線優先
        本機運算
        無雲端上傳
        零網路依賴
      記憶體安全
        Rust 所有權
        自動釋放
        無暫存檔
      行動安全 OpSec
        Whois 預設關閉
        紅色警告提示
        使用者確認
    開發階段
      Phase 1 專案初始化
        Tauri 結構
        Svelte 設定
        主題樣式
      Phase 2 Rust 核心
        Excel 解析器
        IP 比對引擎
        資料處理器
      Phase 3 命令層
        load_file
        run_analysis
        export_excel
      Phase 4 前端 UI
        DropZone
        ControlPanel
        ResultTable
      Phase 5 測試
        單元測試
        整合測試
        效能測試
      Phase 6 發布
        Windows 打包
        程式碼簽署
        GitHub Release
    使用者介面
      賽博龐克主題
        深色背景
        螢光綠主色
        電路藍強調
      操作面板
        功能開關
        即時日誌
        進度顯示
```

---

## 核心功能

### 功能總覽圖

```mermaid
flowchart TB
    subgraph INPUT["📥 資料輸入"]
        A[("檔案 A<br/>存款明細<br/>.xlsx")]
        B[("檔案 B<br/>IP 紀錄<br/>.xlsx")]
    end

    subgraph PREPROCESS["⚙️ 前處理"]
        C{隱藏敏感欄位?}
        D[移除 C,F,L,M 欄]
        E{收支分流?}
        F[分離存入/支出]
    end

    subgraph ANALYSIS["🔍 核心分析"]
        G{IP 比對?}
        H["時間窗口匹配<br/>[-1s, +2s]"]
        I{Whois 查詢?}
        J["線上 API 反查<br/>⚠️ OpSec 風險"]
    end

    subgraph OUTPUT["📤 輸出"]
        K[("Excel 報告<br/>.xlsx")]
        L["Sheet1: 總表"]
        M["Sheet2: 存入"]
        N["Sheet3: 支出"]
    end

    A --> C
    B --> G
    C -->|ON| D --> E
    C -->|OFF| E
    E -->|ON| F --> G
    E -->|OFF| G
    G -->|ON| H --> I
    G -->|OFF| I
    I -->|ON| J --> K
    I -->|OFF| K
    K --> L
    K --> M
    K --> N

    style INPUT fill:#1a1a2e,stroke:#00D2FF,color:#E5E7EB
    style PREPROCESS fill:#1a1a2e,stroke:#00FF9D,color:#E5E7EB
    style ANALYSIS fill:#1a1a2e,stroke:#FF0055,color:#E5E7EB
    style OUTPUT fill:#1a1a2e,stroke:#00D2FF,color:#E5E7EB
```

### 功能詳細說明

| 功能 | 預設值 | 說明 |
|------|--------|------|
| **隱藏敏感欄位** | OFF | 移除檔案 A 的 C、F、L、M 欄（索引 2, 5, 11, 12）以保護個資 |
| **收支分流** | ON | 依據 I 欄（支出）和 J 欄（存入）將交易分離至獨立工作表 |
| **IP 交叉比對** | ON | 以時間窗口 [-1秒, +2秒] 比對交易時間與 IP 登入紀錄 |
| **Whois 線上反查** | OFF | 查詢 IP 所屬國籍與 ISP（需連線，有 OpSec 風險警告） |

---

## 系統架構

### 高階架構圖

```mermaid
flowchart TB
    subgraph FRONTEND["🖥️ 前端 (Svelte)"]
        UI["使用者介面"]
        STORE["狀態管理"]
        COMP["UI 元件"]
    end

    subgraph BRIDGE["🔗 Tauri 橋接層"]
        CMD["Tauri Commands"]
        EVENT["事件系統"]
    end

    subgraph BACKEND["⚙️ 後端 (Rust)"]
        subgraph COMMANDS["命令模組"]
            FILE_OPS["file_ops.rs<br/>檔案操作"]
            ANALYSIS["analysis.rs<br/>分析引擎"]
            WHOIS["whois.rs<br/>網路查詢"]
        end
        subgraph CORE["核心引擎"]
            PARSER["parser.rs<br/>Excel 解析"]
            MATCHER["matcher.rs<br/>IP 比對"]
            TYPES["types.rs<br/>資料結構"]
        end
    end

    UI <--> CMD
    STORE <--> EVENT
    CMD <--> FILE_OPS
    CMD <--> ANALYSIS
    CMD <--> WHOIS
    FILE_OPS --> PARSER
    ANALYSIS --> MATCHER
    PARSER --> TYPES
    MATCHER --> TYPES

    style FRONTEND fill:#2d2d44,stroke:#00D2FF,color:#E5E7EB
    style BRIDGE fill:#2d2d44,stroke:#00FF9D,color:#E5E7EB
    style BACKEND fill:#2d2d44,stroke:#FF0055,color:#E5E7EB
```

### 資料流程圖

```mermaid
sequenceDiagram
    participant U as 使用者
    participant F as 前端 (Svelte)
    participant T as Tauri Bridge
    participant R as Rust 引擎
    participant API as 外部 API

    U->>F: 拖曳檔案 A & B
    F->>T: invoke('load_files')
    T->>R: 解析 Excel
    R-->>T: 回傳資料摘要
    T-->>F: 更新 UI 狀態

    U->>F: 點擊「執行分析」
    F->>T: invoke('run_analysis')
    T->>R: 執行 IP 比對

    loop 每 100 筆
        R-->>T: emit('progress')
        T-->>F: 更新進度條
    end

    alt Whois 開啟
        R->>API: HTTP GET (ip-api.com)
        API-->>R: 回傳國籍/ISP
    end

    R-->>T: 回傳分析結果
    T-->>F: 顯示結果預覽

    U->>F: 點擊「下載報告」
    F->>T: invoke('export_excel')
    T->>R: 產生 .xlsx
    R-->>T: 回傳檔案路徑
    T-->>F: 觸發下載
    F-->>U: 儲存檔案對話框
```

---

## 技術堆疊

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              技 術 堆 疊                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐     │
│  │   Svelte    │   │    Tauri    │   │    Rust     │   │  Libraries  │     │
│  │             │   │             │   │             │   │             │     │
│  │  • UI 渲染  │   │  • 桌面整合 │   │  • 核心邏輯 │   │  • calamine │     │
│  │  • 狀態管理 │   │  • IPC 通訊 │   │  • 高效運算 │   │  • rust_xlsx │     │
│  │  • 元件化   │   │  • 打包發布 │   │  • 記憶體安全│  │  • reqwest  │     │
│  │             │   │             │   │             │   │  • chrono   │     │
│  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘   └──────┬──────┘     │
│         │                 │                 │                 │             │
│         └─────────────────┴─────────────────┴─────────────────┘             │
│                                   │                                         │
│                                   ▼                                         │
│                    ┌─────────────────────────────┐                          │
│                    │     Windows 11 .exe         │                          │
│                    │     單一執行檔（~15MB）      │                          │
│                    └─────────────────────────────┘                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 技術選型理由

| 層級 | 技術 | 選擇理由 |
|------|------|---------|
| **前端** | Svelte | 編譯時優化、打包體積小、元件化架構便於擴充 |
| **框架** | Tauri | 比 Electron 體積小 10 倍、原生效能、Rust 整合 |
| **後端** | Rust | 記憶體安全、高效能、適合處理大量資料 |
| **Excel** | calamine + rust_xlsxwriter (wasm) | Rust 原生、讀寫效能優異 |

---

## 文件導覽

| 文件 | 說明 |
|------|------|
| [📖 ARCHITECTURE.md](./ARCHITECTURE.md) | 詳細系統架構與模組說明 |
| [📘 USER_GUIDE.md](./USER_GUIDE.md) | 使用者操作指南 |
| [📗 TECHNICAL_SPEC.md](./TECHNICAL_SPEC.md) | 技術規格與 API 文件 |
| [📋 plans/](./plans/) | 開發計畫與設計文件 |

---

<div align="center">

**版本**：2.0.0-tauri | **技術架構**：Tauri + Rust + Svelte

*專為數位鑑識打造的戰術級分析工具*

</div>
