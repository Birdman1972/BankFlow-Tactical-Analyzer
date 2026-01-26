<script lang="ts">
  import { onMount } from 'svelte';
  import { t, locale, setLocale } from '$lib/i18n';
  import DropZone from './lib/components/DropZone.svelte';
  import ControlPanel from './lib/components/ControlPanel.svelte';
  import LogConsole from './lib/components/LogConsole.svelte';
  import ResultSummary from './lib/components/ResultSummary.svelte';
  import WarningBanner from './lib/components/WarningBanner.svelte';
  import UpdateDialog from './lib/components/UpdateDialog.svelte';
  import {
    fileA,
    fileB,
    isAnalyzing,
    analysisResult,
  } from './lib/stores/app';
  import {
    selectAndLoadFileA,
    selectAndLoadFileB,
    loadFileA,
    loadFileB,
    clearAllFiles,
  } from './lib/stores/tauri';
  import { 
    checkForUpdates, 
    skipVersion, 
    currentVersion,
    type VersionInfo 
  } from '$lib/services/versionService';

  const appVersion = currentVersion;
  let updateInfo: VersionInfo | null = null;

  onMount(async () => {
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
    window.open('https://github.com/project-bob/bankflow-tactical-analyzer/releases', '_blank');
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
    await selectAndLoadFileA();
  }

  async function handleFileBClick() {
    await selectAndLoadFileB();
  }
  
  async function handleFileADrop(e: CustomEvent<string>) {
    await loadFileA(e.detail);
  }

  async function handleFileBDrop(e: CustomEvent<string>) {
    await loadFileB(e.detail);
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

        {#if $fileA || $fileB}
          <button
            class="text-xs text-gray-500 hover:text-neon-pink transition-colors"
            on:click={handleClear}
            disabled={$isAnalyzing}
          >
            {$t('app.clearAll')}
          </button>
        {/if}
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
        />

        <DropZone
          label="B"
          title={$t('dropZone.ipLogFile')}
          subtitle={$t('dropZone.fileFormat')}
          file={$fileB}
          disabled={$isAnalyzing}
          on:click={handleFileBClick}
          on:drop={handleFileBDrop}
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
    {$t('app.footer')}
  </footer>
</main>
