<script lang="ts">
  import { t } from '$lib/i18n';
  import { settings, canAnalyze, canExport, isAnalyzing, progress } from '../stores/app';
  import { runAnalysis, exportReport } from '../stores/tauri';

  async function handleAnalyze() {
    try {
      await runAnalysis();
    } catch (error) {
      console.error('Analysis failed:', error);
    }
  }

  async function handleExport() {
    try {
      await exportReport();
    } catch (error) {
      console.error('Export failed:', error);
    }
  }
</script>

<div class="cyber-panel p-4">
  <h2 class="text-lg font-semibold text-neon-green mb-4">{$t('controlPanel.title')}</h2>

  <!-- Options Grid -->
  <div class="grid grid-cols-2 gap-4">
    <label class="flex items-center gap-2 cursor-pointer">
      <input
        type="checkbox"
        class="w-4 h-4 accent-neon-green"
        bind:checked={$settings.hideSensitive}
        disabled={$isAnalyzing}
      />
      <span class="text-gray-300">{$t('controlPanel.hideSensitive')}</span>
    </label>

    <label class="flex items-center gap-2 cursor-pointer">
      <input
        type="checkbox"
        class="w-4 h-4 accent-neon-green"
        bind:checked={$settings.splitIncomeExpense}
        disabled={$isAnalyzing}
      />
      <span class="text-gray-300">{$t('controlPanel.splitIncomeExpense')}</span>
    </label>

    <label class="flex items-center gap-2 cursor-pointer">
      <input
        type="checkbox"
        class="w-4 h-4 accent-neon-green"
        bind:checked={$settings.ipCrossReference}
        disabled={$isAnalyzing}
      />
      <span class="text-gray-300">{$t('controlPanel.ipCrossReference')}</span>
    </label>

    <label class="flex items-center gap-2 cursor-pointer">
      <input
        type="checkbox"
        class="w-4 h-4 accent-neon-pink"
        bind:checked={$settings.whoisLookup}
        disabled={$isAnalyzing}
      />
      <span class="text-neon-pink">{$t('controlPanel.whoisLookup')}</span>
    </label>
  </div>

  <!-- Progress Bar -->
  {#if $isAnalyzing && $progress}
    <div class="mt-4">
      <div class="flex justify-between text-xs text-gray-400 mb-1">
        <span>{$progress.stage}</span>
        <span>{$progress.progress}%</span>
      </div>
      <div class="h-2 bg-cyber-card rounded-full overflow-hidden">
        <div
          class="h-full bg-neon-green transition-all duration-300"
          style="width: {$progress.progress}%; box-shadow: 0 0 10px #00ff9d;"
        ></div>
      </div>
      <div class="text-xs text-gray-500 mt-1">{$progress.message}</div>
    </div>
  {/if}

  <!-- Action Buttons -->
  <div class="mt-6 flex gap-4">
    <button
      class="btn-primary flex-1"
      disabled={!$canAnalyze}
      on:click={handleAnalyze}
    >
      {#if $isAnalyzing}
        <span class="animate-pulse">{$t('controlPanel.analyzing')}</span>
      {:else}
        {$t('controlPanel.executeAnalysis')}
      {/if}
    </button>

    <button
      class="btn-danger"
      disabled={!$canExport}
      on:click={handleExport}
    >
      {$t('controlPanel.exportReport')}
    </button>
  </div>
</div>
