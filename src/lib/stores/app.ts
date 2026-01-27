/**
 * Application State Stores
 *
 * Manages global state for BankFlow Fund Flow Analyzer
 */

import { writable, derived } from 'svelte/store';
import pkg from '../../../package.json';

// ============================================
// Types
// ============================================

export interface FileInfo {
  path: string;
  filename: string;
  rowCount: number;
  columnCount: number;
  fileType: string;
}

export interface AnalysisSettings {
  hideSensitive: boolean;
  splitIncomeExpense: boolean;
  ipCrossReference: boolean;
  whoisLookup: boolean;
}

export interface AnalysisResult {
  totalRecords: number;
  matchedCount: number;
  multiIpCount: number;
  whoisQueried: number;
  settings: AnalysisSettings;
}

export interface LogEntry {
  timestamp: Date;
  level: 'info' | 'success' | 'warning' | 'error' | 'system';
  message: string;
}

export interface ProgressInfo {
  stage: string;
  progress: number;
  message: string;
}

// ============================================
// Stores
// ============================================

// File state
export const fileA = writable<FileInfo | null>(null);
export const fileB = writable<FileInfo | null>(null);

// Analysis settings
export const settings = writable<AnalysisSettings>({
  hideSensitive: false,
  splitIncomeExpense: true,
  ipCrossReference: true,
  whoisLookup: false,
});

// Analysis state
export const isAnalyzing = writable<boolean>(false);
export const analysisResult = writable<AnalysisResult | null>(null);
export const progress = writable<ProgressInfo | null>(null);

// Log entries
export const logs = writable<LogEntry[]>([
  {
    timestamp: new Date(),
    level: 'system',
    message: `BankFlow 金流分析器 v${pkg.version}`,
  },
  {
    timestamp: new Date(),
    level: 'info',
    message: 'Awaiting file input...',
  },
]);

// ============================================
// Derived Stores
// ============================================

// Check if ready to analyze
export const canAnalyze = derived(
  [fileA, fileB, isAnalyzing],
  ([$fileA, $fileB, $isAnalyzing]) => {
    return $fileA !== null && $fileB !== null && !$isAnalyzing;
  }
);

// Check if can export
export const canExport = derived(
  [analysisResult, isAnalyzing],
  ([$analysisResult, $isAnalyzing]) => {
    return $analysisResult !== null && !$isAnalyzing;
  }
);

// ============================================
// Actions
// ============================================

export function addLog(level: LogEntry['level'], message: string) {
  logs.update((entries) => [
    ...entries,
    {
      timestamp: new Date(),
      level,
      message,
    },
  ]);
}

export function clearLogs() {
  logs.set([
    {
      timestamp: new Date(),
      level: 'system',
      message: 'Log cleared',
    },
  ]);
}

export function resetState() {
  fileA.set(null);
  fileB.set(null);
  analysisResult.set(null);
  progress.set(null);
  isAnalyzing.set(false);
  addLog('system', 'State reset');
}
