/**
 * Tauri API Wrapper
 *
 * Provides typed interfaces for Tauri commands
 */

import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { open, save } from '@tauri-apps/plugin-dialog';
import {
  fileA,
  fileB,
  isAnalyzing,
  analysisResult,
  progress,
  settings,
  addLog,
  type ProgressInfo,
} from './app';
import { get } from 'svelte/store';

// ============================================
// Tauri Response Types (snake_case from Rust)
// ============================================

interface TauriFileMetadata {
  path: string;
  filename: string;
  row_count: number;
  column_count: number;
  file_type: string;
}

interface TauriAnalysisResult {
  total_records: number;
  matched_count: number;
  multi_ip_count: number;
  whois_queried: number;
  settings: {
    hide_sensitive: boolean;
    split_income_expense: boolean;
    ip_cross_reference: boolean;
    whois_lookup: boolean;
  };
}

interface TauriWhoisResult {
  ip: string;
  country: string | null;
  isp: string | null;
  query_success: boolean;
}

// ============================================
// Utility & Environment
// ============================================

// Mock Data Definitions
const MOCK_FILE_METADATA: TauriFileMetadata = {
  path: '/mock/path/to/file.xlsx',
  filename: 'mock_data.xlsx',
  row_count: 1234,
  column_count: 15,
  file_type: 'xlsx'
};

const MOCK_ANALYSIS_RESULT: TauriAnalysisResult = {
  total_records: 1234,
  matched_count: 56,
  multi_ip_count: 3,
  whois_queried: 12,
  settings: {
    hide_sensitive: false,
    split_income_expense: true,
    ip_cross_reference: true,
    whois_lookup: false
  }
};

/**
 * Helper to handle Tauri invocation errors and return mocks in browser
 */
function handleTauriError<T>(error: unknown, actionName: string, mockData?: T): T {
  const msg = String(error);
  
  // Check for common environment errors
  if (msg.includes('__TAURI_INTERNALS__') || msg.includes('ipc not connected')) {
    const friendlyMsg = `[Browser Mode] Mocking ${actionName} response.`;
    addLog('warning', friendlyMsg);
    console.warn(friendlyMsg, error);
    
    if (mockData) {
      return mockData;
    }
    
    throw new Error(`Tauri API unavailable and no mock data provided for ${actionName}`);
  }
  
  // Pass through other errors
  throw error;
}

// ============================================
// File Operations
// ============================================

export async function loadFileA(path: string): Promise<void> {
  try {
    addLog('info', `Loading File A: ${path.split(/[/\\]/).pop()}`);
    
    const result = await invoke<TauriFileMetadata>('load_file', { path })
      .catch(err => handleTauriError(err, 'load_file', { ...MOCK_FILE_METADATA, filename: path.split(/[/\\]/).pop() || 'mock.xlsx' }));

    fileA.set({
      path: result.path,
      filename: result.filename,
      rowCount: result.row_count,
      columnCount: result.column_count,
      fileType: result.file_type,
    });

    addLog('success', `File A loaded: ${result.filename} (${result.row_count} rows)`);
  } catch (error) {
    addLog('error', `Failed to load File A: ${error}`);
  }
}

export async function selectAndLoadFileA(): Promise<void> {
  try {
    // In browser, open() will fail. We can't really mock the dialog selection easily 
    // without UI changes, but we can catch it.
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Excel', extensions: ['xlsx', 'xls'] }],
      title: 'Select Transaction File (File A)',
    }).catch(err => handleTauriError(err, 'open_dialog', null));

    if (selected && typeof selected === 'string') {
      await loadFileA(selected);
    } else if (!selected) {
        // If mocked with null (browser), we can optionally auto-load a mock file for testing
        // For now, let's just log.
        console.log('Dialog cancelled or mocked with null');
    }
  } catch (error) {
    addLog('error', `Failed to select File A: ${error}`);
  }
}

export async function loadFileB(path: string): Promise<void> {
  try {
    addLog('info', `Loading File B: ${path.split(/[/\\]/).pop()}`);
    const result = await invoke<TauriFileMetadata>('load_ip_file', { path })
      .catch(err => handleTauriError(err, 'load_ip_file', { ...MOCK_FILE_METADATA, filename: path.split(/[/\\]/).pop() || 'mock_ip.xlsx', row_count: 500 }));

    fileB.set({
      path: result.path,
      filename: result.filename,
      rowCount: result.row_count,
      columnCount: result.column_count,
      fileType: result.file_type,
    });

    addLog('success', `File B loaded: ${result.filename} (${result.row_count} rows)`);
  } catch (error) {
    addLog('error', `Failed to load File B: ${error}`);
  }
}

