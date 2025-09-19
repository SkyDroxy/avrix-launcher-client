<template>
  <div class="w-full">
    <div class="flex items-center gap-2 mb-2" v-if="label || $slots.label">
      <slot name="label">
        <label class="text-xs font-semibold text-neutral-200">{{ label }}</label>
      </slot>
      <div class="ml-auto text-[11px] text-neutral-400" v-if="hint || $slots.hint">
        <slot name="hint">{{ hint }}</slot>
      </div>
    </div>
    <div class="flex items-center gap-3" :class="disabled ? 'opacity-60 cursor-not-allowed' : ''">
      <input
        class="flex-1 appearance-none h-5 rounded-full outline-none focus:ring-0"
        type="range"
        :min="min"
        :max="max"
        :step="step"
        :disabled="disabled"
        v-model="innerValue"
      />
      <div class="w-20 text-right text-xs tabular-nums text-neutral-300">{{ displayValue }}</div>
    </div>
    <div class="flex justify-between text-[11px] text-neutral-500 mt-1">
      <div>{{ minLabel ?? format(min) }}</div>
      <div>{{ maxLabel ?? format(max) }}</div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    modelValue: number;
    min?: number;
    max?: number;
    step?: number;
    label?: string;
    hint?: string;
    minLabel?: string;
    maxLabel?: string;
    disabled?: boolean;
    unit?: 'MB' | 'GB';
  }>(),
  {
    min: 0,
    max: 100,
    step: 1,
    unit: 'MB',
    disabled: false,
  }
);

const emit = defineEmits<{ (e: 'update:modelValue', v: number): void }>();

// Bridge prop to internal v-model so programmatic changes update the thumb position
const innerValue = computed({
  get: () => props.modelValue,
  set: (v: number) => emit('update:modelValue', Number(v)),
});

function format(value: number) {
  if (props.unit === 'GB') return `${(value / 1024).toFixed(1)} Go`;
  return `${value} Mo`;
}

const displayValue = computed(() => format(props.modelValue));
</script>
<style scoped>
/* WebKit */
input[type='range'] {
  background: transparent; /* we style the track separately */
}
input[type='range']::-webkit-slider-runnable-track {
  height: 4px;
  background: rgba(60, 62, 68, 0.9); /* neutral track */
  border-radius: 9999px;
}
input[type='range']::-webkit-slider-thumb {
  -webkit-appearance: none;
  height: 14px;
  width: 14px;
  border-radius: 9999px;
  background: #9ca3af; /* neutral-400 */
  border: 2px solid rgba(0, 0, 0, 0.35);
  cursor: pointer;
  margin-top: -5px; /* center thumb on 4px track */
}

/* Firefox */
input[type='range']::-moz-range-track {
  height: 4px;
  background: rgba(60, 62, 68, 0.9);
  border-radius: 9999px;
}
input[type='range']::-moz-range-thumb {
  height: 14px;
  width: 14px;
  border-radius: 9999px;
  background: #9ca3af;
  border: 2px solid rgba(0, 0, 0, 0.35);
  cursor: pointer;
}
</style>
