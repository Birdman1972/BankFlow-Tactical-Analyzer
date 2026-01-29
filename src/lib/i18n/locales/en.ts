/**
 * English Translations
 * TODO: Will be replaced by Gemini CLI output
 */

import type { Translations } from '../types';

const en: Translations = {
  app: {
    title: 'BankFlow Fund Flow Analyzer',
    subtitle: 'Digital Forensics Tool',
    clearAll: 'Clear All',
    footer: 'Built with Tauri + Svelte + Rust | Offline-First Design',
  },
  modern: {
    title: 'Modern UI',
    backToClassic: 'Back to Classic',
    themeLight: 'Light',
    themeDark: 'Dark',
    uiModeClassic: 'Classic',
    uiModeModern: 'Modern',
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
    appVersion: 'BankFlow Fund Flow Analyzer v{version}',
    awaitingInput: 'Awaiting file input...',
    stateReset: 'State reset',
    logCleared: 'Log cleared',
  },
  updateDialog: {
    title: 'Update Available',
    newVersion: 'New Version',
    releaseNotes: 'What\'s New:',
    updateNow: 'Update Now',
    remindLater: 'Remind Me Later',
    skipVersion: 'Skip This Version',
  },
  downloadsDialog: {
    open: 'Downloads',
    title: 'Download Desktop App',
    latestVersion: 'Latest Version',
    loading: 'Loading version info...',
    loadError: 'Unable to load version info (version.json)',
    openReleases: 'Open GitHub Releases',
    downloadWindowsMsi: 'Download Windows MSI',
    downloadPortableZip: 'Download Portable ZIP',
    macosNote: 'macOS: download from GitHub Releases.',
    releaseNotes: 'Release Notes',
    usageTitle: 'How to Use',
    usageStep1: 'Prepare Data: Excel files for Transactions (File A) and IP Logs (File B).',
    usageStep2: 'Drag & Drop: Drop files into their respective target areas.',
    usageStep3: 'Run Analysis: Adjust settings and click "Execute Analysis".',
    usageStep4: 'Export Report: Click "Export Report" to save results after completion.',
    featureTitle: 'Key Features',
    feature1: 'Offline-First: All processing stays local for maximum OpSec.',
    feature2: 'High Performance: Rust core engine handles large datasets smoothly.',
    feature3: 'IP Correlation: Matches transaction time with IP logs (±2s window).',
    close: 'Close',
  },
  feedbackForm: {
    title: 'Send Feedback',
    typeLabel: 'Feedback Type',
    typeFeature: 'Feature Request',
    typeBug: 'Bug Report',
    typeUx: 'User Experience',
    titleLabel: 'Title',
    titlePlaceholder: 'Short summary',
    descriptionLabel: 'Details',
    descriptionPlaceholder: 'Describe the issue or suggestion...',
    attachmentsLabel: 'Attachments (links)',
    attachmentsPlaceholder: 'https://... (one link per line)',
    attachmentsNote: 'Links only. File uploads are not supported.',
    versionLabel: 'Version',
    submit: 'Submit Feedback',
    submitting: 'Submitting...',
    success: 'Thanks! Your feedback was sent.',
    queued: 'Thanks! We saved your feedback and will retry when online.',
    error: 'Unable to send. We\'ll retry later.',
    privacyNote: 'Please avoid including personal or sensitive information.',
    errors: {
      typeRequired: 'Please select a feedback type.',
      titleRequired: 'Title is required.',
      descriptionRequired: 'Details are required.',
      attachmentsInvalid: 'Only http/https links are supported.',
    },
  },
  nav: {
    home: 'Home',
    about: 'About',
    feedback: 'Feedback',
    collapse: 'Collapse Sidebar',
    expand: 'Expand Sidebar',
  },
  shortcuts: {
    title: 'Keyboard Shortcuts',
    close: 'Close',
    openFileA: 'Open File A',
    openFileB: 'Open File B',
    runAnalysis: 'Run Analysis',
    exportReport: 'Export Report',
    clearAll: 'Clear All Files',
    closeDialog: 'Close Dialog',
    showHelp: 'Show Shortcuts Help',
    note: 'Shortcuts are disabled while typing in input fields (except Escape).',
  },
  about: {
    title: 'About BankFlow Fund Flow Analyzer',
    subtitle: 'Offline digital forensics analysis tool',
    currentVersion: 'Current Version',
    versionInfo: 'Version Information',
    releaseDate: 'Release Date',
    changelog: 'Changelog',
    loading: 'Loading...',
    loadError: 'Unable to load version history.',
    changelogEmpty: 'No changelog entries available.',
    developer: 'Developer',
    license: 'License',
    licenseText: `MIT License

Copyright (c) 2026 Antigravity AICoder

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.`,
  },
};

export default en;
