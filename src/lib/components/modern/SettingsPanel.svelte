<script lang="ts">
  import { t } from '$lib/i18n';
  import { settings, isAnalyzing } from '$lib/stores/app';
  import { theme } from '$lib/stores/theme';

  $: isDark = $theme === 'dark';
  $: surfaceClass = isDark ? 'border-slate-800 bg-slate-900 shadow-sm' : 'border-modern-border bg-white shadow-sm';
  $: titleClass = isDark ? 'text-slate-100' : 'text-slate-900';
  $: subClass = isDark ? 'text-slate-400' : 'text-slate-500';
  $: tileClass = isDark 
    ? 'border-slate-800 bg-slate-950/40 text-slate-100 hover:border-slate-700' 
    : 'border-modern-border/60 bg-modern-bg/50 text-slate-900 hover:border-modern-primary/30';
</script>

<section class={['rounded-2xl border p-6', surfaceClass].join(' ')}>
  <div class="flex items-center justify-between gap-4 mb-6">
    <h2 class={['text-sm font-bold uppercase tracking-wider', subClass].join(' ')}>{$t('controlPanel.title')}</h2>
    {#if $isAnalyzing}
      <div class="flex items-center gap-2 text-modern-primary animate-pulse">
        <div class="h-1.5 w-1.5 rounded-full bg-current"></div>
        <span class="text-[11px] font-bold uppercase tracking-tighter">{$t('controlPanel.analyzing')}</span>
      </div>
    {/if}
  </div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <label class={['flex items-start gap-4 rounded-xl border p-4 cursor-pointer transition-all duration-200 shadow-sm hover:shadow-md', tileClass].join(' ')}>
      <div class="relative flex items-center">
        <input
          type="checkbox"
          class="peer h-5 w-5 opacity-0 absolute cursor-pointer"
          bind:checked={$settings.hideSensitive}
          disabled={$isAnalyzing}
        />
        <div class="h-5 w-5 bg-white border-2 border-slate-300 rounded peer-checked:bg-modern-primary peer-checked:border-modern-primary transition-all flex items-center justify-center">
          <svg class="w-3.5 h-3.5 text-white opacity-0 peer-checked:opacity-100 transition-opacity" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
        </div>
      </div>
      <div>
        <div class={['text-sm font-bold', titleClass].join(' ')}>{$t('controlPanel.hideSensitive')}</div>
        <div class={['text-[11px] leading-tight mt-1 opacity-70 font-medium', subClass].join(' ')}>Redact sensitive columns in exported output</div>
      </div>
    </label>

    <label class={['flex items-start gap-4 rounded-xl border p-4 cursor-pointer transition-all duration-200 shadow-sm hover:shadow-md', tileClass].join(' ')}>
      <div class="relative flex items-center">
        <input
          type="checkbox"
          class="peer h-5 w-5 opacity-0 absolute cursor-pointer"
          bind:checked={$settings.splitIncomeExpense}
          disabled={$isAnalyzing}
        />
        <div class="h-5 w-5 bg-white border-2 border-slate-300 rounded peer-checked:bg-modern-primary peer-checked:border-modern-primary transition-all flex items-center justify-center">
          <svg class="w-3.5 h-3.5 text-white opacity-0 peer-checked:opacity-100 transition-opacity" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
        </div>
      </div>
      <div>
        <div class={['text-sm font-bold', titleClass].join(' ')}>{$t('controlPanel.splitIncomeExpense')}</div>
        <div class={['text-[11px] leading-tight mt-1 opacity-70 font-medium', subClass].join(' ')}>Separate income/expense sheets in report</div>
      </div>
    </label>

    <label class={['flex items-start gap-4 rounded-xl border p-4 cursor-pointer transition-all duration-200 shadow-sm hover:shadow-md', tileClass].join(' ')}>
      <div class="relative flex items-center">
        <input
          type="checkbox"
          class="peer h-5 w-5 opacity-0 absolute cursor-pointer"
          bind:checked={$settings.ipCrossReference}
          disabled={$isAnalyzing}
        />
        <div class="h-5 w-5 bg-white border-2 border-slate-300 rounded peer-checked:bg-modern-primary peer-checked:border-modern-primary transition-all flex items-center justify-center">
          <svg class="w-3.5 h-3.5 text-white opacity-0 peer-checked:opacity-100 transition-opacity" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
        </div>
      </div>
      <div>
        <div class={['text-sm font-bold', titleClass].join(' ')}>{$t('controlPanel.ipCrossReference')}</div>
        <div class={['text-[11px] leading-tight mt-1 opacity-70 font-medium', subClass].join(' ')}>Match transactions to IP logs</div>
      </div>
    </label>

    <label class={['flex items-start gap-4 rounded-xl border p-4 cursor-pointer transition-all duration-200 shadow-sm hover:shadow-md', isDark ? 'border-rose-900/40 bg-rose-500/10' : 'border-rose-200 bg-rose-50/40'].join(' ')}>
      <div class="relative flex items-center">
        <input
          type="checkbox"
          class="peer h-5 w-5 opacity-0 absolute cursor-pointer"
          bind:checked={$settings.whoisLookup}
          disabled={$isAnalyzing}
        />
        <div class="h-5 w-5 bg-white border-2 border-rose-300 rounded peer-checked:bg-modern-danger peer-checked:border-modern-danger transition-all flex items-center justify-center">
          <svg class="w-3.5 h-3.5 text-white opacity-0 peer-checked:opacity-100 transition-opacity" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
        </div>
      </div>
      <div>
        <div class={['text-sm font-bold', isDark ? 'text-rose-200' : 'text-rose-900'].join(' ')}>{$t('controlPanel.whoisLookup')}</div>
        <div class={['text-[11px] leading-tight mt-1 font-medium', isDark ? 'text-rose-300/60' : 'text-rose-700/80'].join(' ')}>May trigger network requests and leave traces (OpSec)</div>
      </div>
    </label>
  </div>
</section>
