<script lang="ts">
  import { t } from '$lib/i18n';
  import { get } from 'svelte/store';
  import { currentPlatform } from '../stores/platform';
  import { submitFeedback } from '$lib/services/feedbackService';
  import pkg from '../../../package.json';

  type FeedbackType = 'feature' | 'bug' | 'ux';

  interface FeedbackPayload {
    type: FeedbackType;
    title: string;
    description: string;
    version: string;
    platform: 'tauri' | 'web' | 'unknown';
    createdAt: string;
  }

  interface Props {
    onSubmit?: (payload: FeedbackPayload) => Promise<void>;
  }

  let { onSubmit }: Props = $props();

  let form = $state({
    type: 'feature' as FeedbackType,
    title: '',
    description: '',
  });

  let errors = $state({
    type: '',
    title: '',
    description: '',
  });

  let isSubmitting = $state(false);
  let status = $state<'idle' | 'success' | 'error'>('idle');
  let statusMessageKey = $state('');

  function validate(): boolean {
    errors.type = form.type ? '' : 'feedbackForm.errors.typeRequired';
    errors.title = form.title.trim() ? '' : 'feedbackForm.errors.titleRequired';
    errors.description = form.description.trim() ? '' : 'feedbackForm.errors.descriptionRequired';

    return !errors.type && !errors.title && !errors.description;
  }

  function buildPayload(): FeedbackPayload {
    const platform = get(currentPlatform);
    const version = pkg?.version ?? '0.0.0';

    return {
      type: form.type,
      title: form.title.trim(),
      description: form.description.trim(),
      version,
      platform,
      createdAt: new Date().toISOString(),
    };
  }

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    if (!validate()) {
      status = 'idle';
      statusMessageKey = '';
      return;
    }

    isSubmitting = true;
    status = 'idle';
    statusMessageKey = '';

    try {
      const payload = buildPayload();
      if (onSubmit) {
        await onSubmit(payload);
        status = 'success';
        statusMessageKey = 'feedbackForm.success';
      } else {
        const result = await submitFeedback(payload);
        if (result.ok) {
          status = 'success';
          statusMessageKey = result.queued ? 'feedbackForm.queued' : 'feedbackForm.success';
        } else {
          status = result.queued ? 'success' : 'error';
          statusMessageKey = result.queued ? 'feedbackForm.queued' : 'feedbackForm.error';
        }
      }
      form.title = '';
      form.description = '';
    } catch (error) {
      console.error('Feedback submit failed:', error);
      status = 'error';
      statusMessageKey = 'feedbackForm.error';
    } finally {
      isSubmitting = false;
    }
  }
</script>

<form class="cyber-panel p-4 space-y-4" onsubmit={handleSubmit}>
  <div class="flex items-center justify-between">
    <h2 class="text-lg font-semibold text-neon-green">{$t('feedbackForm.title')}</h2>
    <span class="text-xs text-gray-500">{$t('feedbackForm.versionLabel')}: {pkg?.version ?? '0.0.0'}</span>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    <label class="space-y-1 md:col-span-1">
      <span class="text-xs text-gray-400">{$t('feedbackForm.typeLabel')}</span>
      <select
        class="w-full bg-cyber-card border border-cyber-border rounded px-3 py-2 text-sm text-gray-200 focus:outline-none focus:border-neon-green"
        bind:value={form.type}
      >
        <option value="feature">{$t('feedbackForm.typeFeature')}</option>
        <option value="bug">{$t('feedbackForm.typeBug')}</option>
        <option value="ux">{$t('feedbackForm.typeUx')}</option>
      </select>
      {#if errors.type}
        <div class="text-xs text-neon-pink">{$t(errors.type)}</div>
      {/if}
    </label>

    <label class="space-y-1 md:col-span-2">
      <span class="text-xs text-gray-400">{$t('feedbackForm.titleLabel')}</span>
      <input
        class="w-full bg-cyber-card border border-cyber-border rounded px-3 py-2 text-sm text-gray-200 focus:outline-none focus:border-neon-green"
        type="text"
        placeholder={$t('feedbackForm.titlePlaceholder')}
        bind:value={form.title}
      />
      {#if errors.title}
        <div class="text-xs text-neon-pink">{$t(errors.title)}</div>
      {/if}
    </label>
  </div>

  <label class="space-y-1">
    <span class="text-xs text-gray-400">{$t('feedbackForm.descriptionLabel')}</span>
    <textarea
      class="w-full min-h-[120px] bg-cyber-card border border-cyber-border rounded px-3 py-2 text-sm text-gray-200 focus:outline-none focus:border-neon-green"
      placeholder={$t('feedbackForm.descriptionPlaceholder')}
      bind:value={form.description}
    ></textarea>
    {#if errors.description}
      <div class="text-xs text-neon-pink">{$t(errors.description)}</div>
    {/if}
  </label>

  <div class="flex items-center justify-between gap-4">
    <div class="text-xs text-gray-500">
      {$t('feedbackForm.privacyNote')}
    </div>
    <button
      type="submit"
      class="btn-primary"
      disabled={isSubmitting}
    >
      {#if isSubmitting}
        {$t('feedbackForm.submitting')}
      {:else}
        {$t('feedbackForm.submit')}
      {/if}
    </button>
  </div>

  {#if status !== 'idle' && statusMessageKey}
    <div class={status === 'success' ? 'text-neon-green text-sm' : 'text-neon-pink text-sm'}>
      {$t(statusMessageKey)}
    </div>
  {/if}
</form>
