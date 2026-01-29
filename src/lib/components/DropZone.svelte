<script lang="ts">
  import { t } from '$lib/i18n';
  import { createEventDispatcher } from 'svelte';
  import type { FileInfo } from '../stores/app';
  import ColumnMapper from './ColumnMapper.svelte';

  export let label: string = 'A';
  export let title: string = 'Drop File Here';
  export let subtitle: string = '(.xlsx)';
  export let file: FileInfo | null = null;
  export let disabled: boolean = false;

  const dispatch = createEventDispatcher<{ click: void; drop: string; repair: { path: string; mapping: Record<string, string> } }>();

  let isDragging = false;
  let showMapper = false;

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
      const file = files[0] as any;
      const filePath = file.path || file.name;
      
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

  function handleRepairClick(e: Event) {
    e.stopPropagation();
    showMapper = true;
  }

  function handleMapperApply(e: CustomEvent) {
    showMapper = false;
    if (file) {
        dispatch('repair', { path: file.path, mapping: e.detail });
    }
  }

  function getRequiredColumns(lbl: string): string[] {
    // Basic heuristic based on label context
    if (lbl.includes('A') || lbl.includes('交易')) {
        return ["交易時間", "帳號", "支出金額", "存入金額"];
    } else {
        return ["登入時間", "帳號", "IP位址"];
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

{#if showMapper && file}
    <ColumnMapper 
        file={file} 
        requiredColumns={getRequiredColumns(label)} 
        on:apply={handleMapperApply}
        on:cancel={() => showMapper = false}
    />
{/if}

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
        <div class="flex flex-col items-center">
          <span class="text-sm font-bold text-red-400">{$t('dropZone.fileFormatError')}</span>
          <span class="text-xs text-red-300 max-w-[200px] break-words mb-2">{file.validationError}</span>
          
          {#if file.validationError.includes("Missing required columns")}
             <button 
                class="px-3 py-1 bg-blue-600 hover:bg-blue-500 text-white text-xs rounded shadow-lg animate-pulse"
                on:click={handleRepairClick}
             >
                🔧 Repair File
             </button>
          {/if}
        </div>
        <button 
            class="mt-2 text-[10px] text-red-400 underline hover:text-red-300"
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
