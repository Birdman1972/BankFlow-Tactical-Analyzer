# Bug Fix Report: File Input & Tauri IPC Issues
**Date:** 2026-01-26
**Reporter:** Gemini (NetOps Architect)
**Status:** In Progress / Handover

## 1. Problem Description
Users reported inability to drag-and-drop files or select files via click in the `BankFlow-Tactical-Analyzer` application.
Error logs indicated:
> `Failed to load File A: TypeError: undefined is not an object (evaluating 'window.__TAURI_INTERNALS__.invoke')`

This suggests two concurrent issues:
1. **Event Handling:** The `DropZone` component was missing `on:drop` event bindings.
2. **Environment/IPC:** The application is throwing errors related to Tauri internals, likely due to execution in a standard browser environment (without Tauri injection) or a mismatch in Tauri v2 API usage/detection.

## 2. Changes Implemented (by Gemini)

### A. Store Refactoring (`src/lib/stores/tauri.ts`)
- **Objective:** Separate the "File Selection Dialog" logic from the "Backend Loading" logic to support drag-and-drop (which bypasses the dialog).
- **Changes:**
  - Extracted `loadFileA(path)` and `loadFileB(path)` from `selectAndLoadFile*`.
  - Implemented `tauriGuard` (via `handleTauriError`) to catch environment-specific errors gracefully instead of crashing the UI.
  - Removed strict `window.__TAURI_INTERNALS__` pre-checks in favor of try-catch blocks to accommodate Tauri v2's opaque internals.

### B. UI Event Binding (`src/App.svelte`)
- **Objective:** Enable drag-and-drop interaction.
- **Changes:**
  - Imported `loadFileA` / `loadFileB`.
  - Added `handleFileADrop` and `handleFileBDrop` handlers.
  - Bound `on:drop` event to `DropZone` components.

### C. Component Debugging (`src/lib/components/DropZone.svelte`)
- **Objective:** Inspect file objects dropped into the zone.
- **Changes:**
  - Added `console.log` for dropped files.
  - Added safe access to `file.path` (specific to Tauri) with fallback to `file.name`.

## 3. Analysis & Next Steps for Claude

### Root Cause Analysis
- The primary error `undefined is not an object (evaluating 'window.__TAURI_INTERNALS__.invoke')` confirms that the Tauri API (`@tauri-apps/api/core`) is trying to access global internals that are missing.
- **Hypothesis:** The user might be testing in a standard browser (`npm run dev`) where these internals do not exist, OR there is a configuration issue in `tauri.conf.json` regarding `withGlobalTauri` vs the module-based import strategy in Tauri v2.

### Recommendations
1.  **Verify Environment:** Ensure development is done via `npm run tauri dev`.
2.  **Mock Mode (Optional):** If browser-based development is required, implement a "Mock Mode" in `src/lib/stores/tauri.ts` that returns dummy data when `handleTauriError` catches an IPC error.
3.  **Check Permissions:** Verify `src-tauri/capabilities/default.json` (or similar v2 permission system) allows `fs:read` for the drag-and-drop paths.

### Files Modified
- `src/lib/stores/tauri.ts`
- `src/App.svelte`
- `src/lib/components/DropZone.svelte`

---
*End of Report*
