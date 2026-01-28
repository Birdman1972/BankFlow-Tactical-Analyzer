#!/bin/bash
# Build WASM module from bankflow-core
#
# Usage:
#   ./scripts/build-wasm.sh [--release]
#
# Output:
#   src/lib/wasm/bankflow-core-wasm/

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CORE_DIR="$PROJECT_ROOT/crates/bankflow-core"
OUTPUT_DIR="$PROJECT_ROOT/src/lib/wasm/bankflow-core-wasm"

# Ensure MSVC linker is used when running under Git Bash on Windows runners.
if [[ "$OS" == "Windows_NT" && -n "$VCToolsInstallDir" ]]; then
    if command -v cygpath >/dev/null 2>&1; then
        VC_BIN_PATH="$(cygpath "$VCToolsInstallDir")/bin/Hostx64/x64"
        export PATH="$VC_BIN_PATH:$PATH"
        echo "   Preferring MSVC linker from $VC_BIN_PATH"
    fi
fi

echo "🔧 Building WASM module..."
echo "   Source: $CORE_DIR"
echo "   Output: $OUTPUT_DIR"

# Check for wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Install with: cargo install wasm-pack"
    exit 1
fi

# Build options (wasm-pack uses release by default)
if [ "$1" == "--debug" ]; then
    BUILD_OPTS="--dev"
    echo "   Mode: Development (debug)"
else
    BUILD_OPTS=""
    echo "   Mode: Release (optimized)"
fi

# Build WASM (builds to pkg/ directory by default)
cd "$CORE_DIR"
wasm-pack build --target web --features wasm $BUILD_OPTS

# Move output to target directory
rm -rf "$OUTPUT_DIR"
mkdir -p "$(dirname "$OUTPUT_DIR")"
mv "$CORE_DIR/pkg" "$OUTPUT_DIR"

# Clean up unnecessary files
rm -f "$OUTPUT_DIR/.gitignore"
rm -f "$OUTPUT_DIR/README.md"

echo ""
echo "✅ WASM build complete!"
echo "   Package: $OUTPUT_DIR"
echo ""
echo "📦 Generated files:"
ls -la "$OUTPUT_DIR"
