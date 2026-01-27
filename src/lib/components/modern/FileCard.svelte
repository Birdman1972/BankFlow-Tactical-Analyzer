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
    'group w-full rounded-2xl border p-4 text-left transition-colors',
    disabled ? 'opacity-60 cursor-not-allowed' : 'cursor-pointer',
    isDark ? 'border-slate-800 bg-slate-950/30' : 'border-slate-200 bg-white',
    file
      ? isDark
        ? 'ring-1 ring-emerald-500/20'
        : 'border-emerald-300/60 bg-emerald-50/40'
      : '',
    isDragging ? (isDark ? 'border-sky-500 bg-sky-500/10' : 'border-sky-400 bg-sky-50') : '',
  ].filter(Boolean).join(' ');

  $: titleClass = isDark ? 'text-slate-100' : 'text-slate-900';
  $: subTitleClass = isDark ? 'text-slate-400' : 'text-slate-500';
  $: pillClass = isDark
    ? 'inline-flex items-center rounded-full border border-slate-800 bg-slate-900 px-2 py-1 text-[11px] text-slate-300 group-hover:bg-slate-800'
    : 'inline-flex items-center rounded-full border border-slate-200 bg-white px-2 py-1 text-[11px] text-slate-600 group-hover:border-slate-300';
  $: fileCardClass = isDark
    ? 'mt-3 rounded-xl border border-slate-800 bg-slate-900/40 p-3'
    : 'mt-3 rounded-xl border border-slate-200 bg-white/70 p-3';
  $: fileNameClass = isDark ? 'text-slate-100' : 'text-slate-900';
  $: fileMetaClass = isDark ? 'text-slate-300' : 'text-slate-600';
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
    <div class="min-w-0">
      <div class="flex items-center gap-2">
        <span class="inline-flex h-7 w-7 items-center justify-center rounded-xl bg-slate-900 text-white text-xs font-semibold">
          {label}
        </span>
        <div class="min-w-0">
          <div class={['text-sm font-semibold truncate', titleClass].join(' ')}>{title}</div>
          <div class={['text-xs', subTitleClass].join(' ')}>{subtitle}</div>
        </div>
      </div>

      {#if file}
        <div class={fileCardClass}>
          <div class={['text-sm font-medium truncate', fileNameClass].join(' ')}>{file.filename}</div>
          <div class={['mt-1 text-xs', fileMetaClass].join(' ')}>{file.rowCount.toLocaleString()} rows</div>
        </div>
      {:else}
        <div class={['mt-3 text-xs', subTitleClass].join(' ')}>
          {#if enableDrop}
            Drag & drop (Tauri) or click to select
          {:else}
            Click to select
          {/if}
        </div>
      {/if}
    </div>

    <div class="shrink-0">
      <span class={pillClass}>
        {#if file}
          Loaded
        {:else}
          Select
        {/if}
      </span>
    </div>
  </div>
</button>
