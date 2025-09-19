<template>
  <div :class="wrapperClasses">
    <div
      v-if="iconLeft || $slots.iconLeft"
      class="shrink-0 flex items-center pl-2 text-neutral-400"
    >
      <slot name="iconLeft">
        <Icon v-if="iconLeft" :name="iconLeft" :width="iconSize" />
      </slot>
    </div>
    <input
      ref="inp"
      :type="type"
      :value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      :class="inputClasses"
      @input="onInput"
      @keydown.enter="emit('enter')"
      @focus="emit('focus')"
      @blur="emit('blur')"
      autocomplete="off"
    />
    <div v-if="clearable && !!modelValue" class="pr-1 flex items-center">
      <button
        type="button"
        class="group rounded p-1 text-neutral-500 hover:text-neutral-200 hover:bg-neutral-600/40 transition"
        :title="clearTitle"
        @click="clear"
      >
        <Icon name="mingcute:close-line" :width="iconSize" />
      </button>
    </div>
    <div
      v-if="iconRight || $slots.iconRight"
      class="shrink-0 flex items-center pr-2 text-neutral-400"
    >
      <slot name="iconRight">
        <Icon v-if="iconRight" :name="iconRight" :width="iconSize" />
      </slot>
    </div>
  </div>
</template>
<script setup lang="ts">
import { computed, ref } from 'vue';

import Icon from '@/components/common/Icon.vue';

interface Props {
  modelValue: string | number | undefined | null;
  type?: string;
  size?: 'xs' | 'sm' | 'md' | 'lg';
  variant?: 'solid' | 'outline' | 'ghost';
  placeholder?: string;
  disabled?: boolean;
  invalid?: boolean;
  iconLeft?: string;
  iconRight?: string;
  clearable?: boolean;
  clearTitle?: string;
  dense?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  size: 'sm',
  variant: 'solid',
  placeholder: '',
  disabled: false,
  invalid: false,
  clearable: false,
  clearTitle: 'Clear',
  dense: false,
  iconLeft: undefined,
  iconRight: undefined,
});

const emit = defineEmits<{
  (e: 'update:modelValue', v: string | number | null): void;
  (e: 'enter'): void;
  (e: 'focus'): void;
  (e: 'blur'): void;
}>();

const inp = ref<HTMLInputElement | null>(null);

const sizePad: Record<string, string> = {
  xs: 'text-[11px]',
  sm: 'text-[12px]',
  md: 'text-[13px]',
  lg: 'text-[14px]',
};

const heightMap: Record<string, { normal: string; dense: string }> = {
  xs: { normal: 'h-6', dense: 'h-5.5' },
  sm: { normal: 'h-7', dense: 'h-6.5' },
  md: { normal: 'h-8', dense: 'h-7.5' },
  lg: { normal: 'h-9', dense: 'h-8' },
};

const variantBase: Record<string, string> = {
  solid: 'bg-neutral-800/70 border-neutral-600/60 focus:border-indigo-400/70',
  outline: 'bg-neutral-900/20 border-neutral-500/60 focus:border-indigo-400/70',
  ghost: 'bg-transparent border-neutral-600/40 focus:border-indigo-400/70',
};

const wrapperClasses = computed(() =>
  [
    'relative w-full flex items-center rounded border px-1',
    'transition focus-within:ring-2 focus-within:ring-indigo-500/30',
    props.disabled ? 'opacity-60 cursor-not-allowed' : '',
    variantBase[props.variant],
    props.invalid ? 'border-error-500/70 focus-within:ring-error-500/30' : '',
    sizePad[props.size],
    props.dense ? heightMap[props.size].dense : heightMap[props.size].normal,
  ].join(' ')
);

const inputClasses = computed(() =>
  [
    'outline-none bg-transparent w-full placeholder-neutral-500 text-neutral-200',
    'px-1',
    props.disabled ? 'cursor-not-allowed select-none' : '',
  ].join(' ')
);

const iconSize = computed(() =>
  props.size === 'xs' ? 12 : props.size === 'sm' ? 14 : props.size === 'md' ? 16 : 18
);

function onInput(e: Event) {
  const target = e.target as HTMLInputElement;
  emit('update:modelValue', target.value);
}

function clear() {
  if (props.disabled) return;
  emit('update:modelValue', '');
  // restore focus
  requestAnimationFrame(() => inp.value?.focus());
}
</script>

<style scoped>
/* Additional subtle shadow for solid variant */
.solid:not(.disabled) {
  box-shadow:
    inset 0 0 0 1px rgba(255, 255, 255, 0.02),
    0 0 0 1px rgba(0, 0, 0, 0.4);
}
</style>
