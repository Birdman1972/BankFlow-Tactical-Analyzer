<script lang="ts">
  import { onMount } from 'svelte';
  import ClassicPage from './ClassicPage.svelte';
  import ModernPage from './ModernPage.svelte';

  type UiMode = 'classic' | 'modern';

  function detectUiMode(): UiMode {
    if (typeof window === 'undefined') return 'classic';
    const ui = new URLSearchParams(window.location.search).get('ui');
    return ui === 'modern' ? 'modern' : 'classic';
  }

  let uiMode: UiMode = detectUiMode();

  onMount(() => {
    const sync = () => {
      uiMode = detectUiMode();
    };

    window.addEventListener('popstate', sync);
    return () => window.removeEventListener('popstate', sync);
  });
</script>

{#if uiMode === 'modern'}
  <ModernPage />
{:else}
  <ClassicPage />
{/if}
