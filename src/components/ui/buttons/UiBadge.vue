<template>
  <span
    :class="[
      'inline-flex items-center rounded font-medium select-none',
      sizeClasses,
      colorClasses,
      subtle ? 'bg-opacity-40' : '',
    ]"
  >
    <slot />
  </span>
</template>
<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    variant?:
      | 'default'
      | 'neutral'
      | 'indigo'
      | 'pink'
      | 'gray'
      | 'amber'
      | 'green'
      | 'red'
      | 'core'
      | 'info'
      | 'success'
      | 'warning'
      | 'danger'
      | 'accent';
    size?: 'xs' | 'sm';
    subtle?: boolean;
  }>(),
  {
    variant: 'neutral',
    size: 'xs',
    subtle: false,
  }
);

const sizeClasses = computed(() => {
  return props.size === 'sm' ? 'text-[11px] px-2 py-0.5' : 'text-[10px] px-1.5 py-0.5';
});

const colorMap: Record<string, string> = {
  neutral: 'bg-neutral-700/60 text-neutral-200',
  default: 'bg-neutral-700/60 text-neutral-200',
  indigo: 'bg-indigo-600/70 text-white',
  pink: 'bg-pink-700/70 text-white',
  gray: 'bg-neutral-600/60 text-neutral-100',
  amber: 'bg-amber-600/70 text-white',
  green: 'bg-emerald-600/70 text-white',
  red: 'bg-red-600/70 text-white',
  core: 'bg-indigo-600/70 text-white',
  info: 'bg-info-600/70 text-white',
  success: 'bg-success-600/70 text-white',
  warning: 'bg-warn-500/70   text-white',
  danger: 'bg-error-500/70   text-white',
  accent: 'bg-accent-600/70 text-white',
};

const colorClasses = computed(() => colorMap[props.variant] || colorMap.neutral);
</script>
