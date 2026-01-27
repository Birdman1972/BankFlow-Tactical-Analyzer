<script lang="ts">
  import { t } from '$lib/i18n';
  import { theme, toggleTheme } from '$lib/stores/theme';

  const toggle = () => toggleTheme();
  $: isDark = $theme === 'dark';
  $: buttonClass = isDark
    ? 'inline-flex items-center gap-2 rounded-full border border-slate-800 bg-slate-900 px-3 py-1.5 text-xs font-medium text-slate-100 transition-colors focus:outline-none focus:ring-2 focus:ring-sky-500/40 hover:bg-slate-800'
    : 'inline-flex items-center gap-2 rounded-full border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-800 transition-colors focus:outline-none focus:ring-2 focus:ring-sky-500/40 hover:bg-slate-50';

  $: iconClass = isDark
    ? 'inline-flex h-5 w-5 items-center justify-center rounded-full bg-sky-500/20 text-sky-200'
    : 'inline-flex h-5 w-5 items-center justify-center rounded-full bg-sky-100 text-sky-700';
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
