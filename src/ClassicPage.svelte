<script lang="ts">
  import { onMount } from 'svelte';
  import { t, locale, setLocale } from '$lib/i18n';
  import DropZone from './lib/components/DropZone.svelte';
  import ControlPanel from './lib/components/ControlPanel.svelte';
  import LogConsole from './lib/components/LogConsole.svelte';
  import ResultSummary from './lib/components/ResultSummary.svelte';
  import WarningBanner from './lib/components/WarningBanner.svelte';
  import UpdateDialog from './lib/components/UpdateDialog.svelte';
  import DownloadsDialog from '$lib/components/DownloadsDialog.svelte';
  import ToastContainer from './lib/components/ToastContainer.svelte';
  import ThemeToggle from '$lib/components/modern/ThemeToggle.svelte';
  import FeedbackForm from '$lib/components/FeedbackForm.svelte';
  import BatchQueue from './lib/components/BatchQueue.svelte';
  import { open } from '@tauri-apps/plugin-dialog';
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
    currentPlatform,
    scanFolder,
    type BatchScanResult,
  } from './lib/stores/platform';
  import { fileA as fileAStore, fileB as fileBStore } from './lib/stores/app';
  import {
    checkForUpdates,
    skipVersion,
    currentVersion,
    getInstalledVersion,
    type VersionInfo
  } from '$lib/services/versionService';
  import { currentPage, navigate } from '$lib/stores/router';

  let appVersion = currentVersion;
  let updateInfo: VersionInfo | null = null;
  let showDownloads = false;
  $: isFeedbackPage = $currentPage === 'feedback';

  onMount(async () => {
    appVersion = await getInstalledVersion();
    // Check for updates on startup
    const info = await checkForUpdates();
    if (info) {
      updateInfo = info;
    }
  });

  function handleUpdate() {
    // Open download page
    // Using window.open requires tauri.conf.json allowlist or shell plugin
    // Assuming basic external link capability or will silently fail if restricted
      window.open('https://github.com/Birdman1972/BankFlow-Tactical-Analyzer/releases', '_blank');
    updateInfo = null;
  }

  function handleRemindLater() {
    updateInfo = null;
  }

  function handleSkip() {
    if (updateInfo) {
      skipVersion(updateInfo.version);
      updateInfo = null;
    }
  }

  async function handleFileAClick() {
    try {
      const fileInfo = await selectAndLoadFileA();
      fileAStore.set(fileInfo);
    } catch (error) {
      // User cancelled or error - handled in platform layer
      console.log('File A selection:', error);
    }
  }

  async function handleFileBClick() {
    try {
      const fileInfo = await selectAndLoadFileB();
      fileBStore.set(fileInfo);
    } catch (error) {
      // User cancelled or error - handled in platform layer
      console.log('File B selection:', error);
    }
  }

  // File drop handlers - only work on Tauri platform
  async function handleFileADrop(e: CustomEvent<string>) {
    if ($currentPlatform === 'tauri') {
      // Dynamic import for Tauri-specific functionality
      const { loadFileA } = await import('./lib/stores/tauri');
      await loadFileA(e.detail);
    }
  }

  async function handleFileBDrop(e: CustomEvent<string>) {
    if ($currentPlatform === 'tauri') {
      const { loadFileB } = await import('./lib/stores/tauri');
      await loadFileB(e.detail);
    }
  }

  async function handleFileARepair(e: CustomEvent<{ path: string; mapping: Record<string, string> }>) {
    if ($currentPlatform === 'tauri') {
      const { loadFileA } = await import('./lib/stores/tauri');
      await loadFileA(e.detail.path, e.detail.mapping);
    }
  }

  async function handleFileBRepair(e: CustomEvent<{ path: string; mapping: Record<string, string> }>) {
    if ($currentPlatform === 'tauri') {
      const { loadFileB } = await import('./lib/stores/tauri');
      await loadFileB(e.detail.path, e.detail.mapping);
    }
  }

  async function handleClear() {
    await clearAllFiles();
    fileAStore.set(null);
    fileBStore.set(null);
    analysisResult.set(null);
  }

  let batchResult: BatchScanResult | null = null;
  let isBatchScanning = false;

  async function handleBatchClick() {
    if ($currentPlatform !== 'tauri') return;
    
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Folder for Batch Analysis'
      });

      if (selected && typeof selected === 'string') {
        isBatchScanning = true;
        batchResult = await scanFolder(selected);
      }
    } catch (error) {
      console.error('Batch scan error:', error);
    } finally {
      isBatchScanning = false;
    }
  }
