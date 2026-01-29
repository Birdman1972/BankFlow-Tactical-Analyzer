```javascript
  import { createEventDispatcher } from 'svelte';
  import { get } from 'svelte/store';
  import { 
    loadFileA, 
    loadFileB, 
    runAnalysis, 
    type BatchScanResult, 
    type BatchPair 
  } from '../stores/platform';
  import { addLog, settings } from '../stores/app';

  export let result: BatchScanResult;
  export let isProcessing = false;

  const dispatch = createEventDispatcher();

  let queue: (BatchPair & { processStatus: 'pending' | 'processing' | 'done' | 'error' })[] = 
    result.pairs.map(p => ({ ...p, processStatus: 'pending' }));

  let processedCount = 0;

  async function startBatch() {
    isProcessing = true;
    processedCount = 0;

    for (let i = 0; i < queue.length; i++) {
        const item = queue[i];
        if (item.processStatus === 'done') continue;

        item.processStatus = 'processing';
        queue = [...queue]; // Trigger reactivity

        try {
            addLog('info', `[Batch] Processing ${item.folder_name}...`);
            
            // 1. Load Files
            await loadFileA(item.path_a);
            await loadFileB(item.path_b);
            
            // 2. Run Analysis
            const analysisResult = await runAnalysis(get(settings));
            
            if (analysisResult) {
                item.processStatus = 'done';
                addLog('success', `[Batch] Completed ${item.folder_name}`);
            } else {
                item.processStatus = 'error';
            }
        } catch (error) {
            console.error(`Batch error at ${item.folder_name}:`, error);
            item.processStatus = 'error';
            addLog('error', `[Batch] Failed ${item.folder_name}: ${error}`);
        }

        processedCount++;
        queue = [...queue];
    }

    isProcessing = false;
    addLog('success', `Batch processing finished. ${processedCount}/${queue.length} items processed.`);
  }

  function handleClose() {
    if (isProcessing) {
        if (!confirm('Batch processing is in progress. Are you sure you want to close?')) {
            return;
        }
    }
    dispatch('close');
  }
</script>

<div class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
  <div class="cyber-panel w-full max-w-4xl max-h-[90vh] flex flex-col overflow-hidden bg-cyber-darker border-neon-blue shadow-[0_0_30px_rgba(0,243,255,0.2)]">
    
    <!-- Header -->
    <div class="p-4 border-b border-gray-800 flex justify-between items-center bg-cyber-card">
      <h2 class="text-xl font-bold neon-text-blue flex items-center gap-2">
        <span class="text-2xl">📂</span> Batch Analysis Queue
      </h2>
      <button 
        class="text-gray-400 hover:text-white transition-colors"
        on:click={handleClose}
      >
        ✕
      </button>
    </div>

    <!-- Stats Bar -->
    <div class="px-6 py-3 bg-black/40 flex justify-between items-center text-sm border-b border-gray-800">
      <div class="flex gap-4">
        <span class="text-gray-400">Total Pairs: <b class="text-white">{queue.length}</b></span>
        <span class="text-gray-400">Processed: <b class="text-neon-green">{processedCount}</b></span>
        {#if result.incomplete_folders.length > 0}
            <span class="text-amber-500/80">⚠️ {result.incomplete_folders.length} folders skipped (incomplete)</span>
        {/if}
      </div>
      <div>
        {#if isProcessing}
            <span class="animate-pulse text-neon-blue font-mono">PROCESSING... {Math.round((processedCount/queue.length)*100)}%</span>
        {/if}
      </div>
    </div>

    <!-- Task List -->
    <div class="flex-1 overflow-y-auto p-4 space-y-2 font-mono text-sm">
      {#each queue as item}
        <div class="flex items-center gap-4 p-3 rounded border border-gray-800 bg-cyber-card/30 transition-all {item.processStatus === 'processing' ? 'border-neon-blue bg-neon-blue/5' : ''}">
          <!-- Status icon -->
          <div class="w-6 flex justify-center">
            {#if item.processStatus === 'pending'}
               <span class="text-gray-600">○</span>
            {:else if item.processStatus === 'processing'}
               <span class="animate-spin text-neon-blue">↻</span>
            {:else if item.processStatus === 'done'}
               <span class="text-neon-green">✓</span>
            {:else if item.processStatus === 'error'}
               <span class="text-neon-pink">✗</span>
            {/if}
          </div>

          <div class="flex-1 flex flex-col">
            <span class="font-bold text-gray-200">{item.folder_name}</span>
            <div class="flex gap-2 text-[10px] text-gray-500 truncate">
               <span class="bg-blue-900/20 px-1 rounded">A: {item.path_a.split(/[/\\]/).pop()}</span>
               <span class="bg-purple-900/20 px-1 rounded">B: {item.path_b.split(/[/\\]/).pop()}</span>
            </div>
          </div>

          <div class="text-xs">
            {#if item.processStatus === 'pending'}
                <span class="text-gray-500">Waiting</span>
            {:else if item.processStatus === 'processing'}
                <span class="text-neon-blue">Analyzing...</span>
            {:else if item.processStatus === 'done'}
                <span class="text-neon-green">Success</span>
            {:else if item.processStatus === 'error'}
                <span class="text-neon-pink">Failed</span>
            {/if}
          </div>
        </div>
      {/each}
    </div>

    <!-- Actions -->
    <div class="p-4 bg-cyber-card border-t border-gray-800 flex justify-end gap-3">
      <button 
        class="px-6 py-2 rounded border border-gray-600 text-gray-400 hover:bg-gray-800 transition-colors"
        on:click={handleClose}
        disabled={isProcessing}
      >
        Close
      </button>
      <button 
        class="px-8 py-2 rounded bg-neon-blue text-black font-bold hover:shadow-[0_0_15px_rgba(0,243,255,0.5)] transition-all disabled:opacity-50 disabled:grayscale"
        on:click={startBatch}
        disabled={isProcessing || queue.length === 0}
      >
        {isProcessing ? 'Processing...' : 'Start Batch Analysis'}
      </button>
    </div>

  </div>
</div>
