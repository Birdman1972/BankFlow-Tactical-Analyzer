# Desktop Audit (Tauri)

## Current state (as of this repo)

- Desktop packaging config: `src-tauri/tauri.conf.json`
  - bundle targets: `msi`, `app`, `dmg`, `deb`, `appimage`
- Web UI embedded in desktop: built from `dist/` (`frontendDist: ../dist`)

## Version parity risks

- UI version source: `package.json#version` (web runtime)
- Tauri package version: `src-tauri/tauri.conf.json#version`
- Tauri Rust crate version: `src-tauri/Cargo.toml#[package].version`

All three should match to avoid:

- Desktop showing a different version than the installed bundle
- Update dialog comparing against the wrong local version

## Update-check behavior

- Update dialog logic: `src/lib/services/versionService.ts`
  - Remote manifest URL: `https://bankflow-tactical-analyzer.vercel.app/version.json`
  - Skip/ignore persisted via `localStorage` key `bankflow_skipped_version`
- Desktop fetch path: via Tauri command `check_update` in `src-tauri/src/commands/system.rs`

## Known issues to address

- `GET /version.json` must exist in web build output (Vite serves `public/` → `dist/`)
- Release link mismatch in UI (opens a different GitHub repo than README)
