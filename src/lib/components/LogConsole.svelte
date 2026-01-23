<script lang="ts">
  import { logs, clearLogs, type LogEntry } from '../stores/app';
  import { afterUpdate } from 'svelte';

  let logContainer: HTMLDivElement;

  // Auto-scroll to bottom when new logs are added
  afterUpdate(() => {
    if (logContainer) {
      logContainer.scrollTop = logContainer.scrollHeight;
    }
  });

  function formatTime(date: Date): string {
    return date.toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  }

  function getLevelClass(level: LogEntry['level']): string {
    switch (level) {
      case 'success':
        return 'text-neon-green';
      case 'warning':
        return 'text-neon-yellow';
      case 'error':
        return 'text-neon-pink';
      case 'system':
        return 'text-gray-500';
      default:
        return 'text-neon-blue';
    }
  }

  function getLevelPrefix(level: LogEntry['level']): string {
    switch (level) {
      case 'success':
        return '[OK]';
      case 'warning':
        return '[WARN]';
      case 'error':
        return '[ERR]';
      case 'system':
        return '[SYS]';
      default:
        return '[INFO]';
    }
  }
</script>

<div class="cyber-panel p-4 flex flex-col h-full">
  <div class="flex items-center justify-between mb-4">
    <h2 class="text-lg font-semibold text-neon-green">Log Console</h2>
    <button
      class="text-xs text-gray-500 hover:text-neon-pink transition-colors"
      on:click={clearLogs}
    >
      Clear
    </button>
  </div>

  <div
    bind:this={logContainer}
    class="flex-1 bg-cyber-bg rounded p-3 font-mono text-xs overflow-auto"
  >
    {#each $logs as entry}
      <div class="flex gap-2 leading-relaxed">
        <span class="text-gray-600">{formatTime(entry.timestamp)}</span>
        <span class={getLevelClass(entry.level)}>{getLevelPrefix(entry.level)}</span>
        <span class="text-gray-300">{entry.message}</span>
      </div>
    {/each}

    <!-- Blinking cursor -->
    <div class="mt-1">
      <span class="text-neon-green animate-pulse">_</span>
    </div>
  </div>
</div>
