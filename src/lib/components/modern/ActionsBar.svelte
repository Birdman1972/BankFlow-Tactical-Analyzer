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
  $: surfaceClass = isDark ? 'border-slate-800 bg-slate-900 shadow-sm' : 'border-modern-border bg-white shadow-sm';
  $: subClass = isDark ? 'text-slate-400' : 'text-slate-500';
  $: secondaryButtonClass = isDark
    ? 'inline-flex items-center justify-center rounded-xl border border-slate-700 bg-slate-800 px-5 py-2.5 text-sm font-bold text-slate-100 transition-all hover:bg-slate-700 hover:translate-y-[-1px] active:translate-y-0 disabled:opacity-50 disabled:hover:bg-slate-800 disabled:hover:translate-y-0 shadow-sm'
    : 'inline-flex items-center justify-center rounded-xl border border-modern-border bg-white px-5 py-2.5 text-sm font-bold text-slate-900 transition-all hover:bg-slate-50 hover:translate-y-[-1px] active:translate-y-0 disabled:opacity-50 disabled:hover:bg-white disabled:hover:translate-y-0 shadow-sm';
</script>

<section class={['rounded-2xl border p-6', surfaceClass].join(' ')}>
  <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
    <div class="min-w-0">
      <h2 class={['text-sm font-bold uppercase tracking-wider', subClass].join(' ')}>Control Interface</h2>
      <div class={['text-xs mt-1 font-medium', subClass].join(' ')}>Execute system operations and process data</div>
    </div>

    <div class="flex flex-wrap gap-3">
      <button
        type="button"
        class="inline-flex items-center justify-center rounded-xl bg-modern-primary px-6 py-2.5 text-sm font-bold text-white transition-all hover:bg-modern-primary/90 hover:translate-y-[-1px] active:translate-y-0 disabled:opacity-50 disabled:hover:bg-modern-primary disabled:hover:translate-y-0 shadow-md shadow-modern-primary/20"
        disabled={!$canAnalyze}
        on:click={handleAnalyze}
      >
        {#if $isAnalyzing}
          <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
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
        <svg class="w-4 h-4 mr-2 text-modern-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
        {$t('controlPanel.exportReport')}
      </button>

      {#if showClear}
        <button
          type="button"
          class={isDark
            ? 'inline-flex items-center justify-center rounded-xl border border-rose-900/40 bg-rose-500/10 px-5 py-2.5 text-sm font-bold text-rose-200 transition-all hover:bg-rose-500/15 hover:translate-y-[-1px] active:translate-y-0 disabled:opacity-50 disabled:hover:translate-y-0'
            : 'inline-flex items-center justify-center rounded-xl border border-rose-200 bg-rose-50 px-5 py-2.5 text-sm font-bold text-rose-700 transition-all hover:bg-rose-100 hover:translate-y-[-1px] active:translate-y-0 disabled:opacity-50 disabled:hover:translate-y-0 shadow-sm'}
          disabled={$isAnalyzing}
          on:click={handleClear}
        >
          {$t('app.clearAll')}
        </button>
      {/if}
    </div>
  </div>

  {#if $isAnalyzing && $progress}
    <div class="mt-6 pt-6 border-t border-slate-100 dark:border-slate-800">
      <div class={['flex items-center justify-between text-[11px] font-bold uppercase tracking-wider', isDark ? 'text-slate-400' : 'text-slate-600'].join(' ')}>
        <span class="truncate">STAGE: {$progress.stage} — {$progress.message}</span>
        <span class="tabular-nums">{$progress.progress}%</span>
      </div>
      <div class={['mt-3 h-2.5 w-full overflow-hidden rounded-full shadow-inner', isDark ? 'bg-slate-800' : 'bg-slate-100'].join(' ')}>
        <div
          class="h-full bg-modern-primary transition-all duration-500 ease-out shadow-lg shadow-modern-primary/20"
          style="width: {$progress.progress}%;"
        ></div>
      </div>
    </div>
  {/if}
</section>
