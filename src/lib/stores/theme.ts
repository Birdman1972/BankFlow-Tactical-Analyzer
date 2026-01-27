import { writable } from 'svelte/store';

export type Theme = 'light' | 'dark';

const STORAGE_KEY = 'bf.theme';

function isTheme(value: unknown): value is Theme {
  return value === 'light' || value === 'dark';
}

function detectInitialTheme(): Theme {
  if (typeof window === 'undefined' || typeof localStorage === 'undefined') {
    return 'light';
  }

  const saved = localStorage.getItem(STORAGE_KEY);
  if (isTheme(saved)) {
    return saved;
  }

  try {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    return prefersDark ? 'dark' : 'light';
  } catch {
    return 'light';
  }
}

export const theme = writable<Theme>(detectInitialTheme());

theme.subscribe((value) => {
  if (typeof window === 'undefined' || typeof localStorage === 'undefined') {
    return;
  }
  localStorage.setItem(STORAGE_KEY, value);
});

export function setTheme(next: Theme): void {
  theme.set(next);
}

export function toggleTheme(): void {
  theme.update((current) => (current === 'dark' ? 'light' : 'dark'));
}
