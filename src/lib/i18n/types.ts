/**
 * i18n Type Definitions
 *
 * Defines the structure for all translatable strings in BankFlow Tactical Analyzer
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
    error: string;
    privacyNote: string;
    errors: {
      typeRequired: string;
      titleRequired: string;
      descriptionRequired: string;
    };
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
