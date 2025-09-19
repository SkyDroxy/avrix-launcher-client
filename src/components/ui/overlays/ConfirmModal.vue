<template>
  <BaseModal
    :model-value="modelValue"
    :title="title"
    :width="width"
    :placement="placement"
    :closable="closable"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <div class="flex items-start gap-3">
      <div
        class="shrink-0 rounded-md p-2"
        :class="
          variant === 'danger' ? 'bg-red-500/15 text-red-400' : 'bg-indigo-500/15 text-indigo-300'
        "
      >
        <Icon
          :name="variant === 'danger' ? 'mingcute:alert-fill' : 'mingcute:question-fill'"
          :width="20"
        />
      </div>
      <div class="space-y-1 min-w-0">
        <div v-if="message" class="text-[13px] leading-relaxed whitespace-pre-line break-words">
          {{ message }}
        </div>
        <slot />
      </div>
    </div>

    <template #footer>
      <UiButton size="sm" variant="ghost" :disabled="loading" @click="$emit('cancel')">
        {{ cancelLabel }}
      </UiButton>
      <UiButton
        size="sm"
        :variant="variant === 'danger' ? 'danger' : 'primary'"
        :disabled="loading"
        :autofocus="true"
        @click="$emit('confirm')"
      >
        <span v-if="!loading">{{ confirmLabel }}</span>
        <span v-else>...</span>
      </UiButton>
    </template>
  </BaseModal>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';

import Icon from '@/components/common/Icon.vue';

import BaseModal from './BaseModal.vue';

withDefaults(
  defineProps<{
    modelValue: boolean;
    title?: string;
    message?: string;
    confirmLabel?: string;
    cancelLabel?: string;
    variant?: 'primary' | 'danger';
    loading?: boolean;
    width?: 'sm' | 'md' | 'lg' | 'xl' | '2xl';
    placement?: 'top' | 'center';
    closable?: boolean;
  }>(),
  {
    title: 'Confirmation',
    message: undefined,
    confirmLabel: 'Confirmer',
    cancelLabel: 'Annuler',
    variant: 'primary',
    loading: false,
    width: 'sm',
    placement: 'center',
    closable: true,
  }
);

defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
  (e: 'confirm'): void;
  (e: 'cancel'): void;
}>();
</script>
