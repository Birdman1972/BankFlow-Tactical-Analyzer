<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { FileInfo } from '../stores/app';

  export let file: FileInfo;
  export let requiredColumns: string[] = [];

  const dispatch = createEventDispatcher();

  let detectedHeaders: string[] = [];
  let mapping: Record<string, string> = {};
  let loading = true;
  let error: string | null = null;

  // Initialize mapping with empty strings
  $: if (requiredColumns) {
    requiredColumns.forEach(col => {
      if (!mapping[col]) mapping[col] = "";
    });
  }

  onMount(async () => {
    if (!file.path) {
      error = "File path missing";
      loading = false;
      return;
    }

    try {
      detectedHeaders = await invoke('get_file_headers', { path: file.path });
      // Auto-guess mapping based on exact or partial match
      autoMap();
      loading = false;
    } catch (e) {
      error = typeof e === 'string' ? e : "Failed to load headers";
      loading = false;
    }
  });

  function autoMap() {
    requiredColumns.forEach(req => {
        // Simple heuristic: exact match or partial match
        const found = detectedHeaders.find(h => 
            h.toLowerCase() === req.toLowerCase() || 
            h.includes(req) || 
            req.includes(h)
        );
        if (found) {
            mapping[req] = found;
        }
    });
    mapping = { ...mapping }; // Trigger reactivity
  }

  function handleApply() {
    // Generate mapping map: { "timestamp": "CustomTime" }
    // We need to map "internal key" to "file header".
    // But here 'requiredColumns' might be display names.
    // Let's assume the parent passes internal keys (e.g. 'timestamp', 'account').
    // Wait, the Parser expects specific keys like "timestamp", "account".
    // The UI should display friendly names but map to these keys.
    // For simplicity, let's assume requiredColumns are the KEYS the parser expects
    // OR we manage a map of Display Name -> Key.
    
    // For now, let's emit the raw mapping: Key -> Selected Header
    dispatch('apply', mapping);
  }

  function handleCancel() {
    dispatch('cancel');
  }

  // Helper to check if all fields are mapped
  $: isValid = Object.values(mapping).every(v => v && v.length > 0);
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 backdrop-blur-sm">
  <div class="bg-[var(--color-surface)] border border-[var(--color-border)] rounded-lg shadow-2xl w-full max-w-2xl p-6 relative animate-in fade-in zoom-in duration-200">
    
    <h2 class="text-2xl font-bold mb-4 text-[var(--color-text-primary)] flex items-center gap-2">
      <span>🔧</span> Smart Repair: {file.filename}
    </h2>

    <div class="mb-4 text-[var(--color-text-secondary)] text-sm">
      <p>The file headers do not match the expected format. Please manually map the columns below.</p>
      {#if error}
        <div class="mt-2 p-2 bg-red-500/10 border border-red-500/20 text-red-400 rounded">
          Error: {error}
        </div>
      {/if}
    </div>

    {#if loading}
      <div class="flex items-center justify-center py-8">
        <div class="animate-spin h-8 w-8 border-4 border-[var(--color-primary)] border-t-transparent rounded-full"></div>
      </div>
    {:else}
      <div class="grid grid-cols-2 gap-4 mb-6">
        <div class="font-semibold text-[var(--color-text-secondary)] uppercase text-xs tracking-wider">Required Column</div>
        <div class="font-semibold text-[var(--color-text-secondary)] uppercase text-xs tracking-wider">Map To File Header</div>

        {#each requiredColumns as col}
          <div class="flex items-center h-10 text-[var(--color-text-primary)]">
            {col}
            <span class="text-red-400 ml-1">*</span>
          </div>
          <div>
            <select 
              bind:value={mapping[col]} 
              class="w-full h-10 px-3 rounded bg-[var(--color-bg)] border border-[var(--color-border)] text-[var(--color-text-primary)] focus:outline-none focus:border-[var(--color-primary)] transition-colors"
            >
              <option value="">-- Select Column --</option>
              {#each detectedHeaders as header}
                <option value={header}>{header}</option>
              {/each}
            </select>
          </div>
        {/each}
      </div>
    {/if}

    <div class="flex justify-end gap-3 mt-6 pt-4 border-t border-[var(--color-border)]">
      <button 
        on:click={handleCancel}
        class="px-4 py-2 rounded text-[var(--color-text-secondary)] hover:bg-[var(--color-bg)] transition-colors"
      >
        Cancel
      </button>
      <button 
        on:click={handleApply}
        disabled={!isValid || loading}
        class="px-6 py-2 rounded bg-[var(--color-primary)] text-white font-medium hover:brightness-110 disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-[var(--color-primary)]/20"
      >
        Repair & Load
      </button>
    </div>
  </div>
</div>
