<script lang="ts">
  import { t } from '$lib/i18n';
  import { theme, toggleTheme } from '$lib/stores/theme';

  const toggle = () => toggleTheme();
  $: isDark = $theme === 'dark';
  $: buttonClass = isDark
    ? 'inline-flex items-center gap-2 rounded-xl border border-slate-700 bg-slate-800 px-4 py-2 text-xs font-bold text-slate-100 transition-all focus:outline-none focus:ring-2 focus:ring-modern-primary/40 hover:bg-slate-700 shadow-sm'
    : 'inline-flex items-center gap-2 rounded-xl border border-modern-border bg-white px-4 py-2 text-xs font-bold text-slate-800 transition-all focus:outline-none focus:ring-2 focus:ring-modern-primary/40 hover:bg-slate-50 shadow-sm';

  $: iconClass = isDark
    ? 'inline-flex h-5 w-5 items-center justify-center rounded-lg bg-modern-primary/20 text-modern-primary'
    : 'inline-flex h-5 w-5 items-center justify-center rounded-lg bg-modern-bg text-modern-primary';
</script>

<button
  type="button"
  class={buttonClass}
  aria-label={isDark ? $t('modern.themeDark') : $t('modern.themeLight')}
  aria-pressed={isDark}
  on:click={toggle}
>
  <span class={iconClass}>
    {#if isDark}
      <span aria-hidden="true">\u263D</span>
    {:else}
      <span aria-hidden="true">\u2600</span>
    {/if}
  </span>
  <span>{isDark ? $t('modern.themeDark') : $t('modern.themeLight')}</span>
</button>
