<script lang="ts">
  import { t } from '$lib/i18n';
  import { onMount } from 'svelte';
  import pkg from '../../../package.json';

  interface ChangelogItem {
    version: string;
    date: string;
    changes: string[];
  }

  interface VersionInfo {
    version: string;
    releaseDate: string;
    changelog: ChangelogItem[];
  }

  let versionInfo = $state<VersionInfo | null>(null);
  let isLoading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    try {
      const res = await fetch('/version.json');
      if (!res.ok) throw new Error('version.json not found');
      versionInfo = (await res.json()) as VersionInfo;
      loadError = false;
    } catch (error) {
      console.warn('Failed to load version.json', error);
      loadError = true;
    } finally {
      isLoading = false;
    }
  });
</script>

<div class="cyber-panel p-6 space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-xl font-semibold text-neon-green">{$t('about.title')}</h2>
      <p class="text-xs text-gray-500">{$t('about.subtitle')}</p>
    </div>
    <div class="text-right">
      <div class="text-xs text-gray-400">{$t('about.currentVersion')}</div>
      <div class="text-sm text-gray-200">v{pkg?.version ?? '0.0.0'}</div>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
    <div class="lg:col-span-2 space-y-4">
      <div class="bg-cyber-card border border-cyber-border rounded-lg p-4">
        <div class="text-sm text-gray-200 font-semibold mb-2">{$t('about.versionInfo')}</div>
        <div class="text-xs text-gray-400">{$t('about.releaseDate')}: {versionInfo?.releaseDate ?? '-'}</div>
      </div>

      <div class="bg-cyber-card border border-cyber-border rounded-lg p-4">
        <div class="flex items-center justify-between mb-3">
          <div class="text-sm text-gray-200 font-semibold">{$t('about.changelog')}</div>
          {#if isLoading}
            <span class="text-xs text-gray-500">{$t('about.loading')}</span>
          {/if}
        </div>

        {#if loadError}
          <div class="text-xs text-neon-pink">{$t('about.loadError')}</div>
        {/if}

        <div class="max-h-56 overflow-y-auto pr-2 space-y-3">
          {#if versionInfo?.changelog?.length}
            {#each versionInfo.changelog as item}
              <div class="border border-cyber-border rounded-md p-3">
                <div class="flex items-center justify-between">
                  <div class="text-sm text-neon-blue font-semibold">v{item.version}</div>
                  <div class="text-xs text-gray-500">{item.date}</div>
                </div>
                <ul class="mt-2 text-xs text-gray-300 list-disc list-inside space-y-1">
                  {#each item.changes as change}
                    <li>{change}</li>
                  {/each}
                </ul>
              </div>
            {/each}
          {:else}
            <div class="text-xs text-gray-500">{$t('about.changelogEmpty')}</div>
          {/if}
        </div>
      </div>
    </div>

    <div class="space-y-4">
      <div class="bg-cyber-card border border-cyber-border rounded-lg p-4">
        <div class="text-sm text-gray-200 font-semibold mb-2">{$t('about.developer')}</div>
        <div class="text-xs text-gray-400">Antigravity AICoder</div>
      </div>

      <div class="bg-cyber-card border border-cyber-border rounded-lg p-4">
        <div class="text-sm text-gray-200 font-semibold mb-2">{$t('about.license')}</div>
        <div class="text-xs text-gray-400 whitespace-pre-line max-h-48 overflow-y-auto">{$t('about.licenseText')}</div>
      </div>
    </div>
  </div>
</div>
