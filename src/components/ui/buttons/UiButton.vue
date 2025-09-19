<template>
  <component :is="tooltipWrapper" v-bind="tooltipBindings">
    <button
      v-bind="sanitizedButtonAttrs"
      :type="type"
      :disabled="disabled || loading"
      :class="classes"
      @click="onClick"
    >
      <span v-if="loading" class="absolute inset-0 flex items-center justify-center">
        <span
          class="loader w-3 h-3 border-2 border-current border-t-transparent rounded-full animate-spin"
        />
      </span>
      <span class="flex items-center gap-1" :class="loading ? 'opacity-0' : ''">
        <Icon v-if="icon && !iconRight" :name="icon" :width="iconSize" />
        <slot />
        <Icon v-if="icon && iconRight" :name="icon" :width="iconSize" />
      </span>
    </button>
  </component>
</template>
<script setup lang="ts">
import { computed, useAttrs } from 'vue';

import Icon from '@/components/common/Icon.vue';
import Tooltip from '@/components/ui/overlays/Tooltip.vue';

interface Props {
  variant?:
    | 'primary'
    | 'secondary'
    | 'ghost'
    | 'danger'
    | 'success'
    | 'warning'
    | 'info'
    | 'accent';
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
  block?: boolean;
  square?: boolean;
  loading?: boolean;
  disabled?: boolean;
  icon?: string;
  iconRight?: boolean;
  type?: 'button' | 'submit' | 'reset';
  border?: boolean;
  focusRing?: boolean;
  press?: boolean;
  tooltip?: string;
  tooltipPlacement?: 'auto' | 'top' | 'bottom' | 'left' | 'right';
  disableHover?: boolean;
}
const props = withDefaults(defineProps<Props>(), {
  variant: 'secondary',
  size: 'sm',
  block: false,
  square: false,
  loading: false,
  disabled: false,
  iconRight: false,
  type: 'button',
  border: true,
  focusRing: true,
  press: true,
  tooltip: undefined,
  tooltipPlacement: 'auto',
  disableHover: false,
  icon: undefined,
});
const emit = defineEmits<{ (e: 'click', ev: MouseEvent): void }>();

// We want to intercept native title tooltips and turn them into our Tooltip
defineOptions({ inheritAttrs: false });
const attrs = useAttrs();

// Normalize tooltip text: prefer prop, else fallback to native title attr
const normalizedTooltipText = computed(() => {
  const fromAttr = (attrs.title as string | undefined) || undefined;
  return props.tooltip ?? fromAttr;
});

// Allow tooltip placement from prop, else accept kebab-case attr as fallback
const normalizedPlacement = computed(
  () => props.tooltipPlacement || ((attrs['tooltip-placement'] as string | undefined) ?? 'auto')
);

const sizeMap = {
  xs: 'text-[10px] h-6 px-2 gap-1',
  sm: 'text-[11px] h-7 px-2.5 gap-1.5',
  md: 'text-[12px] h-8 px-3 gap-2',
  lg: 'text-[13px] h-9 px-4 gap-2',
  xl: 'text-[14px] h-10 px-5 gap-3',
};

const rawVariantMap: Record<string, string> = {
  primary: 'bg-primary-600 hover:bg-primary-500 text-white',
  accent: 'bg-accent-600 hover:bg-accent-500 text-white',
  secondary: 'bg-neutral-700/70 hover:bg-neutral-600/70 text-neutral-200',
  ghost: 'bg-transparent hover:bg-neutral-700/40 text-neutral-300',
  info: 'bg-info-600 hover:bg-info-500 text-white',
  danger: 'bg-error-600 hover:bg-error-500 text-white',
  success: 'bg-success-600 hover:bg-success-500 text-white',
  warning: 'bg-warn-600 hover:bg-warn-500 text-white',
};

const variantMap = computed(() => {
  if (!props.disableHover) return rawVariantMap;
  const cleaned: Record<string, string> = {};
  Object.entries(rawVariantMap).forEach(([k, v]) => {
    cleaned[k] = v
      .split(' ')
      .filter((cls) => !cls.startsWith('hover:'))
      .join(' ');
  });
  return cleaned;
});

const variantBorder: Record<string, string> = {
  primary: 'border border-primary-500/60',
  accent: 'border border-accent-500/60',
  secondary: 'border border-neutral-600/70',
  ghost: 'border border-neutral-600/50',
  info: 'border border-info-500/60',
  danger: 'border border-error-500/60',
  success: 'border border-success-500/60',
  warning: 'border border-warn-500/70',
};

const focusRingMap: Record<string, string> = {
  primary: 'focus:ring-primary-500/40',
  accent: 'focus:ring-accent-500/40',
  secondary: 'focus:ring-neutral-400/40',
  ghost: 'focus:ring-neutral-400/30',
  info: 'focus:ring-info-500/40',
  danger: 'focus:ring-error-500/40',
  success: 'focus:ring-success-500/40',
  warning: 'focus:ring-warn-500/40',
};

const classes = computed(() =>
  [
    'relative inline-flex items-center justify-center rounded select-none font-medium disabled:opacity-50 disabled:cursor-not-allowed',
    'transition transition-colors duration-150 will-change-transform',
    props.press ? 'active:scale-[0.94]' : '',
    props.focusRing
      ? `focus:outline-none focus:ring-2 ${focusRingMap[props.variant] || focusRingMap.primary}`
      : 'focus:outline-none',
    props.square ? 'aspect-square' : '',
    props.block ? 'w-full' : '',
    sizeMap[props.size],
    variantMap.value[props.variant] || variantMap.value.secondary,
    props.border
      ? variantBorder[props.variant] || variantBorder.secondary
      : 'border border-transparent',
  ].join(' ')
);

const iconSize = computed(() => (props.size === 'xs' ? 12 : props.size === 'sm' ? 14 : 16));

function onClick(ev: MouseEvent) {
  if (props.disabled || props.loading) return;
  emit('click', ev);
}

// Auto-wrap with Tooltip if either prop.tooltip or a native title attr is present
const tooltipWrapper = computed(() => (normalizedTooltipText.value ? Tooltip : 'span'));
const tooltipBindings = computed(() =>
  normalizedTooltipText.value
    ? {
        text: normalizedTooltipText.value,
        placement: normalizedPlacement.value,
        blockTarget: props.block,
      }
    : {}
);

// Forward all non-tooltip attrs to the inner button, stripping native title to disable browser tooltip
const sanitizedButtonAttrs = computed(() => {
  const { title, 'tooltip-placement': _tp, ...rest } = attrs as Record<string, unknown>;
  return rest;
});
</script>
<style scoped>
.loader {
  animation: spin 0.8s linear infinite;
}
@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
