# Packaging & Distribution Notes

## 1. Prerequisites

### macOS (.app, .dmg)
- **Xcode Command Line Tools**: Install via `xcode-select --install`.
- **Rust target**: `rustup target add x86_64-apple-darwin` and `aarch64-apple-darwin` (for Universal binary).
- **Signing Identity** (Optional but recommended): Apple Developer ID Application certificate.
  - Set `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY` env vars.

### Windows (.msi, .exe)
- **C++ Build Tools**: Install via Visual Studio Installer.
- **WiX Toolset v3**: Required for MSI generation.
- **NSIS**: Required for standard exe installer (optional if only using MSI).

### Linux (.deb, .AppImage)
- **Dependencies**: `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `file`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`.
- **AppImage**: Requires `fuse` on some systems.

## 2. Configuration Details

- **Bundle ID**: `com.bankflow.tactical-analyzer`
- **Version**: Synced with `package.json` (Currently 2.1.0).
- **Icons**: Located in `src-tauri/icons/`.
- **File Associations**: Registered for `.xlsx` and `.xls`.

## 3. Building

To build for the current platform:

```bash
npm run build:tauri
```

To build for a specific target (e.g., debug):

```bash
npm run tauri build -- --debug
```

## 4. Artifacts

After building, artifacts will be found in:

- `src-tauri/target/release/bundle/dmg/*.dmg` (macOS)
- `src-tauri/target/release/bundle/msi/*.msi` (Windows)
- `src-tauri/target/release/bundle/deb/*.deb` (Linux)
- `src-tauri/target/release/bundle/appimage/*.AppImage` (Linux)

## 5. Troubleshooting

- **Icon errors**: Ensure all icon sizes exist in `src-tauri/icons/`.
- **Signing errors**: Check if certificates are valid and environment variables are set correctly.
- **WiX errors**: Ensure WiX Toolset is in your PATH (Windows).
