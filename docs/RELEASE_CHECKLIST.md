# Release Checklist (v0.3.9)

## 1) Pre-Release
- [ ] Verify `cargo test` passes (core)
- [ ] Verify `cargo bench --bench pipeline` baseline is recorded (optional)
- [ ] Run `cargo run --bin verify_memory` to log RSS (optional)
- [ ] Confirm `docs/changelog/v0.3.9.md` is complete
- [ ] Update `.github/RELEASE_TEMPLATE.md` if needed

## 2) Tag + CI
- [ ] Create tag `v0.3.9`
- [ ] Push tag to trigger `release.yml`
- [ ] Monitor CI for Windows/macOS/Linux artifacts

## 3) Assets
- [ ] Confirm MSI, DMG, AppImage/DEB uploaded
- [ ] Run `scripts/build-portable-zip.sh` after local build if needed

## 4) Publish
- [ ] Publish GitHub Release with template
- [ ] Verify release notes and download links
