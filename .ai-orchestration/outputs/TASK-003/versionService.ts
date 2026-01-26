import { invoke } from '@tauri-apps/api/core';
import { currentPlatform } from '../stores/platform';
import { get } from 'svelte/store';
// @ts-ignore - Importing package.json is enabled in tsconfig
import pkg from '../../../package.json';

// Types matches Rust struct and JSON
export interface ChangelogItem {
  version: string;
  date: string;
  changes: string[];
}

export interface VersionInfo {
  version: string;
  releaseDate: string;
  changelog: ChangelogItem[];
}

const VERSION_JSON_URL = 'https://bankflow-tactical-analyzer.vercel.app/version.json';
const CHECK_INTERVAL = 24 * 60 * 60 * 1000; // 24 hours
const STORAGE_KEY = 'bankflow_last_update_check';

/**
 * Compare two semver strings
 * Returns: 1 if v1 > v2, -1 if v1 < v2, 0 if equal
 */
export function compareVersions(v1: string, v2: string): number {
  const parts1 = v1.split('.').map(Number);
  const parts2 = v2.split('.').map(Number);
  
  for (let i = 0; i < Math.max(parts1.length, parts2.length); i++) {
    const p1 = parts1[i] || 0;
    const p2 = parts2[i] || 0;
    if (p1 > p2) return 1;
    if (p1 < p2) return -1;
  }
  return 0;
}

/**
 * Check for updates
 * @param force If true, ignores rate limiting
 */
export async function checkForUpdates(force: boolean = false): Promise<VersionInfo | null> {
  // Check rate limit
  if (!force) {
    const lastCheck = localStorage.getItem(STORAGE_KEY);
    if (lastCheck) {
      const lastTime = parseInt(lastCheck, 10);
      // If checked within last 24 hours, skip
      if (Date.now() - lastTime < CHECK_INTERVAL) {
        console.log('Skipping update check (rate limited)');
        return null;
      }
    }
  }

  try {
    let remoteInfo: VersionInfo;
    const platform = get(currentPlatform);

    console.log(`Checking for updates... (Platform: ${platform})`);

    if (platform === 'tauri') {
      // Use Rust backend to avoid CORS and ensure stability
      remoteInfo = await invoke<VersionInfo>('check_update', { url: VERSION_JSON_URL });
    } else {
      // Use standard fetch for Web
      const res = await fetch(VERSION_JSON_URL);
      if (!res.ok) throw new Error(`Fetch failed: ${res.statusText}`);
      remoteInfo = await res.json();
    }

    // Update last check time
    localStorage.setItem(STORAGE_KEY, Date.now().toString());

    const currentVersion = pkg.version;
    console.log(`Current: ${currentVersion}, Remote: ${remoteInfo.version}`);

    if (compareVersions(remoteInfo.version, currentVersion) > 0) {
      return remoteInfo;
    }
  } catch (error) {
    console.warn('Update check failed:', error);
    // Silent failure as per requirement
  }

  return null;
}
