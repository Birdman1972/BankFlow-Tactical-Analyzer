<script lang="ts">
  import { t } from '$lib/i18n';
  import {
    currentPage,
    isSidebarCollapsed,
    isSidebarOpen,
    navigate,
    toggleCollapse,
    closeSidebar,
  } from '$lib/stores/router';

  const navItems = [
    { id: 'home', labelKey: 'nav.home' },
    { id: 'about', labelKey: 'nav.about' },
    { id: 'feedback', labelKey: 'nav.feedback' },
  ] as const;

  function handleNavigate(target: (typeof navItems)[number]['id']) {
    navigate(target);
    closeSidebar();
  }
</script>

{#if $isSidebarOpen}
  <button
    type="button"
    class="fixed inset-0 bg-black/60 z-40 lg:hidden"
    aria-label={$t('nav.collapse')}
    onclick={closeSidebar}
  ></button>
{/if}

<aside
  class={`fixed inset-y-0 left-0 z-50 w-64 bg-cyber-panel border-r border-cyber-border transition-transform duration-200 ease-out
    ${$isSidebarOpen ? 'translate-x-0' : '-translate-x-full'} lg:translate-x-0
    ${$isSidebarCollapsed ? 'lg:w-16' : 'lg:w-64'}`}
>
  <div class="flex h-full flex-col">
    <div class="flex items-center justify-between px-4 py-4 border-b border-cyber-border">
      <div class="flex items-center gap-3">
        <div class="h-8 w-8 rounded-md bg-cyber-card border border-cyber-border flex items-center justify-center text-neon-green">
          <svg viewBox="0 0 24 24" class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 3l9 6-9 6-9-6 9-6z" />
            <path d="M3 9l9 6 9-6" />
            <path d="M3 15l9 6 9-6" />
          </svg>
        </div>
        <div class={$isSidebarCollapsed ? 'lg:sr-only' : 'text-sm font-semibold text-gray-200'}>
          BankFlow
        </div>
      </div>
      <button
        class="lg:hidden text-gray-400 hover:text-neon-green transition-colors"
        onclick={closeSidebar}
        aria-label={$t('nav.collapse')}
      >
        <svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M18 6L6 18" />
          <path d="M6 6l12 12" />
        </svg>
      </button>
    </div>

    <nav class="flex-1 px-3 py-4 space-y-2">
      {#each navItems as item}
        <button
          class={`w-full flex items-center gap-3 px-3 py-2 rounded-md border transition-colors
            ${$currentPage === item.id ? 'bg-cyber-card border-neon-green text-neon-green shadow-neon' : 'border-transparent text-gray-300 hover:text-neon-green hover:bg-cyber-card'}`}
          onclick={() => handleNavigate(item.id)}
          aria-current={$currentPage === item.id ? 'page' : undefined}
          aria-label={$t(item.labelKey)}
        >
          {#if item.id === 'home'}
            <svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 10l9-7 9 7" />
              <path d="M9 22V12h6v10" />
            </svg>
          {:else if item.id === 'about'}
            <svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10" />
              <path d="M12 16v-4" />
              <path d="M12 8h.01" />
            </svg>
          {:else}
            <svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15a4 4 0 0 1-4 4H7l-4 3V5a2 2 0 0 1 2-2h12a4 4 0 0 1 4 4z" />
            </svg>
          {/if}
          <span class={$isSidebarCollapsed ? 'text-sm lg:sr-only' : 'text-sm'}>
            {$t(item.labelKey)}
          </span>
        </button>
      {/each}
    </nav>

    <div class="px-3 py-4 border-t border-cyber-border">
      <button
        class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-gray-300 hover:text-neon-green hover:bg-cyber-card transition-colors"
        onclick={toggleCollapse}
        aria-label={$isSidebarCollapsed ? $t('nav.expand') : $t('nav.collapse')}
      >
        <svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          {#if $isSidebarCollapsed}
            <path d="M9 18l6-6-6-6" />
          {:else}
            <path d="M15 18l-6-6 6-6" />
          {/if}
        </svg>
        <span class={$isSidebarCollapsed ? 'text-sm lg:sr-only' : 'text-sm'}>
          {$isSidebarCollapsed ? $t('nav.expand') : $t('nav.collapse')}
        </span>
      </button>
    </div>
  </div>
</aside>
