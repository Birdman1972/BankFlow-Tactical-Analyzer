/**
 * Store exports
 *
 * Platform-agnostic API for BankFlow Tactical Analyzer
 */

// Application state
export * from './app';

// Platform abstraction (recommended for new code)
export {
  currentPlatform,
  initializePlatform,
  getPlatform,
  selectAndLoadFileA,
  selectAndLoadFileB,
  clearAllFiles,
  runAnalysis,
  exportReport,
  queryWhois,
  supportsWhois,
  supportsFileDialog,
  getPlatformName,
  type Platform,
  type PlatformAPI,
  type WhoisResult,
} from './platform';

// Legacy Tauri-specific exports (for backward compatibility)
// TODO: Migrate components to use platform abstraction, then remove this
export * from './tauri';
