<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { FileInfo } from '../stores/app';

  export let label: string = 'A';
  export let title: string = 'Drop File Here';
  export let subtitle: string = '(.xlsx)';
  export let file: FileInfo | null = null;
  export let disabled: boolean = false;

  const dispatch = createEventDispatcher<{ click: void; drop: string }>();

  let isDragging = false;

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    if (!disabled) isDragging = true;
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

    if (disabled) return;

    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      // In Tauri, files have a path property (not in standard Web API)
      const file = files[0] as File & { path?: string };
      const filePath = file.path || file.name;
      dispatch('drop', filePath);
    }
  }

  function handleClick() {
    if (!disabled) {
      dispatch('click');
    }
  }

  $: dropZoneClasses = [
    'bg-cyber-card border border-cyber-border rounded-md p-4',
    'border-dashed border-2',
    isDragging ? 'border-neon-green bg-neon-green/10' : 'border-neon-blue',
    file ? 'border-solid border-neon-green/50 bg-neon-green/5' : '',
    disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:border-neon-green',
    'transition-all duration-200 min-h-[150px] w-full',
    'flex flex-col items-center justify-center gap-2',
  ].filter(Boolean).join(' ');
</script>

<button
  type="button"
  class={dropZoneClasses}
  on:click={handleClick}
  on:dragenter={handleDragEnter}
  on:dragleave={handleDragLeave}
  on:dragover={handleDragOver}
  on:drop={handleDrop}
>
  {#if file}
    <!-- File loaded state -->
    <div class="text-neon-green text-2xl font-bold">{label}</div>
    <div class="text-neon-green text-sm font-medium truncate max-w-full px-4">{file.filename}</div>
    <div class="text-gray-500 text-xs">
      {file.rowCount.toLocaleString()} rows
    </div>
  {:else}
    <!-- Empty state -->
    <div class="text-neon-blue text-4xl font-bold">{label}</div>
    <div class="text-gray-400 text-sm text-center">{title}</div>
    <div class="text-gray-500 text-xs">{subtitle}</div>
  {/if}
</button>
