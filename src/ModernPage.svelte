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
    ? 'min-h-screen bg-gradient-to-br from-slate-950 to-slate-900 text-slate-100'
    : 'min-h-screen bg-gradient-to-br from-slate-50 to-white text-slate-900';
  $: cardSurface = isDark ? 'bg-slate-950/30 border-slate-800' : 'bg-white/70 border-slate-200';
</script>

<main class={rootClass} data-bf-ui="modern" data-bf-theme={$theme}>
  <div class="mx-auto max-w-6xl p-4 md:p-6">
    <header class={['rounded-2xl border p-4', cardSurface].join(' ')}>
      <div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-2xl bg-sky-600 text-white flex items-center justify-center font-semibold">
            BF
          </div>
          <div class="min-w-0">
            <div class="flex items-center gap-2">
              <h1 class="text-lg font-semibold truncate">{$t('app.title')}</h1>
              <span class={['rounded-full border px-2 py-0.5 text-[11px]', isDark ? 'border-slate-800 text-slate-300 bg-slate-900' : 'border-slate-200 text-slate-600 bg-white'].join(' ')}>
                {$t('modern.title')} v{appVersion}
              </span>
            </div>
            <div class={['text-xs', isDark ? 'text-slate-400' : 'text-slate-500'].join(' ')}>{$t('app.subtitle')}</div>
          </div>
        </div>

        <div class="flex flex-wrap items-center gap-2">
          <a
            class={['inline-flex items-center rounded-full border px-3 py-1.5 text-xs font-medium transition-colors', isDark ? 'border-slate-800 bg-slate-900 text-slate-200 hover:bg-slate-800' : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50'].join(' ')}
            href="/"
          >
            {$t('modern.backToClassic')}
          </a>

          <div class={['inline-flex items-center overflow-hidden rounded-full border', isDark ? 'border-slate-800 bg-slate-900' : 'border-slate-200 bg-white'].join(' ')}>
            <button
              type="button"
              class={['px-3 py-1.5 text-xs font-medium transition-colors', $locale === 'zh-TW' ? 'bg-sky-600 text-white' : (isDark ? 'text-slate-300 hover:bg-slate-800' : 'text-slate-700 hover:bg-slate-50')].join(' ')}
              on:click={() => setLocale('zh-TW')}
            >
              繁中
            </button>
            <button
              type="button"
              class={['px-3 py-1.5 text-xs font-medium transition-colors', $locale === 'en' ? 'bg-sky-600 text-white' : (isDark ? 'text-slate-300 hover:bg-slate-800' : 'text-slate-700 hover:bg-slate-50')].join(' ')}
              on:click={() => setLocale('en')}
            >
              EN
            </button>
          </div>

          <button
            type="button"
            class={['inline-flex items-center rounded-full border px-3 py-1.5 text-xs font-medium transition-colors', isDark ? 'border-slate-800 bg-slate-900 text-slate-200 hover:bg-slate-800' : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50'].join(' ')}
            on:click={() => (showDownloads = true)}
          >
            {$t('downloadsDialog.open')}
          </button>

          <ThemeToggle />
        </div>
      </div>
    </header>

    <div class="mt-4">
      <WarningBanner show={false} />
    </div>

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

    <div class="mt-6 grid grid-cols-1 gap-6 lg:grid-cols-3">
      <div class="lg:col-span-2 space-y-6">
        <section class={['rounded-2xl border p-4', cardSurface].join(' ')}>
          <div class="flex items-center justify-between gap-4">
            <h2 class="text-sm font-semibold">Inputs</h2>
            <span class={['text-[11px]', isDark ? 'text-slate-400' : 'text-slate-500'].join(' ')}>
              Platform: {$currentPlatform}
            </span>
          </div>

          <div class="mt-4 grid grid-cols-1 gap-4 md:grid-cols-2">
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

      <div class="h-[520px]">
        <LogPanel />
      </div>
    </div>

    <footer class={['mt-8 text-center text-xs', isDark ? 'text-slate-400' : 'text-slate-500'].join(' ')}>
      {$t('app.footer')}
    </footer>
  </div>

  <ToastContainer />
</main>
