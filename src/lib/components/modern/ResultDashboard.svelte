<script lang="ts">
  import { t } from '$lib/i18n';
  import { analysisResult } from '$lib/stores/app';
  import { theme } from '$lib/stores/theme';

  $: isDark = $theme === 'dark';
  $: surfaceClass = isDark ? 'border-slate-800 bg-slate-950/30' : 'border-slate-200 bg-white';
  $: titleClass = isDark ? 'text-slate-100' : 'text-slate-900';
</script>

{#if $analysisResult}
  <section class={['rounded-2xl border p-4', surfaceClass].join(' ')}>
    <div class="flex items-center justify-between gap-4">
      <h2 class={['text-sm font-semibold', titleClass].join(' ')}>{$t('resultSummary.title')}</h2>
      <span class={['text-[11px]', isDark ? 'text-slate-400' : 'text-slate-500'].join(' ')}>v2</span>
    </div>

    <div class="mt-4 grid grid-cols-2 gap-3 md:grid-cols-4">
      <div class={['rounded-xl border p-3', isDark ? 'border-slate-800 bg-slate-900/40' : 'border-slate-200 bg-slate-50'].join(' ')}>
        <div class={['text-[11px]', isDark ? 'text-slate-400' : 'text-slate-500'].join(' ')}>{$t('resultSummary.totalRecords')}</div>
        <div class={['mt-1 text-xl font-semibold tabular-nums', titleClass].join(' ')}>{$analysisResult.totalRecords.toLocaleString()}</div>
      </div>
      <div class={['rounded-xl border p-3', isDark ? 'border-emerald-900/40 bg-emerald-500/10' : 'border-emerald-200 bg-emerald-50/40'].join(' ')}>
        <div class={['text-[11px]', isDark ? 'text-emerald-200' : 'text-emerald-700'].join(' ')}>{$t('resultSummary.ipMatched')}</div>
        <div class={['mt-1 text-xl font-semibold tabular-nums', isDark ? 'text-emerald-100' : 'text-emerald-900'].join(' ')}>{$analysisResult.matchedCount.toLocaleString()}</div>
      </div>
      <div class={['rounded-xl border p-3', isDark ? 'border-amber-900/40 bg-amber-500/10' : 'border-amber-200 bg-amber-50/40'].join(' ')}>
        <div class={['text-[11px]', isDark ? 'text-amber-200' : 'text-amber-700'].join(' ')}>{$t('resultSummary.multiIp')}</div>
        <div class={['mt-1 text-xl font-semibold tabular-nums', isDark ? 'text-amber-100' : 'text-amber-900'].join(' ')}>{$analysisResult.multiIpCount.toLocaleString()}</div>
      </div>
      <div class={['rounded-xl border p-3', isDark ? 'border-slate-800 bg-slate-900/40' : 'border-slate-200 bg-slate-50'].join(' ')}>
        <div class={['text-[11px]', isDark ? 'text-slate-400' : 'text-slate-500'].join(' ')}>{$t('resultSummary.whoisQueries')}</div>
        <div class={['mt-1 text-xl font-semibold tabular-nums', titleClass].join(' ')}>{$analysisResult.whoisQueried.toLocaleString()}</div>
      </div>
    </div>

    <div class={['mt-4 rounded-xl border p-3', isDark ? 'border-slate-800 bg-slate-900/40' : 'border-slate-200 bg-white'].join(' ')}>
      <div class={['text-[11px]', isDark ? 'text-slate-400' : 'text-slate-500'].join(' ')}>{$t('resultSummary.settingsUsed')}</div>
      <div class="mt-2 flex flex-wrap gap-2">
        {#if $analysisResult.settings.hideSensitive}
          <span class={['rounded-full px-2 py-1 text-[11px]', isDark ? 'bg-slate-800 text-slate-200' : 'bg-slate-100 text-slate-700'].join(' ')}>{$t('resultSummary.sensitiveHidden')}</span>
        {/if}
        {#if $analysisResult.settings.splitIncomeExpense}
          <span class={['rounded-full px-2 py-1 text-[11px]', isDark ? 'bg-slate-800 text-slate-200' : 'bg-slate-100 text-slate-700'].join(' ')}>{$t('resultSummary.incomeExpenseSplit')}</span>
        {/if}
        {#if $analysisResult.settings.ipCrossReference}
          <span class={['rounded-full px-2 py-1 text-[11px]', isDark ? 'bg-slate-800 text-slate-200' : 'bg-slate-100 text-slate-700'].join(' ')}>{$t('resultSummary.ipCrossRef')}</span>
        {/if}
        {#if $analysisResult.settings.whoisLookup}
          <span class={['rounded-full px-2 py-1 text-[11px]', isDark ? 'bg-rose-500/10 text-rose-200' : 'bg-rose-50 text-rose-700'].join(' ')}>{$t('resultSummary.whoisEnabled')}</span>
        {/if}
      </div>
    </div>
  </section>
{/if}
