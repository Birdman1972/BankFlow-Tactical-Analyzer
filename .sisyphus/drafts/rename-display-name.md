# Draft: Rename Product Display Name (BankFlow 戰術分析器 -> 金流分析器)

## Requirements (confirmed)
- Rename the product display name from `BankFlow 戰術分析器` to `金流分析器` across:
  - Web UI (including HTML title where applicable)
  - Tauri desktop metadata (display-facing fields only)
  - User-facing docs (`README.md`, `docs/*.md`, `docs/USER_MANUAL_zh-TW.md`, etc.)
- Keep deployment risk minimal for Vercel and package identities.
- Provide a step-by-step checklist with file-by-file edits, safe defaults, explicit exclusions, and verification commands.

## Must NOT Rename (guardrails)
- Repo slug / GitHub repo name
- Vercel project/domain and URLs
- npm package name(s) (e.g. `package.json:name`)
- Tauri bundle identifier / app identifier
- Updater/version endpoints and release URLs that contain `bankflow-tactical-analyzer`

## Known Repo Facts (from user + quick scan)
- `index.html` includes `<title>BankFlow Tactical Analyzer</title>`.
- i18n:
  - `src/lib/i18n/locales/zh-TW.ts` includes `BankFlow 戰術分析器`.
  - `src/lib/i18n/locales/en.ts` includes `BankFlow Tactical Analyzer`.
- Tauri config exists at `src-tauri/tauri.conf.json` and includes `productName`, `title`, `longDescription` mentioning `BankFlow Tactical Analyzer`.
- Docs include many references to `Tactical Analyzer` and `https://bankflow-tactical-analyzer.vercel.app`.
- Version/update related URLs include `bankflow-tactical-analyzer`:
  - `src/lib/services/versionService.ts` has `https://bankflow-tactical-analyzer.vercel.app/version.json`.
  - UI opens GitHub releases: `https://github.com/project-bob/bankflow-tactical-analyzer/releases`.

## Open Questions
- Should the English display name also change (e.g. `BankFlow Tactical Analyzer` -> something else), or keep English as-is and only change zh-TW display strings?
- For Tauri `productName`: do we want fully `金流分析器`, or keep brand prefix like `BankFlow 金流分析器` for continuity?

## Scope Boundaries
- INCLUDE: User-facing display strings and descriptions.
- EXCLUDE: Identifiers, slugs, domains, updater URLs, release URLs.
