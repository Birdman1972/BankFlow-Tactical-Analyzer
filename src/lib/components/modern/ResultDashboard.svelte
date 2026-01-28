<script lang="ts">
  import { t } from '$lib/i18n';
  import { analysisResult } from '$lib/stores/app';
  import { theme } from '$lib/stores/theme';

  $: isDark = $theme === 'dark';
  $: surfaceClass = isDark ? 'border-slate-800 bg-slate-900 shadow-sm' : 'border-modern-border bg-white shadow-sm';
  $: titleClass = isDark ? 'text-slate-100' : 'text-slate-900';
  $: subClass = isDark ? 'text-slate-400' : 'text-slate-500';
</script>

{#if $analysisResult}
  <section class={['rounded-2xl border p-6', surfaceClass].join(' ')}>
    <div class="flex items-center justify-between gap-4 mb-6">
      <h2 class={['text-sm font-bold uppercase tracking-wider', subClass].join(' ')}>{$t('resultSummary.title')}</h2>
      <div class="flex items-center gap-1.5 px-2 py-0.5 rounded-md bg-modern-primary/10 text-modern-primary text-[10px] font-bold uppercase">
        <span class="relative flex h-1.5 w-1.5">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-modern-primary opacity-75"></span>
          <span class="relative inline-flex rounded-full h-1.5 w-1.5 bg-modern-primary"></span>
        </span>
        LIVE ANALYTICS
      </div>
    </div>

    <div class="grid grid-cols-2 gap-4 md:grid-cols-4">
      <div class={['rounded-xl border p-4 transition-all hover:shadow-md', isDark ? 'border-slate-800 bg-slate-950/40' : 'border-modern-border/60 bg-modern-bg/50'].join(' ')}>
        <div class={['text-[10px] font-bold uppercase tracking-tight opacity-70', subClass].join(' ')}>{$t('resultSummary.totalRecords')}</div>
        <div class={['mt-2 text-2xl font-bold tabular-nums tracking-tighter', titleClass].join(' ')}>{$analysisResult.totalRecords.toLocaleString()}</div>
      </div>
      
      <div class={['rounded-xl border p-4 transition-all hover:shadow-md', isDark ? 'border-emerald-900/40 bg-emerald-500/5' : 'border-emerald-100 bg-emerald-50/50'].join(' ')}>
        <div class={['text-[10px] font-bold uppercase tracking-tight', isDark ? 'text-emerald-400' : 'text-emerald-600'].join(' ')}>{$t('resultSummary.ipMatched')}</div>
        <div class={['mt-2 text-2xl font-bold tabular-nums tracking-tighter', isDark ? 'text-emerald-300' : 'text-emerald-700'].join(' ')}>{$analysisResult.matchedCount.toLocaleString()}</div>
      </div>

      <div class={['rounded-xl border p-4 transition-all hover:shadow-md', isDark ? 'border-amber-900/40 bg-amber-500/5' : 'border-amber-100 bg-amber-50/50'].join(' ')}>
        <div class={['text-[10px] font-bold uppercase tracking-tight', isDark ? 'text-amber-400' : 'text-amber-600'].join(' ')}>{$t('resultSummary.multiIp')}</div>
        <div class={['mt-2 text-2xl font-bold tabular-nums tracking-tighter', isDark ? 'text-amber-300' : 'text-amber-700'].join(' ')}>{$analysisResult.multiIpCount.toLocaleString()}</div>
      </div>

      <div class={['rounded-xl border p-4 transition-all hover:shadow-md', isDark ? 'border-slate-800 bg-slate-950/40' : 'border-modern-border/60 bg-modern-bg/50'].join(' ')}>
        <div class={['text-[10px] font-bold uppercase tracking-tight opacity-70', subClass].join(' ')}>{$t('resultSummary.whoisQueries')}</div>
        <div class={['mt-2 text-2xl font-bold tabular-nums tracking-tighter', titleClass].join(' ')}>{$analysisResult.whoisQueried.toLocaleString()}</div>
      </div>
    </div>

    <div class="mt-6 pt-6 border-t border-slate-100 dark:border-slate-800">
      <div class={['text-[10px] font-bold uppercase tracking-widest mb-3', subClass].join(' ')}>{$t('resultSummary.settingsUsed')}</div>
      <div class="flex flex-wrap gap-2">
        {#if $analysisResult.settings.hideSensitive}
          <span class="modern-badge modern-badge-primary">{$t('resultSummary.sensitiveHidden')}</span>
        {/if}
        {#if $analysisResult.settings.splitIncomeExpense}
          <span class="modern-badge modern-badge-primary">{$t('resultSummary.incomeExpenseSplit')}</span>
        {/if}
        {#if $analysisResult.settings.ipCrossReference}
          <span class="modern-badge modern-badge-primary">{$t('resultSummary.ipCrossRef')}</span>
        {/if}
        {#if $analysisResult.settings.whoisLookup}
          <span class="modern-badge modern-badge-danger shadow-sm shadow-rose-500/10">{$t('resultSummary.whoisEnabled')}</span>
        {/if}
      </div>
    </div>
  </section>
{/if}
