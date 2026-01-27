/**
 * Toast Notification Store
 * Uses Svelte writable store for state management
 */

import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration: number;
}

// Global state using Svelte store
const toasts = writable<Toast[]>([]);

export const toastStore = {
  // Subscribe to the store (for use in components)
  subscribe: toasts.subscribe,

  /**
   * Show a notification
   */
  show(type: ToastType, message: string, duration: number = 5000) {
    const id = Math.random().toString(36).substring(2, 9);
    const newToast: Toast = { id, type, message, duration };

    toasts.update(current => {
      // Limit to 5 toasts
      if (current.length >= 5) {
        return [...current.slice(1), newToast];
      }
      return [...current, newToast];
    });

    if (duration > 0) {
      setTimeout(() => {
        this.dismiss(id);
      }, duration);
    }

    return id;
  },

  // Helper methods
  success(message: string, duration?: number) {
    return this.show('success', message, duration);
  },

  error(message: string, duration?: number) {
    return this.show('error', message, duration);
  },

  warning(message: string, duration?: number) {
    return this.show('warning', message, duration);
  },

  info(message: string, duration?: number) {
    return this.show('info', message, duration);
  },

  /**
   * Dismiss a specific toast
   */
  dismiss(id: string) {
    toasts.update(current => current.filter(t => t.id !== id));
  },

  /**
   * Dismiss all toasts
   */
  dismissAll() {
    toasts.set([]);
  }
};
