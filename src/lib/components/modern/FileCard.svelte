<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { FileInfo } from '$lib/stores/app';
  import { theme } from '$lib/stores/theme';

  export let label: string;
  export let title: string;
  export let subtitle: string;
  export let file: FileInfo | null = null;
  export let disabled: boolean = false;
  export let enableDrop: boolean = false;

  const dispatch = createEventDispatcher<{ click: void; drop: string }>();

  let isDragging = false;

  $: isDark = $theme === 'dark';

  function handleClick() {
    if (disabled) return;
    dispatch('click');
  }

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    if (disabled || !enableDrop) return;
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    if (disabled || !enableDrop) return;

    const files = e.dataTransfer?.files;
    if (!files || files.length === 0) return;

    const first = files[0] as unknown as { path?: string; name: string };
    const filePath = first.path ?? first.name;
    dispatch('drop', filePath);
  }

  $: containerClass = [
    'group w-full rounded-2xl border p-5 text-left transition-all duration-300 shadow-sm hover:shadow-md',
    disabled ? 'opacity-60 cursor-not-allowed' : 'cursor-pointer hover:translate-y-[-2px]',
    isDark ? 'border-slate-800 bg-slate-900' : 'border-modern-border bg-white',
    file
      ? isDark
        ? 'ring-2 ring-modern-success/20 border-modern-success/40'
        : 'ring-2 ring-modern-success/10 border-modern-success/30 bg-modern-success/[0.02]'
      : '',
    isDragging ? (isDark ? 'border-modern-primary bg-modern-primary/10' : 'border-modern-primary bg-modern-primary/[0.05]') : '',
  ].filter(Boolean).join(' ');

  $: titleClass = isDark ? 'text-slate-100' : 'text-slate-900';
  $: subTitleClass = isDark ? 'text-slate-400' : 'text-slate-500';
  $: pillClass = isDark
    ? 'inline-flex items-center rounded-lg bg-slate-800 px-2.5 py-1 text-[10px] font-bold uppercase tracking-wider text-slate-400 group-hover:text-slate-200 transition-colors'
    : 'inline-flex items-center rounded-lg bg-modern-bg px-2.5 py-1 text-[10px] font-bold uppercase tracking-wider text-slate-500 group-hover:text-modern-primary transition-colors';
  $: fileCardClass = isDark
    ? 'mt-4 rounded-xl bg-slate-950/50 p-4 border border-slate-800'
    : 'mt-4 rounded-xl bg-modern-bg p-4 border border-modern-border/50';
  $: fileNameClass = isDark ? 'text-slate-100' : 'text-slate-900';
  $: fileMetaClass = isDark ? 'text-slate-400 font-mono' : 'text-slate-500 font-mono';
</script>

<button
  type="button"
  class={containerClass}
  on:click={handleClick}
  on:dragenter={handleDragEnter}
  on:dragleave={handleDragLeave}
  on:dragover={handleDragOver}
  on:drop={handleDrop}
>
  <div class="flex items-start justify-between gap-4">
    <div class="min-w-0 flex-1">
      <div class="flex items-center gap-3">
        <div class={['flex h-10 w-10 items-center justify-center rounded-xl font-bold text-sm shadow-inner transition-colors', file ? 'bg-modern-success text-white' : 'bg-modern-primary text-white'].join(' ')}>
          {label}
        </div>
        <div class="min-w-0">
          <div class={['text-sm font-bold truncate', titleClass].join(' ')}>{title}</div>
          <div class={['text-[11px] font-medium opacity-70', subTitleClass].join(' ')}>{subtitle}</div>
        </div>
      </div>

      {#if file}
        <div class={fileCardClass}>
          <div class="flex items-center gap-2 mb-1">
            <svg class="w-3.5 h-3.5 text-modern-success" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
            <div class={['text-sm font-bold truncate', fileNameClass].join(' ')}>{file.filename}</div>
          </div>
          <div class={['text-[11px] ml-5', fileMetaClass].join(' ')}>{file.rowCount.toLocaleString()} records</div>
        </div>
      {:else}
        <div class={['mt-4 text-xs italic opacity-60 flex items-center gap-2', subTitleClass].join(' ')}>
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
          {#if enableDrop}
            Drop Excel or click to browse
          {:else}
            Click to browse
          {/if}
        </div>
      {/if}
    </div>

    <div class="shrink-0 pt-1">
      <span class={pillClass}>
        {file ? 'Active' : 'Missing'}
      </span>
    </div>
  </div>
</button>
