#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TAURI_CONF="$ROOT_DIR/src-tauri/tauri.conf.json"

VERSION=$(python3 - <<'PY'
import json, sys
with open(sys.argv[1], 'r', encoding='utf-8') as f:
    data = json.load(f)
print(data.get('version','unknown'))
PY
"$TAURI_CONF")

OUT_DIR="$ROOT_DIR/dist/portable"
mkdir -p "$OUT_DIR"

ZIP_NAME="BankFlow-Tactical-Analyzer_${VERSION}_portable.zip"
ZIP_PATH="$OUT_DIR/$ZIP_NAME"

# Prefer Windows portable exe if available
EXE_PATH=$(ls "$ROOT_DIR"/src-tauri/target/release/*.exe 2>/dev/null | head -n 1 || true)
MAC_APP="$ROOT_DIR/src-tauri/target/release/bundle/macos/BankFlow-Tactical-Analyzer.app"
APPIMAGE_PATH=$(ls "$ROOT_DIR"/src-tauri/target/release/bundle/appimage/*.AppImage 2>/dev/null | head -n 1 || true)

rm -f "$ZIP_PATH"

if [[ -n "$EXE_PATH" ]]; then
  (cd "$(dirname "$EXE_PATH")" && zip -r "$ZIP_PATH" "$(basename "$EXE_PATH")")
  echo "Portable ZIP created: $ZIP_PATH"
  exit 0
fi

if [[ -d "$MAC_APP" ]]; then
  (cd "$(dirname "$MAC_APP")" && zip -r "$ZIP_PATH" "$(basename "$MAC_APP")")
  echo "Portable ZIP created (macOS app bundle): $ZIP_PATH"
  exit 0
fi

if [[ -n "$APPIMAGE_PATH" ]]; then
  (cd "$(dirname "$APPIMAGE_PATH")" && zip -r "$ZIP_PATH" "$(basename "$APPIMAGE_PATH")")
  echo "Portable ZIP created (AppImage): $ZIP_PATH"
  exit 0
fi

cat <<MSG
No build artifacts found.
Build the app first, then re-run:
  npm run build:tauri

Expected locations:
- Windows: src-tauri/target/release/*.exe
- macOS:   src-tauri/target/release/bundle/macos/BankFlow-Tactical-Analyzer.app
- Linux:   src-tauri/target/release/bundle/appimage/*.AppImage
MSG
exit 1