</script>

<main class="min-h-screen p-6">
  {#if batchResult}
    <BatchQueue 
        result={batchResult} 
        on:close={() => batchResult = null} 
    />
  {/if}

  <!-- Header -->
  <header class="cyber-panel p-4 mb-6">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-4">
        <h1 class="text-2xl font-bold neon-text">
          {$t('app.title')}
        </h1>
        <span class="text-xs text-gray-500 bg-cyber-card px-2 py-1 rounded">
          v{appVersion}
        </span>
      </div>
      <div class="flex items-center gap-4">
        <span class="text-sm text-gray-400">{$t('app.subtitle')}</span>

        <!-- Language Switcher -->
        <div class="flex items-center gap-1 text-xs">
          <ThemeToggle />
          <span class="text-gray-600">|</span>
          <a
            class="px-2 py-1 rounded text-gray-500 hover:text-white transition-colors border border-transparent hover:border-gray-700"
            href="?ui=modern"
          >
            Modern UI
          </a>
          <span class="text-gray-600">|</span>
          <button
            class="px-2 py-1 rounded transition-colors {$locale === 'zh-TW' ? 'bg-neon-green text-black font-medium' : 'text-gray-500 hover:text-white'}"
            on:click={() => setLocale('zh-TW')}
          >
            繁中
          </button>
          <span class="text-gray-600">|</span>
          <button
            class="px-2 py-1 rounded transition-colors {$locale === 'en' ? 'bg-neon-green text-black font-medium' : 'text-gray-500 hover:text-white'}"
            on:click={() => setLocale('en')}
          >
            EN
          </button>
        </div>

        <button
          class="text-xs text-gray-500 hover:text-neon-blue transition-colors"
          on:click={() => (showDownloads = true)}
          disabled={$isAnalyzing}
        >
          {$t('downloadsDialog.open')}
        </button>

        <button
          class="text-xs text-gray-500 hover:text-neon-green transition-colors"
          on:click={() => navigate(isFeedbackPage ? 'home' : 'feedback')}
          disabled={$isAnalyzing}
        >
          {$t(isFeedbackPage ? 'nav.home' : 'nav.feedback')}
        </button>

        {#if $fileA || $fileB}
          <button
            class="text-xs text-gray-500 hover:text-neon-pink transition-colors"
            on:click={handleClear}
            disabled={$isAnalyzing}
          >
            {$t('app.clearAll')}
          </button>
        {/if}

        <button
          class="text-xs text-gray-500 hover:text-neon-blue transition-colors flex items-center gap-1 border border-gray-800 px-2 py-1 rounded hover:border-neon-blue/40"
          on:click={handleBatchClick}
          disabled={$isAnalyzing || isBatchScanning}
        >
          <span class={isBatchScanning ? 'animate-pulse' : ''}>📂</span>
          Batch Folder
        </button>
      </div>
    </div>
  </header>

  <!-- Warning Banner -->
  <WarningBanner show={false} />

  {#if updateInfo}
    <UpdateDialog
      version={updateInfo.version}
      changelog={updateInfo.changelog.flatMap(c => c.changes)}
      onUpdate={handleUpdate}
      onRemindLater={handleRemindLater}
      onSkip={handleSkip}
    />
  {/if}

  {#if showDownloads}
    <DownloadsDialog onClose={() => (showDownloads = false)} />
  {/if}

  {#if isFeedbackPage}
    <div class="cyber-panel p-6">
      <FeedbackForm />
    </div>
  {:else}
    <!-- Main Content Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left Panel: File Input & Control -->
      <div class="lg:col-span-2 space-y-6">
        <!-- Drop Zones -->
        <div class="grid grid-cols-2 gap-4">
          <DropZone
            label="A"
            title={$t('dropZone.transactionFile')}
            subtitle={$t('dropZone.fileFormat')}
            file={$fileA}
            disabled={$isAnalyzing}
            on:click={handleFileAClick}
            on:drop={handleFileADrop}
            on:repair={handleFileARepair}
          />

          <DropZone
            label="B"
            title={$t('dropZone.ipLogFile')}
            subtitle={$t('dropZone.fileFormat')}
            file={$fileB}
            disabled={$isAnalyzing}
            on:click={handleFileBClick}
            on:drop={handleFileBDrop}
            on:repair={handleFileBRepair}
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
  {/if}

  <!-- Footer -->
  <footer class="mt-6 text-center text-xs text-gray-600">
    {$t('app.footer')}
  </footer>

  <ToastContainer />
</main>
