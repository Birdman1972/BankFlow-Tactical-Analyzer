/**
 * Simple router state for sidebar navigation
 */

import { writable, get } from 'svelte/store';

export type PageId = 'home' | 'about' | 'feedback';

const COLLAPSE_KEY = 'bf.sidebar.collapsed';

function readCollapsed(): boolean {
  if (typeof localStorage === 'undefined') return false;
  return localStorage.getItem(COLLAPSE_KEY) === '1';
}

function writeCollapsed(value: boolean): void {
  if (typeof localStorage === 'undefined') return;
  localStorage.setItem(COLLAPSE_KEY, value ? '1' : '0');
}

export const currentPage = writable<PageId>('home');
export const isSidebarCollapsed = writable<boolean>(readCollapsed());
export const isSidebarOpen = writable<boolean>(false);

export function navigate(page: PageId): void {
  currentPage.set(page);
}

export function toggleCollapse(): void {
  isSidebarCollapsed.update((value) => {
    const next = !value;
    writeCollapsed(next);
    return next;
  });
}

export function openSidebar(): void {
  isSidebarOpen.set(true);
}

export function closeSidebar(): void {
  isSidebarOpen.set(false);
}

export function toggleSidebar(): void {
  isSidebarOpen.set(!get(isSidebarOpen));
}
