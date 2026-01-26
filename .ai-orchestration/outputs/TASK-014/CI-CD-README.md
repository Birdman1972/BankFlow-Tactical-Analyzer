# GitHub Actions CI/CD Configuration

This repository uses GitHub Actions for Continuous Integration (CI) and Continuous Deployment (CD).

## Workflows

### 1. CI (`ci.yml`)
- **Triggers**: On `push` and `pull_request` to `main` and `develop` branches.
- **Tasks**:
  - Installs dependencies (Rust & Node.js).
  - Runs TypeScript type checks (`npm run check`).
  - Builds WASM core (`./scripts/build-wasm.sh`).
  - Runs Rust unit tests (`cargo test`).
  - Builds the web application (`npm run build:web`).

### 2. Release (`release.yml`)
- **Triggers**: On pushing a tag starting with `v` (e.g., `v1.0.0`).
- **Tasks**:
  - Builds the application for **macOS** (Universal), **Windows** (x64), and **Linux** (deb/AppImage).
  - Automatically creates a GitHub Release draft.
  - Uploads the generated artifacts (.dmg, .msi, .deb, .AppImage) to the release.

## Secrets Configuration

To enable code signing and auto-publishing, configure the following secrets in **Settings > Secrets and variables > Actions**:

### Recommended (Minimal)
- `GITHUB_TOKEN`: Automatically provided by GitHub (no setup needed).

### For macOS Code Signing (Optional but Recommended)
If you have an Apple Developer Account:

| Secret Name | Description |
|---|---|
| `APPLE_CERTIFICATE` | Base64 encoded p12 certificate. |
| `APPLE_CERTIFICATE_PASSWORD` | Password for the p12 certificate. |
| `APPLE_SIGNING_IDENTITY` | The name of the identity (e.g., "Developer ID Application: Your Name"). |
| `APPLE_ID` | Your Apple ID email (for notarization). |
| `APPLE_PASSWORD` | App-specific password generated from appleid.apple.com. |

## Release Process

1. **Update Version**:
   ```bash
   npm version patch  # or minor/major
   # This updates package.json and creates a git tag
   ```

2. **Push to GitHub**:
   ```bash
   git push origin main --tags
   ```

3. **Monitor Action**:
   - Go to the "Actions" tab in GitHub.
   - Watch the "Release" workflow.
   - Once complete, go to "Releases" to verify the draft and publish it.
