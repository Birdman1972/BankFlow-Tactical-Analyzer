/**
 * Tauri Platform Implementation
 *
 * Implements PlatformAPI for Tauri desktop environment.
 */

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { PlatformAPI, WhoisResult, BatchScanResult } from "./platform";
import type {
  FileInfo,
  AnalysisSettings,
  AnalysisResult,
  ProgressInfo,
} from "./app";
import { addLog } from "./app";

// ============================================
// Tauri Response Types (snake_case from Rust)
// ============================================

interface TauriFileMetadata {
  path: string | null;
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
// Tauri Platform Implementation
// ============================================

export class TauriPlatform implements PlatformAPI {
  readonly platformName = "Tauri Desktop";
  readonly supportsWhois = true;
  readonly supportsFileDialog = true;

  private progressUnlisten: UnlistenFn | null = null;

  // ----------------------------------------
  // File Operations
  // ----------------------------------------

  async selectAndLoadFileA(): Promise<FileInfo> {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Excel", extensions: ["xlsx", "xls"] }],
      title: "Select Transaction File (File A)",
    });

    if (!selected || typeof selected !== "string") {
      throw new Error("No file selected");
    }

    addLog("info", `Loading File A: ${selected.split("/").pop()}`);
    const result = await invoke<TauriFileMetadata>("load_file", {
      path: selected,
    });

    const fileInfo: FileInfo = {
      path: result.path ?? selected,
      filename: result.filename,
      rowCount: result.row_count,
      columnCount: result.column_count,
      fileType: result.file_type,
      isValid: true,
    };

    addLog(
      "success",
      `File A loaded: ${fileInfo.filename} (${fileInfo.rowCount} rows)`,
    );
    return fileInfo;
  }

  async loadFileA(
    path: string,
    mapping?: Record<string, string>,
  ): Promise<FileInfo> {
    addLog("info", `Loading File A: ${path.split(/[/\\]/).pop()}`);
    const result = await invoke<TauriFileMetadata>("load_file", {
      path,
      mapping,
    });

    return {
      path: result.path ?? path,
      filename: result.filename,
      rowCount: result.row_count,
      columnCount: result.column_count,
      fileType: result.file_type,
      isValid: true,
    };
  }

  async selectAndLoadFileB(): Promise<FileInfo> {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Excel", extensions: ["xlsx", "xls"] }],
      title: "Select IP Log File (File B)",
    });

    if (!selected || typeof selected !== "string") {
      throw new Error("No file selected");
    }

    return this.loadFileB(selected);
  }

  async loadFileB(
    path: string,
    mapping?: Record<string, string>,
  ): Promise<FileInfo> {
    addLog("info", `Loading File B: ${path.split(/[/\\]/).pop()}`);
    const result = await invoke<TauriFileMetadata>("load_ip_file", {
      path,
      mapping,
    });

    return {
      path: result.path ?? path,
      filename: result.filename,
      rowCount: result.row_count,
      columnCount: result.column_count,
      fileType: result.file_type,
      isValid: true,
    };
  }

  async clearAllFiles(): Promise<void> {
    await invoke("clear_files");
    addLog("info", "All files cleared");
  }

  // ----------------------------------------
  // Analysis
  // ----------------------------------------

  async runAnalysis(
    settings: AnalysisSettings,
    onProgress?: (progress: ProgressInfo) => void,
  ): Promise<AnalysisResult> {
    try {
      addLog("info", "Starting analysis...");

      // Listen for progress events
      if (onProgress) {
        this.progressUnlisten = await listen<ProgressInfo>(
          "analysis-progress",
          (event) => {
            onProgress(event.payload);
            addLog("info", `[${event.payload.stage}] ${event.payload.message}`);
          },
        );
      }

      // Run analysis
      const result = await invoke<TauriAnalysisResult>("run_analysis", {
        hideSensitive: settings.hideSensitive,
        splitIncomeExpense: settings.splitIncomeExpense,
        ipCrossReference: settings.ipCrossReference,
        whoisLookup: settings.whoisLookup,
      });

      const analysisResult: AnalysisResult = {
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
      };

      addLog(
        "success",
        `Analysis complete: ${analysisResult.matchedCount}/${analysisResult.totalRecords} records matched`,
      );

      if (analysisResult.multiIpCount > 0) {
        addLog(
          "warning",
          `${analysisResult.multiIpCount} transactions have multiple IP matches`,
        );
      }

      return analysisResult;
    } finally {
      if (this.progressUnlisten) {
        this.progressUnlisten();
        this.progressUnlisten = null;
      }
    }
  }

  // ----------------------------------------
  // Export
  // ----------------------------------------

  async exportReport(): Promise<string> {
    const outputPath = await save({
      filters: [{ name: "Excel", extensions: ["xlsx"] }],
      defaultPath: `bankflow_report_${new Date().toISOString().split("T")[0]}.xlsx`,
      title: "Save Analysis Report",
    });

    if (!outputPath) {
      throw new Error("No output path selected");
    }

    addLog("info", `Exporting report to: ${outputPath.split("/").pop()}`);
    const result = await invoke<string>("export_excel", { outputPath });
    addLog("success", result);

    return result;
  }

  // ----------------------------------------
  // Whois
  // ----------------------------------------

  async queryWhois(ip: string): Promise<WhoisResult> {
    try {
      const result = await invoke<TauriWhoisResult>("query_whois", { ip });
      return {
        ip: result.ip,
        country: result.country,
        isp: result.isp,
        querySuccess: result.query_success,
      };
    } catch (error) {
      addLog("error", `Whois query failed for ${ip}: ${error}`);
      throw error;
    }
  }

  // ----------------------------------------
  // Batch Scan
  // ----------------------------------------

  async scanFolder(
    path: string,
    maxDepth: number = 3,
  ): Promise<BatchScanResult> {
    try {
      addLog("info", `Scanning folder: ${path.split(/[/\\]/).pop()}`);
      const result = await invoke<BatchScanResult>("scan_folder", {
        path,
        max_depth: maxDepth,
      });
      addLog(
        "success",
        `Scan complete: Found ${result.pairs.length} analysis pairs.`,
      );
      return result;
    } catch (error) {
      addLog("error", `Scan failed: ${error}`);
      throw error;
    }
  }
}
