# BankFlow Tactical Analyzer v0.3.9

**Release Date**: 2026-01-29
**Codename**: High-Tech Lab

## Highlights
- Cyberpunk Light Theme and modern Theme Toggle
- Fail-Safe File Validation with clear UI errors
- WASM export stability improvements

## ✨ New Features
- **Cyberpunk Light Theme**: A cleaner, high-contrast interface designed for prolonged usage. Inspired by "High-Tech Lab" aesthetics.
- **Theme Toggle**: Redesigned as a modern "Pill Switch" with sliding animations, located in the header.
- **Dynamic Theming**: Real-time CSS variable switching without page reload.
- **Fail-Safe File Validation**:
  - **Strict Header Check**: Automatically rejects files missing mandatory columns.
  - **Visual Feedback**: DropZone turns red and lists exact missing fields on error.
  - **Safety Lock**: Analysis buttons are disabled until valid files are loaded.

## 🐛 Bug Fixes
- **Vercel Runtime Crash**: Fixed unsafe `<html>` injection by using reactive DOM manipulation.
- **WASM Export Stability**: Confirmed fix for `SystemTime` panic by enabling `rust_xlsxwriter/wasm`.

## 📦 Build & Packaging
- **Windows**: MSI build via CI
- **macOS**: DMG build via CI
- **Linux**: AppImage and DEB via CI

## Assets
See attached assets for your platform:
- Windows: `.msi`
- macOS: `.dmg`
- Linux: `.AppImage`, `.deb`
