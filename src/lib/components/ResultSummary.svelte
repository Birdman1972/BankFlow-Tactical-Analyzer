<script lang="ts">
  import { t } from '$lib/i18n';
  import { analysisResult } from '../stores/app';
</script>

{#if $analysisResult}
  <div class="cyber-panel p-4">
    <h2 class="text-lg font-semibold text-neon-green mb-4">{$t('resultSummary.title')}</h2>

    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <!-- Total Records -->
      <div class="bg-cyber-card rounded-lg p-4 text-center">
        <div class="text-2xl font-bold text-neon-blue">
          {$analysisResult.totalRecords.toLocaleString()}
        </div>
        <div class="text-xs text-gray-400 mt-1">{$t('resultSummary.totalRecords')}</div>
      </div>

      <!-- Matched -->
      <div class="bg-cyber-card rounded-lg p-4 text-center">
        <div class="text-2xl font-bold text-neon-green">
          {$analysisResult.matchedCount.toLocaleString()}
        </div>
        <div class="text-xs text-gray-400 mt-1">{$t('resultSummary.ipMatched')}</div>
        <div class="text-xs text-neon-green mt-1">
          {(($analysisResult.matchedCount / $analysisResult.totalRecords) * 100).toFixed(1)}%
        </div>
      </div>

      <!-- Multi-IP -->
      <div class="bg-cyber-card rounded-lg p-4 text-center">
        <div class="text-2xl font-bold text-neon-yellow">
          {$analysisResult.multiIpCount.toLocaleString()}
        </div>
        <div class="text-xs text-gray-400 mt-1">{$t('resultSummary.multiIp')}</div>
        {#if $analysisResult.multiIpCount > 0}
          <div class="text-xs text-neon-pink mt-1 animate-pulse">{$t('resultSummary.requiresReview')}</div>
        {/if}
      </div>

      <!-- Whois Queries -->
      <div class="bg-cyber-card rounded-lg p-4 text-center">
        <div class="text-2xl font-bold text-neon-blue">
          {$analysisResult.whoisQueried.toLocaleString()}
        </div>
        <div class="text-xs text-gray-400 mt-1">{$t('resultSummary.whoisQueries')}</div>
      </div>
    </div>

    <!-- Settings Used -->
    <div class="mt-4 pt-4 border-t border-cyber-border">
      <div class="text-xs text-gray-500 mb-2">{$t('resultSummary.settingsUsed')}</div>
      <div class="flex flex-wrap gap-2">
        {#if $analysisResult.settings.hideSensitive}
          <span class="text-xs px-2 py-1 bg-cyber-card rounded text-gray-400">{$t('resultSummary.sensitiveHidden')}</span>
        {/if}
        {#if $analysisResult.settings.splitIncomeExpense}
          <span class="text-xs px-2 py-1 bg-cyber-card rounded text-gray-400">{$t('resultSummary.incomeExpenseSplit')}</span>
        {/if}
        {#if $analysisResult.settings.ipCrossReference}
          <span class="text-xs px-2 py-1 bg-cyber-card rounded text-gray-400">{$t('resultSummary.ipCrossRef')}</span>
        {/if}
        {#if $analysisResult.settings.whoisLookup}
          <span class="text-xs px-2 py-1 bg-neon-pink/10 rounded text-neon-pink">{$t('resultSummary.whoisEnabled')}</span>
        {/if}
      </div>
    </div>
  </div>
{/if}
