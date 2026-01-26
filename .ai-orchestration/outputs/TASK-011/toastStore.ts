/**
 * Toast Notification Store
 * Uses Svelte 5 Runes for state management
 */

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration: number;
}

// Global state using Svelte 5 runes
let toasts = $state<Toast[]>([]);

export const toastStore = {
  // Getter for the list of toasts
  get all() {
    return toasts;
  },

  /**
   * Show a notification
   */
  show(type: ToastType, message: string, duration: number = 5000) {
    const id = Math.random().toString(36).substring(2, 9);
    const newToast: Toast = { id, type, message, duration };
    
    // Limit to 5 toasts
    if (toasts.length >= 5) {
      toasts = [...toasts.slice(1), newToast];
    } else {
      toasts = [...toasts, newToast];
    }

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
    toasts = toasts.filter(t => t.id !== id);
  },

  /**
   * Dismiss all toasts
   */
  dismissAll() {
    toasts = [];
  }
};