export async function selectAndLoadFileB(): Promise<void> {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Excel', extensions: ['xlsx', 'xls'] }],
      title: 'Select IP Log File (File B)',
    }).catch(err => handleTauriError(err, 'open_dialog', null));

    if (selected && typeof selected === 'string') {
      await loadFileB(selected);
    }
  } catch (error) {
    addLog('error', `Failed to select File B: ${error}`);
  }
}

// ============================================
// Analysis
// ============================================

let progressUnlisten: UnlistenFn | null = null;

export async function runAnalysis(): Promise<void> {
  const currentSettings = get(settings);

  try {
    isAnalyzing.set(true);
    progress.set({ stage: 'starting', progress: 0, message: 'Initializing analysis...' });
    addLog('info', 'Starting analysis...');

    // Listen for progress events
    try {
      progressUnlisten = await listen<ProgressInfo>('analysis-progress', (event) => {
        progress.set(event.payload);
        addLog('info', `[${event.payload.stage}] ${event.payload.message}`);
      });
    } catch (err) {
      console.warn('Failed to setup progress listener (likely non-Tauri env)', err);
    }

    // Run analysis
    const result = await invoke<TauriAnalysisResult>('run_analysis', {
      hideSensitive: currentSettings.hideSensitive,
      splitIncomeExpense: currentSettings.splitIncomeExpense,
      ipCrossReference: currentSettings.ipCrossReference,
      whoisLookup: currentSettings.whoisLookup,
    }).catch(err => handleTauriError(err, 'run_analysis', MOCK_ANALYSIS_RESULT));

    analysisResult.set({
      totalRecords: result.total_records,
      matchedCount: result.matched_count,
      multiIpCount: result.multi_ip_count,
      whoisQueried: result.whois_queried,
      settings: {
        hideSensitive: result.settings.hide_sensitive,
        splitIncomeExpense: result.settings.split_income_expense,
        ipCrossReference: result.settings.ip_cross_reference,
        whoisLookup: result.settings.whois_lookup,
      },
    });

    addLog('success', `Analysis complete: ${result.matched_count}/${result.total_records} records matched`);

    if (result.multi_ip_count > 0) {
      addLog('warning', `${result.multi_ip_count} transactions have multiple IP matches`);
    }
  } catch (error) {
    addLog('error', `Analysis failed: ${error}`);
  } finally {
    isAnalyzing.set(false);
    progress.set(null);
    if (progressUnlisten) {
      progressUnlisten();
      progressUnlisten = null;
    }
  }
}

// ============================================
// Export
// ============================================

export async function exportReport(): Promise<void> {
  try {
    const defaultFileName = `bankflow_report_${new Date().toISOString().split('T')[0]}.xlsx`;
    const outputPath = await save({
      filters: [{ name: 'Excel', extensions: ['xlsx'] }],
      defaultPath: defaultFileName,
      title: 'Save Analysis Report',
    }).catch(err => handleTauriError<string | null>(err, 'save_dialog', null));

    if (outputPath) {
      addLog('info', `Exporting report to: ${outputPath.split('/').pop()}`);

      const mockExportResult = `[Mock] Report exported to ${defaultFileName}`;
      const result = await invoke<string>('export_excel', { outputPath })
        .catch(err => handleTauriError(err, 'export_excel', mockExportResult));
      addLog('success', result);
    } else {
      // Browser mock mode - simulate export
      addLog('warning', '[Browser Mode] Export simulated - in real app, file would be saved');
    }
  } catch (error) {
    addLog('error', `Export failed: ${error}`);
  }
}

// ============================================
// Whois (Standalone)
// ============================================

export interface WhoisResult {
  ip: string;
  country: string | null;
  isp: string | null;
  querySuccess: boolean;
}

export async function queryWhois(ip: string): Promise<WhoisResult> {
  try {
    // Mock whois data for browser mode
    const mockWhoisResult: TauriWhoisResult = {
      ip,
      country: 'Taiwan',
      isp: 'Mock ISP Provider',
      query_success: true,
    };

    const result = await invoke<TauriWhoisResult>('query_whois', { ip })
      .catch(err => handleTauriError(err, 'query_whois', mockWhoisResult));

    return {
      ip: result.ip,
      country: result.country,
      isp: result.isp,
      querySuccess: result.query_success,
    };
  } catch (error) {
    addLog('error', `Whois query failed for ${ip}: ${error}`);
    // Return dummy data instead of crashing
    return { ip, country: 'Error', isp: String(error), querySuccess: false };
  }
}

// ============================================
// Utility
// ============================================

export async function clearAllFiles(): Promise<void> {
  try {
    // Try to clear backend first
    await invoke('clear_files').catch(err => {
        // If backend fails (e.g. browser), we still clear frontend
        console.warn('Backend clear failed, proceeding with frontend clear', err);
    });
    
    fileA.set(null);
    fileB.set(null);
    analysisResult.set(null);
    addLog('info', 'All files cleared');
  } catch (error) {
    addLog('error', `Failed to clear files: ${error}`);
  }
}
