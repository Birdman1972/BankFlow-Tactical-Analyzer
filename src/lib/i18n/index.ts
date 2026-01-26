/**
 * i18n Core Module
 *
 * Provides internationalization support for BankFlow Tactical Analyzer
 * - Locale store with localStorage persistence
 * - Browser language detection
 * - Reactive translation function
 */

import { writable, derived, get } from 'svelte/store';
import type { Locale, Translations } from './types';
import en from './locales/en';
import zhTW from './locales/zh-TW';

// ============================================
// Locale Registry
// ============================================

const locales: Record<Locale, Translations> = {
  en,
  'zh-TW': zhTW,
};

// ============================================
// Locale Detection & Persistence
// ============================================

const STORAGE_KEY = 'bankflow-locale';

/**
 * Detect initial locale from localStorage or browser settings
 */
function detectLocale(): Locale {
  if (typeof window === 'undefined') return 'zh-TW';

  // Check localStorage first
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved && (saved === 'en' || saved === 'zh-TW')) {
    return saved;
  }

  // Fall back to browser language
  const browserLang = navigator.language;
  if (browserLang.startsWith('zh')) {
    return 'zh-TW';
  }

  return 'en';
}

// ============================================
// Stores
// ============================================

/**
 * Current locale store
 */
export const locale = writable<Locale>(detectLocale());

// Persist locale changes to localStorage
locale.subscribe((value) => {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_KEY, value);
    document.documentElement.lang = value;
  }
});

/**
 * Current translations object (derived from locale)
 */
export const translations = derived(locale, ($locale) => locales[$locale]);

// ============================================
// Translation Function
// ============================================

/**
 * Get a translation by dot-notation key
 * @param key - Dot-notation key (e.g., 'app.title', 'controlPanel.executeAnalysis')
 * @returns The translated string or the key if not found
 */
function getTranslation(trans: Translations, key: string): string {
  const keys = key.split('.');
  let value: unknown = trans;

  for (const k of keys) {
    if (value && typeof value === 'object' && k in value) {
      value = (value as Record<string, unknown>)[k];
    } else {
      return key; // Return key if path not found
    }
  }

  return typeof value === 'string' ? value : key;
}

/**
 * Reactive translation store
 * Use as: {$t('app.title')}
 */
export const t = derived(translations, ($trans) => {
  return (key: string): string => getTranslation($trans, key);
});

/**
 * Non-reactive translation function (for use outside Svelte components)
 * @param key - Dot-notation key
 * @returns The translated string
 */
export function translate(key: string): string {
  return getTranslation(get(translations), key);
}

// ============================================
// Locale Actions
// ============================================

/**
 * Set the current locale
 */
export function setLocale(newLocale: Locale): void {
  locale.set(newLocale);
}

/**
 * Toggle between available locales
 */
export function toggleLocale(): void {
  locale.update((current) => (current === 'en' ? 'zh-TW' : 'en'));
}

/**
 * Get all available locales
 */
export function getAvailableLocales(): Locale[] {
  return Object.keys(locales) as Locale[];
}

/**
 * Get locale display name
 */
export function getLocaleDisplayName(loc: Locale): string {
  const names: Record<Locale, string> = {
    en: 'English',
    'zh-TW': '繁體中文',
  };
  return names[loc];
}
