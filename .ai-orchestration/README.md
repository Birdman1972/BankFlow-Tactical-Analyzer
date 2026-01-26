# 多 AI 協作框架

這是一個用於協調多個 AI 模型協作的框架，支援自動任務路由、成本優化、品質控制。

---

## 可用的 AI 模型

### CLI 工具

| 代理 ID | 名稱 | 適合任務 | 成本 |
|---------|------|----------|------|
| `claude-code` | Claude Opus 4.5 | 架構、審查、複雜推理 | 💰💰💰 |
| `gemini-cli` | Gemini CLI | 翻譯、文件 | 💰 |
| `openai-codex` | OpenAI Codex | 批量修改、重構 | 💰💰 |

### Antigravity IDE

| 代理 ID | 名稱 | 適合任務 | 成本 |
|---------|------|----------|------|
| `gemini-3-pro-thinking` | Gemini 3 Pro (Thinking) | 深度分析、複雜除錯 | 💰💰 |
| `gemini-3-flash` | Gemini 3 Flash | 簡單快速任務 | 💰 |
| `claude-opus-ide` | Claude Opus (IDE) | 同 Claude Code | 💰💰💰 |
| `claude-sonnet-ide` | Claude Sonnet (IDE) | 中等複雜度 | 💰💰 |
| `gpt-oss-120b` | GPT-OSS 120B | 成本敏感任務 | 🆓 |

---

## 快速開始

### 1. 建立任務

複製模板到 `tasks/queue/`：

```bash
cp templates/task-code.yaml tasks/queue/TASK-001.yaml
```

編輯任務檔案，填入任務詳情。

### 2. 通知 AI 執行

告訴對應的 AI：

```
請讀取 .ai-orchestration/tasks/queue/TASK-001.yaml 並執行任務
```

### 3. AI 執行任務

AI 會：
1. 讀取任務定義
2. 將產出寫入 `outputs/TASK-001/`
3. （可選）更新任務狀態

### 4. 審查整合

Claude Opus（總指揮）會：
1. 審查產出品質
2. 整合到專案
3. 移動任務到 `completed/`

---

## 資料夾結構

```
.ai-orchestration/
├── config/
│   ├── agents.yaml      # AI 代理定義
│   └── routing.yaml     # 路由規則
│
├── tasks/
│   ├── queue/           # 待處理
│   ├── in-progress/     # 處理中
│   ├── review/          # 等待審查
│   ├── completed/       # 已完成
│   └── failed/          # 失敗
│
├── outputs/             # 任務產出
├── handoffs/            # 代理間傳遞
├── templates/           # 任務模板
└── logs/                # 執行日誌
```

---

## 自動路由規則

系統會根據任務特性自動建議最適合的 AI：

| 任務特性 | 建議代理 |
|---------|---------|
| 需要翻譯 | Gemini CLI |
| 複雜度：expert | Claude Opus |
| 程式碼審查 | Claude Opus |
| 批量修改（>3 檔案） | Codex |
| 簡單文件 | Gemini Flash |
| 成本敏感 | GPT-OSS 120B |
| 預設 | Claude Sonnet |

---

## 任務檔案格式

```yaml
id: "TASK-001"
metadata:
  title: "任務標題"
  category: feature
  priority: high

classification:
  complexity: moderate
  requires_translation: false

context:
  files:
    - "src/example.ts"

requirements:
  - "需求 1"
  - "需求 2"

acceptance:
  - "[ ] 驗收項目"

output:
  directory: ".ai-orchestration/outputs/TASK-001"

status: pending
```

---

## 成本優化策略

1. **先便宜後昂貴** - 簡單任務用低成本模型
2. **失敗後升級** - 2 次失敗後升級到更強模型
3. **批量處理** - 相似任務合併處理

---

## 品質控制

### 自動檢查

所有程式碼任務完成後自動執行：
- `npm run check` - TypeScript 檢查
- `npm run build:web` - 建置測試

### Guardian Protocol

連續失敗 5 次 → 自動停止 → 通知人工介入

---

## 使用範例

### 範例 1：翻譯任務

```yaml
# tasks/queue/TASK-I18N-001.yaml
id: "TASK-I18N-001"
metadata:
  title: "新增日文翻譯"
  category: i18n

classification:
  requires_translation: true

routing:
  suggested_agent: gemini-cli  # 自動建議
```

### 範例 2：批量重構

```yaml
# tasks/queue/TASK-REFACTOR-001.yaml
id: "TASK-REFACTOR-001"
metadata:
  title: "重構所有元件使用新 API"
  category: refactor

classification:
  complexity: simple
  files_count: 10

routing:
  suggested_agent: openai-codex  # 批量修改用 Codex
```

---

## 協調者角色

**Claude Opus（我）** 作為總指揮，負責：

1. 分析任務並建議最佳代理
2. 產生任務檔案
3. 審查其他代理的產出
4. 整合到專案
5. 維護品質標準

---

## 注意事項

- 任務檔案使用 YAML 格式
- 所有輸出寫入 `outputs/{TASK-ID}/`
- 完成後將任務移至 `completed/`
- 失敗任務保留在 `failed/` 供分析
