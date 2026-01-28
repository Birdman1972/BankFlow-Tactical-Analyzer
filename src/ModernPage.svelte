<script lang="ts">
  import { onMount } from 'svelte';
  import { t, locale, setLocale } from '$lib/i18n';
  import WarningBanner from './lib/components/WarningBanner.svelte';
  import UpdateDialog from './lib/components/UpdateDialog.svelte';
  import DownloadsDialog from '$lib/components/DownloadsDialog.svelte';
  import ToastContainer from './lib/components/ToastContainer.svelte';

  import { fileA, fileB, isAnalyzing, analysisResult } from '$lib/stores/app';
  import { fileA as fileAStore, fileB as fileBStore } from '$lib/stores/app';
  import { selectAndLoadFileA, selectAndLoadFileB, currentPlatform } from '$lib/stores/platform';
  import ThemeToggle from '$lib/components/modern/ThemeToggle.svelte';
  import FileCard from '$lib/components/modern/FileCard.svelte';
  import SettingsPanel from '$lib/components/modern/SettingsPanel.svelte';
  import ActionsBar from '$lib/components/modern/ActionsBar.svelte';
  import ResultDashboard from '$lib/components/modern/ResultDashboard.svelte';
  import LogPanel from '$lib/components/modern/LogPanel.svelte';
  import { theme } from '$lib/stores/theme';

  import {
    checkForUpdates,
    skipVersion,
    currentVersion,
    getInstalledVersion,
    type VersionInfo,
  } from '$lib/services/versionService';

  let appVersion = currentVersion;
  let updateInfo: VersionInfo | null = null;
  let showDownloads = false;

  onMount(async () => {
    appVersion = await getInstalledVersion();
    const info = await checkForUpdates();
    if (info) updateInfo = info;
  });

  function handleUpdate() {
      window.open('https://github.com/Birdman1972/BankFlow-Tactical-Analyzer/releases', '_blank');
    updateInfo = null;
  }

  function handleRemindLater() {
    updateInfo = null;
  }

  function handleSkip() {
    if (!updateInfo) return;
    skipVersion(updateInfo.version);
    updateInfo = null;
  }

  async function handleFileAClick() {
    try {
      const fileInfo = await selectAndLoadFileA();
      fileAStore.set(fileInfo);
    } catch (error) {
      console.log('File A selection:', error);
    }
  }

  async function handleFileBClick() {
    try {
      const fileInfo = await selectAndLoadFileB();
      fileBStore.set(fileInfo);
    } catch (error) {
      console.log('File B selection:', error);
    }
  }

  async function handleFileADrop(e: CustomEvent<string>) {
    if ($currentPlatform !== 'tauri') return;
    const { loadFileA } = await import('./lib/stores/tauri');
    await loadFileA(e.detail);
  }

  async function handleFileBDrop(e: CustomEvent<string>) {
    if ($currentPlatform !== 'tauri') return;
    const { loadFileB } = await import('./lib/stores/tauri');
    await loadFileB(e.detail);
  }

  $: isDark = $theme === 'dark';
  $: rootClass = isDark
    ? 'min-h-screen bg-slate-950 text-slate-100 font-sans'
    : 'min-h-screen bg-modern-bg text-slate-900 font-sans';
  $: cardSurface = isDark ? 'bg-slate-900 border-slate-800' : 'bg-white border-modern-border';
  $: sidebarClass = isDark ? 'bg-slate-900 border-slate-800' : 'bg-white border-modern-border shadow-sm';
</script>

