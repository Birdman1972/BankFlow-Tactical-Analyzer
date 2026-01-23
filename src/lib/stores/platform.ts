/**
 * Platform Abstraction Layer
 *
 * Provides unified API for both Tauri (desktop) and WASM (web) platforms.
 * Auto-detects the current environment and routes calls appropriately.
 */

import { writable } from 'svelte/store';
import type { FileInfo, AnalysisSettings, AnalysisResult, ProgressInfo } from './app';
import { addLog } from './app';

// ============================================
// Platform Detection
// ============================================

export type Platform = 'tauri' | 'web' | 'unknown';

function detectPlatform(): Platform {
  if (typeof window === 'undefined') {
    return 'unknown';
  }

  // Check for Tauri
  if ('__TAURI__' in window || '__TAURI_INTERNALS__' in window) {
    return 'tauri';
  }

  // Default to web
  return 'web';
}

export const currentPlatform = writable<Platform>(detectPlatform());

// ============================================
// Platform Interface
// ============================================

export interface PlatformAPI {
  // File Operations
  selectAndLoadFileA(): Promise<FileInfo>;
  selectAndLoadFileB(): Promise<FileInfo>;
  clearAllFiles(): Promise<void>;

  // Analysis
  runAnalysis(
    settings: AnalysisSettings,
    onProgress?: (progress: ProgressInfo) => void
  ): Promise<AnalysisResult>;

  // Export
  exportReport(): Promise<string>;

  // Whois (may not be available on web)
  queryWhois?(ip: string): Promise<WhoisResult>;

  // Platform info
  readonly platformName: string;
  readonly supportsWhois: boolean;
  readonly supportsFileDialog: boolean;
}

export interface WhoisResult {
  ip: string;
  country: string | null;
  isp: string | null;
  querySuccess: boolean;
}

// ============================================
// Platform Implementation Registry
// ============================================

let platformImpl: PlatformAPI | null = null;

export function registerPlatform(impl: PlatformAPI): void {
  platformImpl = impl;
  addLog('system', `Platform initialized: ${impl.platformName}`);
}

export function getPlatform(): PlatformAPI {
  if (!platformImpl) {
    throw new Error('Platform not initialized. Call initializePlatform() first.');
  }
  return platformImpl;
}

// ============================================
// Platform Initialization
// ============================================

export async function initializePlatform(): Promise<PlatformAPI> {
  const platform = detectPlatform();
  currentPlatform.set(platform);

  if (platform === 'tauri') {
    // Dynamic import for Tauri
    const { TauriPlatform } = await import('./tauri-impl');
    const impl = new TauriPlatform();
    registerPlatform(impl);
    return impl;
  } else if (platform === 'web') {
    // Dynamic import for WASM
    const { WasmPlatform } = await import('./wasm-impl');
    const impl = new WasmPlatform();
    await impl.initialize();
    registerPlatform(impl);
    return impl;
  } else {
    throw new Error('Unknown platform');
  }
}

// ============================================
// Convenience Functions (delegates to current platform)
// ============================================

export async function selectAndLoadFileA(): Promise<FileInfo> {
  return getPlatform().selectAndLoadFileA();
}

export async function selectAndLoadFileB(): Promise<FileInfo> {
  return getPlatform().selectAndLoadFileB();
}

export async function clearAllFiles(): Promise<void> {
  return getPlatform().clearAllFiles();
}

export async function runAnalysis(
  settings: AnalysisSettings,
  onProgress?: (progress: ProgressInfo) => void
): Promise<AnalysisResult> {
  return getPlatform().runAnalysis(settings, onProgress);
}

export async function exportReport(): Promise<string> {
  return getPlatform().exportReport();
}

export async function queryWhois(ip: string): Promise<WhoisResult | null> {
  const platform = getPlatform();
  if (platform.supportsWhois && platform.queryWhois) {
    return platform.queryWhois(ip);
  }
  addLog('warning', 'Whois lookup is not available on this platform');
  return null;
}

// ============================================
// Platform Capability Checks
// ============================================

export function supportsWhois(): boolean {
  return platformImpl?.supportsWhois ?? false;
}

export function supportsFileDialog(): boolean {
  return platformImpl?.supportsFileDialog ?? false;
}

export function getPlatformName(): string {
  return platformImpl?.platformName ?? 'Unknown';
}
