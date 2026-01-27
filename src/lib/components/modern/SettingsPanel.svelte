<script lang="ts">
  import { t } from '$lib/i18n';
  import { settings, isAnalyzing } from '$lib/stores/app';
  import { theme } from '$lib/stores/theme';

  $: isDark = $theme === 'dark';
  $: surfaceClass = isDark ? 'border-slate-800 bg-slate-950/30' : 'border-slate-200 bg-white';
  $: titleClass = isDark ? 'text-slate-100' : 'text-slate-900';
  $: subClass = isDark ? 'text-slate-400' : 'text-slate-500';
  $: tileClass = isDark ? 'border-slate-800 text-slate-100' : 'border-slate-200 text-slate-900';
</script>

<section class={['rounded-2xl border p-4', surfaceClass].join(' ')}>
  <div class="flex items-center justify-between gap-4">
    <h2 class={['text-sm font-semibold', titleClass].join(' ')}>{$t('controlPanel.title')}</h2>
    <span class={['text-[11px]', subClass].join(' ')}>{$isAnalyzing ? $t('controlPanel.analyzing') : ''}</span>
  </div>

  <div class="mt-4 grid grid-cols-1 gap-3 md:grid-cols-2">
    <label class={['flex items-start gap-3 rounded-xl border p-3 cursor-pointer', tileClass].join(' ')}>
      <input
        type="checkbox"
        class="mt-0.5 h-4 w-4 accent-sky-600"
        bind:checked={$settings.hideSensitive}
        disabled={$isAnalyzing}
      />
      <div>
        <div class={['text-sm font-medium', titleClass].join(' ')}>{$t('controlPanel.hideSensitive')}</div>
        <div class={['text-xs', subClass].join(' ')}>Redact sensitive columns in exported output</div>
      </div>
    </label>

    <label class={['flex items-start gap-3 rounded-xl border p-3 cursor-pointer', tileClass].join(' ')}>
      <input
        type="checkbox"
        class="mt-0.5 h-4 w-4 accent-sky-600"
        bind:checked={$settings.splitIncomeExpense}
        disabled={$isAnalyzing}
      />
      <div>
        <div class={['text-sm font-medium', titleClass].join(' ')}>{$t('controlPanel.splitIncomeExpense')}</div>
        <div class={['text-xs', subClass].join(' ')}>Separate income/expense sheets in report</div>
      </div>
    </label>

    <label class={['flex items-start gap-3 rounded-xl border p-3 cursor-pointer', tileClass].join(' ')}>
      <input
        type="checkbox"
        class="mt-0.5 h-4 w-4 accent-sky-600"
        bind:checked={$settings.ipCrossReference}
        disabled={$isAnalyzing}
      />
      <div>
        <div class={['text-sm font-medium', titleClass].join(' ')}>{$t('controlPanel.ipCrossReference')}</div>
        <div class={['text-xs', subClass].join(' ')}>Match transactions to IP logs</div>
      </div>
    </label>

    <label class={['flex items-start gap-3 rounded-xl border p-3 cursor-pointer', isDark ? 'border-rose-900/40 bg-rose-500/10' : 'border-rose-200 bg-rose-50/40'].join(' ')}>
      <input
        type="checkbox"
        class="mt-0.5 h-4 w-4 accent-rose-600"
        bind:checked={$settings.whoisLookup}
        disabled={$isAnalyzing}
      />
      <div>
        <div class={['text-sm font-medium', isDark ? 'text-rose-200' : 'text-rose-900'].join(' ')}>{$t('controlPanel.whoisLookup')}</div>
        <div class={['text-xs', isDark ? 'text-rose-200/80' : 'text-rose-700'].join(' ')}>May trigger network requests and leave traces (OpSec)</div>
      </div>
    </label>
  </div>
</section>