<main class={rootClass} data-bf-ui="modern" data-bf-theme={$theme}>
  <div class="flex h-screen overflow-hidden">
    <aside class={['w-64 flex-shrink-0 border-r transition-all duration-300 z-20 flex flex-col', sidebarClass].join(' ')}>
      <div class="p-6 border-b border-inherit">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-xl bg-modern-primary text-white flex items-center justify-center font-bold text-xl shadow-lg shadow-modern-primary/20">
            BF
          </div>
          <div class="min-w-0">
            <h1 class="text-base font-bold tracking-tight truncate">{$t('app.title')}</h1>
            <div class="text-[10px] font-bold text-slate-400 uppercase tracking-widest">v{appVersion}</div>
          </div>
        </div>
      </div>

      <nav class="flex-1 overflow-y-auto p-4 space-y-2">
        <div class="text-[10px] font-bold text-slate-400 uppercase tracking-widest px-3 mb-2">Main Menu</div>
        <a 
          href="/" 
          class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-bold text-slate-500 hover:text-modern-primary hover:bg-modern-primary/5 transition-all"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path></svg>
          {$t('modern.backToClassic')}
        </a>
        
        <button 
          on:click={() => (showDownloads = true)}
          class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-bold text-slate-500 hover:text-modern-primary hover:bg-modern-primary/5 transition-all text-left"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path></svg>
          {$t('downloadsDialog.open')}
        </button>

        <div class="pt-4 mt-4 border-t border-inherit">
          <div class="text-[10px] font-bold text-slate-400 uppercase tracking-widest px-3 mb-2">Localization</div>
          <div class="flex p-1 rounded-lg bg-slate-100 dark:bg-slate-800">
            <button
              class={['flex-1 py-1.5 text-[10px] font-bold rounded-md transition-all', $locale === 'zh-TW' ? 'bg-white dark:bg-slate-700 shadow-sm text-modern-primary' : 'text-slate-500'].join(' ')}
              on:click={() => setLocale('zh-TW')}
            >
              繁中
            </button>
            <button
              class={['flex-1 py-1.5 text-[10px] font-bold rounded-md transition-all', $locale === 'en' ? 'bg-white dark:bg-slate-700 shadow-sm text-modern-primary' : 'text-slate-500'].join(' ')}
              on:click={() => setLocale('en')}
            >
              EN
            </button>
          </div>
        </div>
      </nav>

      <div class="p-4 border-t border-inherit">
        <ThemeToggle />
      </div>
    </aside>

    <div class="flex-1 flex flex-col min-w-0 overflow-hidden">
      <header class="h-16 border-b border-inherit bg-inherit flex-shrink-0 flex items-center justify-between px-6">
        <div class="flex items-center gap-2">
          <div class="h-2 w-2 rounded-full bg-modern-success animate-pulse"></div>
          <span class="text-xs font-bold text-slate-400 uppercase tracking-widest">System Active</span>
        </div>
        <div class="flex items-center gap-4">
          <span class="text-xs font-medium text-slate-500">{$currentPlatform.toUpperCase()} Mode</span>
          <div class="h-8 w-8 rounded-full bg-slate-200 dark:bg-slate-800 flex items-center justify-center text-[10px] font-bold">K</div>
        </div>
      </header>

      <div class="flex-1 overflow-y-auto p-6 space-y-6">
        <WarningBanner show={false} />

        {#if updateInfo}
          <UpdateDialog
            version={updateInfo.version}
            changelog={updateInfo.changelog.flatMap((c) => c.changes)}
            onUpdate={handleUpdate}
            onRemindLater={handleRemindLater}
            onSkip={handleSkip}
          />
        {/if}

        {#if showDownloads}
          <DownloadsDialog onClose={() => (showDownloads = false)} />
        {/if}

        <div class="grid grid-cols-1 gap-6 lg:grid-cols-3 items-start">
          <div class="lg:col-span-2 space-y-6">
            <section class={['rounded-2xl border p-6 shadow-sm', cardSurface].join(' ')}>
              <div class="flex items-center justify-between gap-4 mb-6">
                <h2 class="text-xs font-bold uppercase tracking-widest text-slate-400">Data Sources</h2>
              </div>

              <div class="grid grid-cols-1 gap-5 md:grid-cols-2">
                <FileCard
                  label="A"
                  title={$t('dropZone.transactionFile')}
                  subtitle={$t('dropZone.fileFormat')}
                  file={$fileA}
                  disabled={$isAnalyzing}
                  enableDrop={$currentPlatform === 'tauri'}
                  on:click={handleFileAClick}
                  on:drop={handleFileADrop}
                />
                <FileCard
                  label="B"
                  title={$t('dropZone.ipLogFile')}
                  subtitle={$t('dropZone.fileFormat')}
                  file={$fileB}
                  disabled={$isAnalyzing}
                  enableDrop={$currentPlatform === 'tauri'}
                  on:click={handleFileBClick}
                  on:drop={handleFileBDrop}
                />
              </div>
            </section>

            <SettingsPanel />
            <ActionsBar />

            {#if $analysisResult}
              <ResultDashboard />
            {/if}
          </div>

          <div class="h-full min-h-[500px] lg:sticky lg:top-0">
            <LogPanel />
          </div>
        </div>

        <footer class="pt-10 pb-6 text-center text-[10px] text-slate-400 font-bold uppercase tracking-widest">
          {$t('app.footer')}
        </footer>
      </div>
    </div>
  </div>

  <ToastContainer />
</main>
