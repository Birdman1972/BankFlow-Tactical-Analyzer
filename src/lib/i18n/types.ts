/**
 * i18n Type Definitions
 *
 * Defines the structure for all translatable strings in BankFlow 金流分析器
 */

// Supported locales
export type Locale = 'en' | 'zh-TW';

// Translation structure
export interface Translations {
  // App-level strings
  app: {
    title: string;
    subtitle: string;
    clearAll: string;
    footer: string;
  };

  // Modern UI shell
  modern: {
    title: string;
    backToClassic: string;
    themeLight: string;
    themeDark: string;
    uiModeClassic: string;
    uiModeModern: string;
  };

  // DropZone component
  dropZone: {
    transactionFile: string;
    ipLogFile: string;
    fileFormat: string;
    rows: string;
  };

  // ControlPanel component
  controlPanel: {
    title: string;
    hideSensitive: string;
    splitIncomeExpense: string;
    ipCrossReference: string;
    whoisLookup: string;
    analyzing: string;
    executeAnalysis: string;
    exportReport: string;
  };

  // ResultSummary component
  resultSummary: {
    title: string;
    totalRecords: string;
    ipMatched: string;
    multiIp: string;
    requiresReview: string;
    whoisQueries: string;
    settingsUsed: string;
    sensitiveHidden: string;
    incomeExpenseSplit: string;
    ipCrossRef: string;
    whoisEnabled: string;
    yes: string;
    no: string;
  };

  // WarningBanner component
  warningBanner: {
    title: string;
    whoisWarning: string;
    generalWarning: string;
  };

  // LogConsole component
  logConsole: {
    title: string;
    clear: string;
  };

  // System messages (for logs)
  system: {
    appVersion: string;
    awaitingInput: string;
    stateReset: string;
    logCleared: string;
  };

  // UpdateDialog component
  updateDialog: {
    title: string;
    newVersion: string;
    releaseNotes: string;
    updateNow: string;
    remindLater: string;
    skipVersion: string;
  };

  // DownloadsDialog component
  downloadsDialog: {
    open: string;
    title: string;
    latestVersion: string;
    loading: string;
    loadError: string;
    openReleases: string;
    downloadWindowsMsi: string;
    downloadPortableZip: string;
    macosNote: string;
    releaseNotes: string;
    usageTitle: string;
    usageStep1: string;
    usageStep2: string;
    usageStep3: string;
    usageStep4: string;
    featureTitle: string;
    feature1: string;
    feature2: string;
    feature3: string;
    close: string;
  };

  // FeedbackForm component
  feedbackForm: {
    title: string;
    typeLabel: string;
    typeFeature: string;
    typeBug: string;
    typeUx: string;
    titleLabel: string;
    titlePlaceholder: string;
    descriptionLabel: string;
    descriptionPlaceholder: string;
    versionLabel: string;
    submit: string;
    submitting: string;
    success: string;
    queued: string;
    error: string;
    privacyNote: string;
    errors: {
      typeRequired: string;
      titleRequired: string;
      descriptionRequired: string;
    };
  };

  // Navigation (Sidebar)
  nav: {
    home: string;
    about: string;
    feedback: string;
    collapse: string;
    expand: string;
  };

  // Keyboard shortcuts
  shortcuts: {
    title: string;
    close: string;
    openFileA: string;
    openFileB: string;
    runAnalysis: string;
    exportReport: string;
    clearAll: string;
    closeDialog: string;
    showHelp: string;
    note: string;
  };

  // AboutPage component
  about: {
    title: string;
    subtitle: string;
    currentVersion: string;
    versionInfo: string;
    releaseDate: string;
    changelog: string;
    loading: string;
    loadError: string;
    changelogEmpty: string;
    developer: string;
    license: string;
    licenseText: string;
  };
}

// Helper type for nested key access
export type TranslationKey = string;
