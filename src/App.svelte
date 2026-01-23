<script lang="ts">
  import DropZone from './lib/components/DropZone.svelte';
  import ControlPanel from './lib/components/ControlPanel.svelte';
  import LogConsole from './lib/components/LogConsole.svelte';
  import ResultSummary from './lib/components/ResultSummary.svelte';
  import WarningBanner from './lib/components/WarningBanner.svelte';
  import {
    fileA,
    fileB,
    isAnalyzing,
    analysisResult,
  } from './lib/stores/app';
  import {
    selectAndLoadFileA,
    selectAndLoadFileB,
    clearAllFiles,
  } from './lib/stores/tauri';

  const appVersion = '2.0.0';

  async function handleFileAClick() {
    await selectAndLoadFileA();
  }

  async function handleFileBClick() {
    await selectAndLoadFileB();
  }

  async function handleClear() {
    await clearAllFiles();
  }
</script>

<main class="min-h-screen p-6">
  <!-- Header -->
  <header class="cyber-panel p-4 mb-6">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-4">
        <h1 class="text-2xl font-bold neon-text">
          BankFlow Tactical Analyzer
        </h1>
        <span class="text-xs text-gray-500 bg-cyber-card px-2 py-1 rounded">
          v{appVersion}
        </span>
      </div>
      <div class="flex items-center gap-4">
        <span class="text-sm text-gray-400">Digital Forensics Tool</span>
        {#if $fileA || $fileB}
          <button
            class="text-xs text-gray-500 hover:text-neon-pink transition-colors"
            on:click={handleClear}
            disabled={$isAnalyzing}
          >
            Clear All
          </button>
        {/if}
      </div>
    </div>
  </header>

  <!-- Warning Banner -->
  <WarningBanner show={false} />

  <!-- Main Content Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Left Panel: File Input & Control -->
    <div class="lg:col-span-2 space-y-6">
      <!-- Drop Zones -->
      <div class="grid grid-cols-2 gap-4">
        <DropZone
          label="A"
          title="Drop Transaction File"
          subtitle="(.xlsx)"
          file={$fileA}
          disabled={$isAnalyzing}
          on:click={handleFileAClick}
        />

        <DropZone
          label="B"
          title="Drop IP Log File"
          subtitle="(.xlsx)"
          file={$fileB}
          disabled={$isAnalyzing}
          on:click={handleFileBClick}
        />
      </div>

      <!-- Control Panel -->
      <ControlPanel />

      <!-- Result Summary (shown after analysis) -->
      {#if $analysisResult}
        <ResultSummary />
      {/if}
    </div>

    <!-- Right Panel: Log Console -->
    <div class="h-[500px]">
      <LogConsole />
    </div>
  </div>

  <!-- Footer -->
  <footer class="mt-6 text-center text-xs text-gray-600">
    Built with Tauri + Svelte + Rust | Offline-First Design
  </footer>
</main>
