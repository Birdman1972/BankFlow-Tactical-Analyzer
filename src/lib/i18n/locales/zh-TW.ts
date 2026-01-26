/**
 * Traditional Chinese (Taiwan) Translations
 * TODO: Will be replaced by Gemini CLI output
 */

import type { Translations } from '../types';

const zhTW: Translations = {
  app: {
    title: 'BankFlow 戰術分析器',
    subtitle: '數位鑑識工具',
    clearAll: '全部清除',
    footer: '以 Tauri + Svelte + Rust 打造 | 離線優先設計',
  },
  dropZone: {
    transactionFile: '拖放交易檔案',
    ipLogFile: '拖放 IP 日誌檔案',
    fileFormat: '(.xlsx)',
    rows: '筆資料',
  },
  controlPanel: {
    title: '控制面板',
    hideSensitive: '隱藏敏感欄位',
    splitIncomeExpense: '分離收支',
    ipCrossReference: 'IP 交叉比對',
    whoisLookup: 'Whois 查詢（操作安全風險）',
    analyzing: '分析中...',
    executeAnalysis: '執行分析',
    exportReport: '匯出報告',
  },
  resultSummary: {
    title: '分析結果',
    totalRecords: '總筆數',
    ipMatched: 'IP 比對成功',
    multiIp: '多重 IP',
    requiresReview: '需審查',
    whoisQueries: 'Whois 查詢數',
    settingsUsed: '使用設定：',
    sensitiveHidden: '隱藏敏感資料',
    incomeExpenseSplit: '收支分離',
    ipCrossRef: 'IP 交叉比對',
    whoisEnabled: 'Whois 啟用',
    yes: '是',
    no: '否',
  },
  warningBanner: {
    title: '操作安全警告',
    whoisWarning: 'Whois 查詢已啟用。這將發送可能被記錄的網路請求。',
    generalWarning: '處理敏感財務資料時請謹慎小心。',
  },
  logConsole: {
    title: '日誌控制台',
    clear: '清除',
  },
  system: {
    appVersion: 'BankFlow 戰術分析器 v{version}',
    awaitingInput: '等待檔案輸入...',
    stateReset: '狀態已重設',
    logCleared: '日誌已清除',
  },
  updateDialog: {
    title: '有可用的更新',
    newVersion: '新版本',
    releaseNotes: '更新內容：',
    updateNow: '立即更新',
    remindLater: '稍後提醒',
    skipVersion: '略過此版本',
  },
  feedbackForm: {
    title: '提交意見回饋',
    typeLabel: '回饋類型',
    typeFeature: '功能建議',
    typeBug: 'Bug 回報',
    typeUx: '使用體驗',
    titleLabel: '標題',
    titlePlaceholder: '請簡要描述',
    descriptionLabel: '詳細內容',
    descriptionPlaceholder: '請描述問題或建議…',
    versionLabel: '版本',
    submit: '送出回饋',
    submitting: '送出中...',
    success: '感謝！你的回饋已送出。',
    error: '送出失敗，稍後會重試。',
    privacyNote: '請避免提供個人或敏感資訊。',
    errors: {
      typeRequired: '請選擇回饋類型。',
      titleRequired: '請填寫標題。',
      descriptionRequired: '請填寫詳細內容。',
    },
  },
};

export default zhTW;
