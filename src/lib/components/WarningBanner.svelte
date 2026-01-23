<script lang="ts">
  import { settings } from '../stores/app';

  export let show: boolean = false;

  $: showBanner = show || $settings.whoisLookup;
</script>

{#if showBanner}
  <div class="flex items-start gap-4 p-4 rounded-lg mb-6 bg-neon-pink/10 border border-neon-pink/30">
    <div class="w-8 h-8 flex items-center justify-center rounded-full bg-neon-pink text-white font-bold text-lg flex-shrink-0">
      !
    </div>
    <div class="flex-1">
      <div class="text-neon-pink font-bold mb-1">OpSec Warning</div>
      <div class="text-sm text-gray-300">
        {#if $settings.whoisLookup}
          Whois lookup is enabled. External network requests will be made to ip-api.com.
          This may expose your analysis activity. Use only in authorized environments.
        {:else}
          Be cautious when handling sensitive financial data.
          Ensure proper authorization before analysis.
        {/if}
      </div>
    </div>
    <button
      class="w-6 h-6 flex items-center justify-center rounded text-gray-500 hover:text-neon-pink hover:bg-neon-pink/10 transition-colors cursor-pointer flex-shrink-0"
      on:click={() => (show = false)}
      aria-label="Dismiss warning"
    >
      x
    </button>
  </div>
{/if}
