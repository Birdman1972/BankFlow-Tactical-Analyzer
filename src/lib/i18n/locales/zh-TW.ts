/**
 * Traditional Chinese (Taiwan) Translations
 * TODO: Will be replaced by Gemini CLI output
 */

import type { Translations } from '../types';

const zhTW: Translations = {
  app: {
    title: 'BankFlow 金流分析器',
    subtitle: '數位鑑識工具',
    clearAll: '全部清除',
    footer: '以 Tauri + Svelte + Rust 打造 | 離線優先設計',
  },
  modern: {
    title: 'Modern 介面',
    backToClassic: '回到 Classic',
    themeLight: '亮色',
    themeDark: '暗色',
    uiModeClassic: 'Classic',
    uiModeModern: 'Modern',
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
    appVersion: 'BankFlow 金流分析器 v{version}',
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
  downloadsDialog: {
    open: '下載',
    title: '下載桌面版',
    latestVersion: '最新版本',
    loading: '載入版本資訊中...',
    loadError: '無法載入版本資訊（version.json）',
    openReleases: '開啟 GitHub Releases',
    downloadWindowsMsi: '下載 Windows MSI',
    downloadPortableZip: '下載可攜版 ZIP',
    macosNote: 'macOS：請至 GitHub Releases 下載。',
    releaseNotes: '更新內容',
    usageTitle: '使用說明',
    usageStep1: '準備資料：存款明細 (檔案 A) 與 IP 紀錄 (檔案 B) 的 Excel 檔。',
    usageStep2: '拖曳上傳：將檔案分別拖入對應區域。',
    usageStep3: '執行分析：調整右側設定後點擊「執行分析」。',
    usageStep4: '下載報告：分析完成後點擊「匯出報告」下載結果。',
    featureTitle: '系統特色',
    feature1: '離線優先：所有運算在本機完成，確保資料操作安全。',
    feature2: '高性能：Rust 核心引擎，流暢處理大量交易紀錄。',
    feature3: 'IP 匹配：自動比對交易時間與 IP 登入紀錄（±2秒）。',
    close: '關閉',
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
    attachmentsLabel: '附件連結',
    attachmentsPlaceholder: 'https://...（每行一個連結）',
    attachmentsNote: '僅支援連結，不支援檔案上傳。',
    versionLabel: '版本',
    submit: '送出回饋',
    submitting: '送出中...',
    success: '感謝！你的回饋已送出。',
    queued: '感謝！已暫存回饋，待連線後自動送出。',
    error: '送出失敗，稍後會重試。',
    privacyNote: '請避免提供個人或敏感資訊。',
    errors: {
      typeRequired: '請選擇回饋類型。',
      titleRequired: '請填寫標題。',
      descriptionRequired: '請填寫詳細內容。',
      attachmentsInvalid: '僅允許 http/https 連結。',
    },
  },
  nav: {
    home: '首頁',
    about: '關於',
    feedback: '意見回饋',
    collapse: '收合側邊欄',
    expand: '展開側邊欄',
  },
  shortcuts: {
    title: '快捷鍵說明',
    close: '關閉',
    openFileA: '開啟檔案 A',
    openFileB: '開啟檔案 B',
    runAnalysis: '執行分析',
    exportReport: '匯出報告',
    clearAll: '清除所有檔案',
    closeDialog: '關閉對話框',
    showHelp: '顯示快捷鍵說明',
    note: '輸入文字時快捷鍵將停用（Esc 除外）。',
  },
  about: {
    title: '關於 BankFlow 金流分析器',
    subtitle: '離線數位鑑識分析工具',
    currentVersion: '目前版本',
    versionInfo: '版本資訊',
    releaseDate: '發佈日期',
    changelog: '更新歷史',
    loading: '載入中...',
    loadError: '無法載入版本歷史。',
    changelogEmpty: '目前沒有更新紀錄。',
    developer: '開發者',
    license: '授權條款',
    licenseText: `MIT 授權條款

Copyright (c) 2026 Antigravity AICoder

特此授權，任何取得本軟體與相關文件（以下稱「軟體」）之人，得以免費取得且不受限制地使用本軟體，包括但不限於使用、複製、修改、合併、出版、散布、再授權及／或販售本軟體之副本，並允許提供本軟體之人行使上述權利，惟須符合以下條件：

上述著作權聲明與本授權聲明須包含於所有副本或軟體重要部分之中。

本軟體係依「現狀」提供，不作任何明示或默示之保證，包括但不限於適售性、特定用途適用性及不侵權之保證。作者或著作權人不對因本軟體或本軟體之使用或其他交易行為所生之任何索賠、損害或其他責任負責，無論係因契約、侵權或其他原因所致。`,
  },
};

export default zhTW;
