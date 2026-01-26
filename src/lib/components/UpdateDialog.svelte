<script lang="ts">
  import { t } from '$lib/i18n';
  import { fade, scale } from 'svelte/transition';

  interface Props {
    version: string;
    changelog: string[];
    onUpdate: () => void;
    onRemindLater: () => void;
    onSkip: () => void;
  }

  let { 
    version, 
    changelog, 
    onUpdate, 
    onRemindLater, 
    onSkip 
  }: Props = $props();

</script>

<!-- Backdrop -->
<div 
  class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4"
  transition:fade={{ duration: 200 }}
>
  <!-- Modal -->
  <div 
    class="cyber-panel w-full max-w-md p-6 flex flex-col gap-6 shadow-2xl border-neon-green/30"
    transition:scale={{ duration: 300, start: 0.95 }}
  >
    <!-- Header -->
    <div class="flex items-center gap-4">
      <div class="w-12 h-12 flex items-center justify-center rounded-lg bg-neon-green/20 text-neon-green border border-neon-green/50">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
      </div>
      <div>
        <h2 class="text-xl font-bold neon-text-green">{$t('updateDialog.title')}</h2>
        <div class="flex items-center gap-2 mt-1">
          <span class="text-xs text-gray-400">{$t('updateDialog.newVersion')}:</span>
          <span class="text-xs font-mono bg-neon-green/10 text-neon-green px-2 py-0.5 rounded border border-neon-green/20">
            v{version}
          </span>
        </div>
      </div>
    </div>

    <!-- Body -->
    <div class="space-y-3">
      <h3 class="text-sm font-semibold text-gray-300">{$t('updateDialog.releaseNotes')}</h3>
      <div class="max-h-40 overflow-y-auto pr-2 custom-scrollbar bg-black/40 rounded p-3 border border-gray-800">
        <ul class="space-y-2">
          {#each changelog as item}
            <li class="text-sm text-gray-400 flex gap-2">
              <span class="text-neon-green">•</span>
              <span>{item}</span>
            </li>
          {/each}
        </ul>
      </div>
    </div>

    <!-- Footer / Actions -->
    <div class="flex flex-col gap-2 pt-2">
      <button 
        class="w-full py-2.5 bg-neon-green hover:bg-neon-green/80 text-black font-bold rounded transition-all transform active:scale-[0.98]"
        onclick={onUpdate}
      >
        {$t('updateDialog.updateNow')}
      </button>
      
      <div class="grid grid-cols-2 gap-2">
        <button 
          class="py-2 text-sm text-gray-400 hover:text-white hover:bg-white/5 rounded border border-gray-800 transition-colors"
          onclick={onRemindLater}
        >
          {$t('updateDialog.remindLater')}
        </button>
        <button 
          class="py-2 text-sm text-gray-400 hover:text-neon-pink hover:bg-neon-pink/5 rounded border border-gray-800 hover:border-neon-pink/30 transition-colors"
          onclick={onSkip}
        >
          {$t('updateDialog.skipVersion')}
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: #333;
    border-radius: 2px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: #444;
  }

  .neon-text-green {
    color: #00ff9d;
    text-shadow: 0 0 10px rgba(0, 255, 157, 0.4);
  }
</style>
