<script lang="ts">
  import { toastStore } from '../stores/toast';
  import Toast from './Toast.svelte';

  // toasts is a proxy from Svelte 5 state
  const toasts = $derived(toastStore.all);
</script>

<div class="fixed top-6 right-6 z-[9999] pointer-events-none flex flex-col items-end">
  {#each toasts as toast (toast.id)}
    <Toast {...toast} />
  {/each}
</div>
