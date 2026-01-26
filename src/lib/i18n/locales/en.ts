/**
 * English Translations
 * TODO: Will be replaced by Gemini CLI output
 */

import type { Translations } from '../types';

const en: Translations = {
  app: {
    title: 'BankFlow Tactical Analyzer',
    subtitle: 'Digital Forensics Tool',
    clearAll: 'Clear All',
    footer: 'Built with Tauri + Svelte + Rust | Offline-First Design',
  },
  dropZone: {
    transactionFile: 'Drop Transaction File',
    ipLogFile: 'Drop IP Log File',
    fileFormat: '(.xlsx)',
    rows: 'rows',
  },
  controlPanel: {
    title: 'Control Panel',
    hideSensitive: 'Hide Sensitive Columns',
    splitIncomeExpense: 'Split Income/Expense',
    ipCrossReference: 'IP Cross-Reference',
    whoisLookup: 'Whois Lookup (OpSec Risk)',
    analyzing: 'Analyzing...',
    executeAnalysis: 'Execute Analysis',
    exportReport: 'Export Report',
  },
  resultSummary: {
    title: 'Analysis Results',
    totalRecords: 'Total Records',
    ipMatched: 'IP Matched',
    multiIp: 'Multi-IP',
    requiresReview: 'Requires Review',
    whoisQueries: 'Whois Queries',
    settingsUsed: 'Settings Used:',
    sensitiveHidden: 'Sensitive Hidden',
    incomeExpenseSplit: 'Income/Expense Split',
    ipCrossRef: 'IP Cross-Ref',
    whoisEnabled: 'Whois Enabled',
    yes: 'Yes',
    no: 'No',
  },
  warningBanner: {
    title: 'OpSec Warning',
    whoisWarning: 'Whois lookup is enabled. This will make network requests that may be logged.',
    generalWarning: 'Be cautious when handling sensitive financial data.',
  },
  logConsole: {
    title: 'Log Console',
    clear: 'Clear',
  },
  system: {
    appVersion: 'BankFlow Tactical Analyzer v{version}',
    awaitingInput: 'Awaiting file input...',
    stateReset: 'State reset',
    logCleared: 'Log cleared',
  },
};

export default en;
