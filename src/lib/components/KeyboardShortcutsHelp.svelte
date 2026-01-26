<script lang="ts">
  import { t } from '$lib/i18n';

  interface Props {
    open: boolean;
    onClose: () => void;
  }

  let { open, onClose }: Props = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <button
      type="button"
      class="absolute inset-0 bg-black/70"
      aria-label={$t('shortcuts.close')}
      onclick={onClose}
    ></button>
    <div class="relative w-full max-w-xl cyber-panel p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold text-neon-green">{$t('shortcuts.title')}</h2>
        <button
          class="text-gray-400 hover:text-white transition-colors"
          onclick={onClose}
          aria-label={$t('shortcuts.close')}
        >
          <svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M18 6L6 18" />
            <path d="M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="space-y-3 text-sm">
        <div class="flex items-center justify-between border-b border-cyber-border pb-2">
          <span class="text-gray-300">{$t('shortcuts.openFileA')}</span>
          <span class="text-neon-blue">⌘/Ctrl + O</span>
        </div>
        <div class="flex items-center justify-between border-b border-cyber-border pb-2">
          <span class="text-gray-300">{$t('shortcuts.openFileB')}</span>
          <span class="text-neon-blue">⌘/Ctrl + Shift + O</span>
        </div>
        <div class="flex items-center justify-between border-b border-cyber-border pb-2">
          <span class="text-gray-300">{$t('shortcuts.runAnalysis')}</span>
          <span class="text-neon-blue">⌘/Ctrl + Enter</span>
        </div>
        <div class="flex items-center justify-between border-b border-cyber-border pb-2">
          <span class="text-gray-300">{$t('shortcuts.exportReport')}</span>
          <span class="text-neon-blue">⌘/Ctrl + E</span>
        </div>
        <div class="flex items-center justify-between border-b border-cyber-border pb-2">
          <span class="text-gray-300">{$t('shortcuts.clearAll')}</span>
          <span class="text-neon-blue">⌘/Ctrl + K</span>
        </div>
        <div class="flex items-center justify-between border-b border-cyber-border pb-2">
          <span class="text-gray-300">{$t('shortcuts.closeDialog')}</span>
          <span class="text-neon-blue">Esc</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-gray-300">{$t('shortcuts.showHelp')}</span>
          <span class="text-neon-blue">?</span>
        </div>
      </div>

      <div class="mt-4 text-xs text-gray-500">
        {$t('shortcuts.note')}
      </div>
    </div>
  </div>
{/if}
