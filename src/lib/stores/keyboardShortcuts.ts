export interface ShortcutHandlerOptions {
  onOpenFileA: () => void;
  onOpenFileB: () => void;
  onRunAnalysis: () => void;
  onExportReport: () => void;
  onClearAll: () => void;
  onCloseDialog: () => void;
  onShowHelp: () => void;
}

function isEditableTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName.toLowerCase();
  if (tag === 'input' || tag === 'textarea' || tag === 'select') return true;
  return target.isContentEditable;
}

function isPrimaryModifier(event: KeyboardEvent): boolean {
  return event.metaKey || event.ctrlKey;
}

export function createKeyboardShortcuts(options: ShortcutHandlerOptions) {
  function handleKeydown(event: KeyboardEvent) {
    const key = event.key;
    const isShift = event.shiftKey;
    const primary = isPrimaryModifier(event);

    if (isEditableTarget(event.target) && key !== 'Escape') {
      return;
    }

    if (key === 'Escape') {
      options.onCloseDialog();
      return;
    }

    if (key === '?') {
      event.preventDefault();
      options.onShowHelp();
      return;
    }

    if (!primary) return;

    const lowerKey = key.toLowerCase();

    if (lowerKey === 'o' && isShift) {
      event.preventDefault();
      options.onOpenFileB();
      return;
    }

    if (lowerKey === 'o') {
      event.preventDefault();
      options.onOpenFileA();
      return;
    }

    if (key === 'Enter') {
      event.preventDefault();
      options.onRunAnalysis();
      return;
    }

    if (lowerKey === 'e') {
      event.preventDefault();
      options.onExportReport();
      return;
    }

    if (lowerKey === 'k') {
      event.preventDefault();
      options.onClearAll();
      return;
    }
  }

  function attach() {
    window.addEventListener('keydown', handleKeydown);
  }

  function detach() {
    window.removeEventListener('keydown', handleKeydown);
  }

  return { attach, detach };
}
