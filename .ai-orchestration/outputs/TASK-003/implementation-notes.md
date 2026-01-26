# TASK-003 Implementation Notes

## Architecture Decisions

1. **Dual-Platform Strategy**:
   - **Tauri**: Uses a custom Rust command `check_update` utilizing `reqwest`. This ensures robust networking, avoids CORS issues common in some WebView configurations, and allows for stricter security controls in the future.
   - **Web**: Uses standard browser `fetch` API.

2. **File Structure**:
   - `src/lib/services/versionService.ts`: Core logic for version checking and comparison.
   - `src-tauri/src/commands/system.rs`: Rust implementation for HTTP request.
   - `version.json`: Static file hosted on Vercel acting as the source of truth.

3. **Rate Limiting**:
   - Implemented a 24-hour cooldown using `localStorage`.
   - `force` parameter added to `checkForUpdates` for manual checks (e.g., "Check for Updates" button).

## Rust Changes

Modified the following files in `src-tauri`:
- Created `src/commands/system.rs`: Implements `check_update` and `VersionInfo` struct.
- Updated `src/commands/mod.rs`: Exports `system` module.
- Updated `src/lib.rs`: Registers `check_update` command handler.

## Dependencies

- `reqwest` (Rust) was already present in `Cargo.toml`.
- `@tauri-apps/api` (TS) used for `invoke`.

## Verification

- `npm run check`: Validates TypeScript types.
- `cargo check`: Validates Rust code (implicit via next build step).
- `compareVersions` logic covers semver (major.minor.patch).
