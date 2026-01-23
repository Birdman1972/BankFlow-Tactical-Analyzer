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
// File Operations
// ============================================

export async function selectAndLoadFileA(): Promise<void> {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Excel', extensions: ['xlsx', 'xls'] }],
      title: 'Select Transaction File (File A)',
    });

    if (selected && typeof selected === 'string') {
      addLog('info', `Loading File A: ${selected.split('/').pop()}`);
      const result = await invoke<TauriFileMetadata>('load_file', { path: selected });

      fileA.set({
        path: result.path,
        filename: result.filename,
        rowCount: result.row_count,
        columnCount: result.column_count,
        fileType: result.file_type,
      });

      addLog('success', `File A loaded: ${result.filename} (${result.row_count} rows)`);
    }
  } catch (error) {
    addLog('error', `Failed to load File A: ${error}`);
    throw error;
  }
}

export async function selectAndLoadFileB(): Promise<void> {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Excel', extensions: ['xlsx', 'xls'] }],
      title: 'Select IP Log File (File B)',
    });

    if (selected && typeof selected === 'string') {
      addLog('info', `Loading File B: ${selected.split('/').pop()}`);
      const result = await invoke<TauriFileMetadata>('load_ip_file', { path: selected });

      fileB.set({
        path: result.path,
        filename: result.filename,
        rowCount: result.row_count,
        columnCount: result.column_count,
        fileType: result.file_type,
      });

      addLog('success', `File B loaded: ${result.filename} (${result.row_count} rows)`);
    }
  } catch (error) {
    addLog('error', `Failed to load File B: ${error}`);
    throw error;
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
    progressUnlisten = await listen<ProgressInfo>('analysis-progress', (event) => {
      progress.set(event.payload);
      addLog('info', `[${event.payload.stage}] ${event.payload.message}`);
    });

    // Run analysis
    const result = await invoke<TauriAnalysisResult>('run_analysis', {
      hideSensitive: currentSettings.hideSensitive,
      splitIncomeExpense: currentSettings.splitIncomeExpense,
      ipCrossReference: currentSettings.ipCrossReference,
      whoisLookup: currentSettings.whoisLookup,
    });

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
    throw error;
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
    const outputPath = await save({
      filters: [{ name: 'Excel', extensions: ['xlsx'] }],
      defaultPath: `bankflow_report_${new Date().toISOString().split('T')[0]}.xlsx`,
      title: 'Save Analysis Report',
    });

    if (outputPath) {
      addLog('info', `Exporting report to: ${outputPath.split('/').pop()}`);

      const result = await invoke<string>('export_excel', { outputPath });
      addLog('success', result);
    }
  } catch (error) {
    addLog('error', `Export failed: ${error}`);
    throw error;
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
    const result = await invoke<TauriWhoisResult>('query_whois', { ip });
    return {
      ip: result.ip,
      country: result.country,
      isp: result.isp,
      querySuccess: result.query_success,
    };
  } catch (error) {
    addLog('error', `Whois query failed for ${ip}: ${error}`);
    throw error;
  }
}

// ============================================
// Utility
// ============================================

export async function clearAllFiles(): Promise<void> {
  try {
    await invoke('clear_files');
    fileA.set(null);
    fileB.set(null);
    analysisResult.set(null);
    addLog('info', 'All files cleared');
  } catch (error) {
    addLog('error', `Failed to clear files: ${error}`);
    throw error;
  }
}
