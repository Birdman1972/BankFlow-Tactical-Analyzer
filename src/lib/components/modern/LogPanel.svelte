<script lang="ts">
  import { t } from '$lib/i18n';
  import { afterUpdate } from 'svelte';
  import { logs, clearLogs } from '$lib/stores/app';
  import { theme } from '$lib/stores/theme';

  let logContainer: HTMLDivElement;

  afterUpdate(() => {
    if (!logContainer) return;
    logContainer.scrollTop = logContainer.scrollHeight;
  });

  function formatTime(date: Date): string {
    return date.toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  }

  $: isDark = $theme === 'dark';
  $: surfaceClass = isDark ? 'border-slate-800 bg-slate-900 shadow-sm' : 'border-modern-border bg-white shadow-sm';
  $: subClass = isDark ? 'text-slate-400' : 'text-slate-500';
</script>

<section class={['rounded-2xl border p-6 flex flex-col h-full shadow-sm', surfaceClass].join(' ')}>
  <div class="flex items-center justify-between gap-4 mb-6">
    <h2 class={['text-sm font-bold uppercase tracking-wider', subClass].join(' ')}>{$t('logConsole.title')}</h2>
    <button
      type="button"
      class={['text-[11px] font-bold uppercase tracking-widest px-2 py-1 rounded transition-colors', isDark ? 'text-slate-400 hover:text-slate-100 hover:bg-slate-800' : 'text-slate-500 hover:text-slate-900 hover:bg-slate-100'].join(' ')}
      on:click={clearLogs}
    >
      {$t('logConsole.clear')}
    </button>
  </div>

  <div
    bind:this={logContainer}
    class="flex-1 rounded-xl border border-slate-800 bg-slate-950 p-4 font-mono text-[11px] text-slate-300 overflow-auto shadow-inner"
  >
    {#if $logs.length === 0}
      <div class="h-full flex items-center justify-center text-slate-600 italic">
        System idling...
      </div>
    {/if}
    {#each $logs as entry}
      <div class="flex gap-3 mb-2 leading-relaxed group">
        <span class="text-slate-600 tabular-nums shrink-0">{formatTime(entry.timestamp)}</span>
        <span class={['font-bold shrink-0', 
          entry.level === 'error' ? 'text-rose-500' : 
          entry.level === 'warning' ? 'text-amber-500' : 
          entry.level === 'success' ? 'text-emerald-500' : 
          'text-modern-primary'].join(' ')}>
          [{entry.level.toUpperCase()}]
        </span>
        <span class="text-slate-100 break-all">{entry.message}</span>
      </div>
    {/each}
  </div>
</section>
