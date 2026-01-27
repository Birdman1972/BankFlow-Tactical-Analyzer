<script lang="ts">
  import { toastStore } from '../stores/toast';
  import Toast from './Toast.svelte';
</script>

<div class="fixed top-6 right-6 z-[9999] pointer-events-none flex flex-col items-end">
  {#each $toastStore as toast (toast.id)}
    <Toast {...toast} />
  {/each}
</div>
