<script lang="ts">
  import { get } from 'svelte/store';
  import { t } from '$lib/i18n';
  import { theme } from '$lib/stores/theme';
  import {
    settings,
    canAnalyze,
    canExport,
    isAnalyzing,
    progress,
    analysisResult,
    fileA,
    fileB,
    addLog,
  } from '$lib/stores/app';
  import { runAnalysis, exportReport, clearAllFiles } from '$lib/stores/platform';

  async function handleAnalyze() {
    try {
      isAnalyzing.set(true);
      progress.set({ stage: 'starting', progress: 0, message: 'Initializing analysis...' });

      const currentSettings = get(settings);
      const result = await runAnalysis(currentSettings, (p) => {
        progress.set(p);
      });

      analysisResult.set(result);
    } catch (error) {
      addLog('error', `Analysis failed: ${error}`);
      console.error('Analysis failed:', error);
    } finally {
      isAnalyzing.set(false);
      progress.set(null);
    }
  }

  async function handleExport() {
    try {
      await exportReport();
    } catch (error) {
      addLog('error', `Export failed: ${error}`);
      console.error('Export failed:', error);
    }
  }

  async function handleClear() {
    try {
      await clearAllFiles();
    } catch (error) {
      console.warn('clearAllFiles failed:', error);
    } finally {
      fileA.set(null);
      fileB.set(null);
      analysisResult.set(null);
    }
  }

  $: showClear = $fileA || $fileB || $analysisResult;
  $: isDark = $theme === 'dark';
  $: surfaceClass = isDark ? 'border-slate-800 bg-slate-950/30' : 'border-slate-200 bg-white';
  $: titleClass = isDark ? 'text-slate-100' : 'text-slate-900';
  $: subClass = isDark ? 'text-slate-400' : 'text-slate-500';
  $: secondaryButtonClass = isDark
    ? 'inline-flex items-center justify-center rounded-xl border border-slate-800 bg-slate-900 px-4 py-2 text-sm font-semibold text-slate-100 transition-colors hover:bg-slate-800 disabled:opacity-50 disabled:hover:bg-slate-900'
    : 'inline-flex items-center justify-center rounded-xl border border-slate-200 bg-white px-4 py-2 text-sm font-semibold text-slate-900 transition-colors hover:bg-slate-50 disabled:opacity-50 disabled:hover:bg-white';
</script>

<section class={['rounded-2xl border p-4', surfaceClass].join(' ')}>
  <div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
    <div class="min-w-0">
      <div class={['text-sm font-semibold', titleClass].join(' ')}>Actions</div>
      <div class={['text-xs', subClass].join(' ')}>Run analysis and export the report</div>
    </div>

    <div class="flex flex-wrap gap-2">
      <button
        type="button"
        class="inline-flex items-center justify-center rounded-xl bg-sky-600 px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-sky-500 disabled:opacity-50 disabled:hover:bg-sky-600"
        disabled={!$canAnalyze}
        on:click={handleAnalyze}
      >
        {#if $isAnalyzing}
          {$t('controlPanel.analyzing')}
        {:else}
          {$t('controlPanel.executeAnalysis')}
        {/if}
      </button>

      <button
        type="button"
        class={secondaryButtonClass}
        disabled={!$canExport}
        on:click={handleExport}
      >
        {$t('controlPanel.exportReport')}
      </button>

      {#if showClear}
        <button
          type="button"
          class={isDark
            ? 'inline-flex items-center justify-center rounded-xl border border-rose-900/40 bg-rose-500/10 px-4 py-2 text-sm font-semibold text-rose-200 transition-colors hover:bg-rose-500/15 disabled:opacity-50 disabled:hover:bg-rose-500/10'
            : 'inline-flex items-center justify-center rounded-xl border border-rose-200 bg-rose-50 px-4 py-2 text-sm font-semibold text-rose-700 transition-colors hover:bg-rose-100 disabled:opacity-50 disabled:hover:bg-rose-50'}
          disabled={$isAnalyzing}
          on:click={handleClear}
        >
          {$t('app.clearAll')}
        </button>
      {/if}
    </div>
  </div>

  {#if $isAnalyzing && $progress}
    <div class="mt-4">
      <div class={['flex items-center justify-between text-xs', isDark ? 'text-slate-400' : 'text-slate-600'].join(' ')}>
        <span class="truncate">{$progress.stage}: {$progress.message}</span>
        <span class="tabular-nums">{$progress.progress}%</span>
      </div>
      <div class={['mt-2 h-2 w-full overflow-hidden rounded-full', isDark ? 'bg-slate-800' : 'bg-slate-200'].join(' ')}>
        <div
          class="h-full bg-sky-600 transition-all duration-300"
          style="width: {$progress.progress}%;"
        ></div>
      </div>
    </div>
  {/if}
</section>
