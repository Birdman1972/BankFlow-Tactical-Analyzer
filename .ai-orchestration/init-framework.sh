#!/bin/bash
# 多 AI 協作框架初始化腳本
# 用法: curl -sL [url] | bash -s /path/to/project
#   或: ./init-framework.sh /path/to/project

set -e

# 來源目錄（框架所在位置）
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 目標目錄
TARGET_DIR="${1:-.}"

if [ "$TARGET_DIR" = "." ]; then
    echo "⚠️  用法: $0 <目標專案路徑>"
    echo "   範例: $0 ~/projects/my-new-project"
    exit 1
fi

# 確認目標目錄存在
if [ ! -d "$TARGET_DIR" ]; then
    echo "❌ 目標目錄不存在: $TARGET_DIR"
    exit 1
fi

echo "🚀 初始化多 AI 協作框架..."
echo "   來源: $SCRIPT_DIR"
echo "   目標: $TARGET_DIR"

# 建立目錄結構
mkdir -p "$TARGET_DIR/.ai-orchestration/config"
mkdir -p "$TARGET_DIR/.ai-orchestration/tasks/queue"
mkdir -p "$TARGET_DIR/.ai-orchestration/tasks/in-progress"
mkdir -p "$TARGET_DIR/.ai-orchestration/tasks/review"
mkdir -p "$TARGET_DIR/.ai-orchestration/tasks/completed"
mkdir -p "$TARGET_DIR/.ai-orchestration/tasks/failed"
mkdir -p "$TARGET_DIR/.ai-orchestration/outputs"
mkdir -p "$TARGET_DIR/.ai-orchestration/handoffs"
mkdir -p "$TARGET_DIR/.ai-orchestration/templates"
mkdir -p "$TARGET_DIR/.ai-orchestration/logs"

# 複製配置檔案
cp "$SCRIPT_DIR/config/agents.yaml" "$TARGET_DIR/.ai-orchestration/config/"
cp "$SCRIPT_DIR/config/routing.yaml" "$TARGET_DIR/.ai-orchestration/config/"

# 複製模板
cp "$SCRIPT_DIR/templates/task-code.yaml" "$TARGET_DIR/.ai-orchestration/templates/"
cp "$SCRIPT_DIR/templates/task-docs.yaml" "$TARGET_DIR/.ai-orchestration/templates/"
cp "$SCRIPT_DIR/templates/task-review.yaml" "$TARGET_DIR/.ai-orchestration/templates/"

# 複製 README
cp "$SCRIPT_DIR/README.md" "$TARGET_DIR/.ai-orchestration/"

# 建立 .gitkeep 檔案
touch "$TARGET_DIR/.ai-orchestration/tasks/queue/.gitkeep"
touch "$TARGET_DIR/.ai-orchestration/tasks/in-progress/.gitkeep"
touch "$TARGET_DIR/.ai-orchestration/tasks/review/.gitkeep"
touch "$TARGET_DIR/.ai-orchestration/tasks/completed/.gitkeep"
touch "$TARGET_DIR/.ai-orchestration/tasks/failed/.gitkeep"
touch "$TARGET_DIR/.ai-orchestration/outputs/.gitkeep"
touch "$TARGET_DIR/.ai-orchestration/handoffs/.gitkeep"
touch "$TARGET_DIR/.ai-orchestration/logs/.gitkeep"

# 複製 AGENTS.md 到專案根目錄（如果不存在）
if [ ! -f "$TARGET_DIR/AGENTS.md" ]; then
    cat > "$TARGET_DIR/AGENTS.md" << 'AGENTSEOF'
<INSTRUCTIONS>
## 全域偏好
- 回覆請使用繁體中文（台灣用語）。
</INSTRUCTIONS>
AGENTSEOF
    echo "✅ 已建立 AGENTS.md"
fi

echo ""
echo "✅ 框架初始化完成！"
echo ""
echo "📁 已建立結構："
echo "   .ai-orchestration/"
echo "   ├── config/          # 代理與路由設定"
echo "   ├── tasks/           # 任務佇列"
echo "   ├── outputs/         # 任務產出"
echo "   ├── templates/       # 任務模板"
echo "   └── README.md        # 使用說明"
echo ""
echo "🎯 下一步："
echo "   1. 根據專案需求調整 config/routing.yaml"
echo "   2. 建立第一個任務: cp templates/task-code.yaml tasks/queue/TASK-001.yaml"
echo "   3. 告訴 AI: 請讀取 .ai-orchestration/tasks/queue/TASK-001.yaml 並執行"
