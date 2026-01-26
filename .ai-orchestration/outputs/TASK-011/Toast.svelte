<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import { toastStore, type ToastType } from '../stores/toast';
  import { X } from 'lucide-svelte';

  let { id, type, message }: { id: string; type: ToastType; message: string } = $props();

  const typeStyles = {
    success: {
      bg: 'bg-black/80 border-neon-green/50',
      text: 'text-neon-green',
      icon: '✓',
      glow: 'shadow-[0_0_10px_rgba(57,255,20,0.2)]'
    },
    error: {
      bg: 'bg-black/80 border-neon-pink/50',
      text: 'text-neon-pink',
      icon: '✕',
      glow: 'shadow-[0_0_10px_rgba(255,0,255,0.2)]'
    },
    warning: {
      bg: 'bg-black/80 border-neon-yellow/50',
      text: 'text-neon-yellow',
      icon: '⚠',
      glow: 'shadow-[0_0_10px_rgba(255,255,0,0.2)]'
    },
    info: {
      bg: 'bg-black/80 border-neon-blue/50',
      text: 'text-neon-blue',
      icon: 'ℹ',
      glow: 'shadow-[0_0_10px_rgba(0,255,255,0.2)]'
    }
  };

  const style = $derived(typeStyles[type]);

  function handleDismiss() {
    toastStore.dismiss(id);
  }
</script>

<div
  in:fly={{ x: 300, duration: 400 }}
  out:fade={{ duration: 200 }}
  class="flex items-start gap-3 p-4 rounded-md border backdrop-blur-md mb-3 pointer-events-auto min-w-[300px] max-w-md {style.bg} {style.glow}"
>
  <span class="flex-shrink-0 font-bold {style.text}">{style.icon}</span>
  
  <div class="flex-grow">
    <p class="text-sm font-medium {style.text}">{message}</p>
  </div>

  <button
    on:click={handleDismiss}
    class="flex-shrink-0 text-gray-500 hover:text-white transition-colors p-0.5"
    aria-label="Dismiss"
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
  </button>
</div>
