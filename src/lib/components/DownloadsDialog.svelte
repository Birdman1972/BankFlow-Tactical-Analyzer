<script lang="ts">
  import { t } from '$lib/i18n';
  import { onMount } from 'svelte';
  import { fade, scale } from 'svelte/transition';

  interface VersionInfo {
    version: string;
    releaseDate: string;
    changelog: { version: string; date: string; changes: string[] }[];
  }

  export let onClose: () => void;

  const releasesUrl = 'https://github.com/Birdman1972/BankFlow-Tactical-Analyzer/releases';

  let latest: VersionInfo | null = null;
  let loadError = false;

  $: latestVersion = latest?.version ?? '';
  $: windowsMsiUrl = latestVersion
    ? `${releasesUrl}/latest/download/BankFlow-Tactical-Analyzer_${latestVersion}_x64.msi`
    : `${releasesUrl}/latest`;
  $: portableZipUrl = latestVersion
    ? `${releasesUrl}/latest/download/BankFlow-Tactical-Analyzer_${latestVersion}_portable.zip`
    : `${releasesUrl}/latest`;

  async function loadVersionInfo() {
    try {
      const res = await fetch('/version.json');
      if (!res.ok) throw new Error('version.json not found');
      latest = (await res.json()) as VersionInfo;
      loadError = false;
    } catch (e) {
      console.warn('Failed to load version.json', e);
      loadError = true;
    }
  }

  onMount(() => {
    loadVersionInfo();
  });
</script>

<div
  class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4"
  transition:fade={{ duration: 200 }}
>
  <div
    class="cyber-panel w-full max-w-lg p-6 flex flex-col gap-5 shadow-2xl border-neon-blue/30"
    transition:scale={{ duration: 250, start: 0.96 }}
  >
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-bold neon-text-green">{$t('downloadsDialog.title')}</h2>
        <div class="text-xs text-gray-400 mt-1">
          {#if loadError}
            {$t('downloadsDialog.loadError')}
          {:else if latest}
            {$t('downloadsDialog.latestVersion')}: <span class="font-mono">v{latest.version}</span>
            <span class="text-gray-600">({latest.releaseDate})</span>
          {:else}
            {$t('downloadsDialog.loading')}
          {/if}
        </div>
      </div>
      <button
        class="text-gray-500 hover:text-white text-sm border border-gray-800 rounded px-2 py-1"
        onclick={onClose}
        aria-label={$t('downloadsDialog.close')}
      >
        ✕
      </button>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <a
        class="w-full py-2.5 text-center bg-neon-blue hover:bg-neon-blue/80 text-black font-bold rounded transition-all"
        href={releasesUrl}
        target="_blank"
        rel="noreferrer"
      >
        {$t('downloadsDialog.openReleases')}
      </a>
      <a
        class="w-full py-2.5 text-center bg-neon-green hover:bg-neon-green/80 text-black font-bold rounded transition-all"
        href={windowsMsiUrl}
        target="_blank"
        rel="noreferrer"
      >
        {$t('downloadsDialog.downloadWindowsMsi')}
      </a>
      <a
        class="w-full py-2.5 text-center border border-gray-800 hover:border-neon-green/30 hover:bg-white/5 text-gray-200 font-medium rounded transition-all"
        href={portableZipUrl}
        target="_blank"
        rel="noreferrer"
      >
        {$t('downloadsDialog.downloadPortableZip')}
      </a>
      <div class="rounded border border-gray-800 bg-black/40 p-3 text-xs text-gray-400">
        {$t('downloadsDialog.macosNote')}
      </div>
    </div>

    {#if latest?.changelog?.length}
      <div class="space-y-2">
        <div class="text-sm font-semibold text-gray-300">{$t('downloadsDialog.releaseNotes')}</div>
        <div class="max-h-40 overflow-y-auto pr-2 custom-scrollbar bg-black/40 rounded p-3 border border-gray-800">
          <ul class="space-y-2">
            {#each latest.changelog[0]?.changes ?? [] as item}
              <li class="text-sm text-gray-400 flex gap-2">
                <span class="text-neon-green">•</span>
                <span>{item}</span>
              </li>
            {/each}
          </ul>
        </div>
      </div>
    {/if}

    <div class="pt-2">
      <button
        class="w-full py-2 text-sm text-gray-400 hover:text-white hover:bg-white/5 rounded border border-gray-800 transition-colors"
        onclick={onClose}
      >
        {$t('downloadsDialog.close')}
      </button>
    </div>
  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: #333;
    border-radius: 2px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: #444;
  }

  .neon-text-green {
    color: #00ff9d;
    text-shadow: 0 0 10px rgba(0, 255, 157, 0.4);
  }
</style>
