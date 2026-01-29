<script lang="ts">
  import { t } from '$lib/i18n';
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
      console.log('Dropped files:', files);
      // In Tauri, files should have a path property if configured correctly
      // We cast to any to access the path property safely
      const file = files[0] as any;
      const filePath = file.path || file.name; // Fallback to name if path is missing
      
      console.log('Detected file path:', filePath);
      
      if (!file.path) {
        console.warn('File path is missing. Drag & Drop might not work fully in browser mode.');
      }
      
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

<div
  role="button"
  tabindex="0"
  class={dropZoneClasses}
  on:click={handleClick}
  on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && handleClick()}
  on:dragenter={handleDragEnter}
  on:dragleave={handleDragLeave}
  on:dragover={handleDragOver}
  on:drop={handleDrop}
>
  {#if file}
    {#if file.validationError}
      <!-- Error State -->
      <div class="flex flex-col items-center gap-2 p-4 text-center animate-in fade-in zoom-in duration-300">
        <span class="text-3xl">⚠️</span>
        <div class="flex flex-col">
          <span class="text-sm font-bold text-red-400">{$t('dropZone.fileFormatError')}</span>
          <span class="text-xs text-red-300 max-w-[200px] break-words">{file.validationError}</span>
        </div>
        <button 
            class="mt-1 text-[10px] text-red-400 underline hover:text-red-300"
            on:click|stopPropagation={() => handleClick()}
        >
            {$t('dropZone.reselectFile')}
        </button>
      </div>
    {:else}
      <!-- File loaded state -->
      <div class="text-neon-green text-2xl font-bold">{label}</div>
      <div class="text-neon-green text-sm font-medium truncate max-w-full px-4">{file.filename}</div>
      <div class="text-gray-500 text-xs">
        {file.rowCount.toLocaleString()} {$t('dropZone.rows')}
      </div>
    {/if}
  {:else}
    <!-- Empty state -->
    <div class="text-neon-blue text-4xl font-bold">{label}</div>
    <div class="text-gray-400 text-sm text-center">{title}</div>
    <div class="text-gray-500 text-xs">{subtitle}</div>
  {/if}
</div>
