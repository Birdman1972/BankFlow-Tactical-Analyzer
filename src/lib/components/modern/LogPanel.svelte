<script lang="ts">
  import { t } from '$lib/i18n';
  import { afterUpdate } from 'svelte';
  import { logs, clearLogs, type LogEntry } from '$lib/stores/app';
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

  function levelBadge(level: LogEntry['level']): string {
    switch (level) {
      case 'success':
        return 'bg-emerald-50 text-emerald-700 border-emerald-200';
      case 'warning':
        return 'bg-amber-50 text-amber-800 border-amber-200';
      case 'error':
        return 'bg-rose-50 text-rose-700 border-rose-200';
      case 'system':
        return 'bg-slate-100 text-slate-700 border-slate-200';
      default:
        return 'bg-sky-50 text-sky-700 border-sky-200';
    }
  }

  $: isDark = $theme === 'dark';
  $: surfaceClass = isDark ? 'border-slate-800 bg-slate-950/30' : 'border-slate-200 bg-white';
  $: titleClass = isDark ? 'text-slate-100' : 'text-slate-900';
</script>

<section class={['rounded-2xl border p-4 flex flex-col h-full', surfaceClass].join(' ')}>
  <div class="flex items-center justify-between gap-4">
    <h2 class={['text-sm font-semibold', titleClass].join(' ')}>{$t('logConsole.title')}</h2>
    <button
      type="button"
      class={['text-xs font-medium', isDark ? 'text-slate-300 hover:text-slate-100' : 'text-slate-600 hover:text-slate-900'].join(' ')}
      on:click={clearLogs}
    >
      {$t('logConsole.clear')}
    </button>
  </div>

  <div
    bind:this={logContainer}
    class="mt-4 flex-1 rounded-xl border border-slate-200 bg-slate-950 p-3 font-mono text-xs text-slate-100 overflow-auto"
  >
    {#each $logs as entry}
      <div class="flex gap-2 leading-relaxed">
        <span class="text-slate-400 tabular-nums">{formatTime(entry.timestamp)}</span>
        <span class={['inline-flex items-center rounded border px-1.5 py-0.5 text-[10px]', levelBadge(entry.level)].join(' ')}>
          {entry.level.toUpperCase()}
        </span>
        <span class="text-slate-100">{entry.message}</span>
      </div>
    {/each}
  </div>
</section>
