<template>
  <label :class="wrapperClasses">
    <!-- Standard checkbox mode -->
    <span v-if="!switchMode" class="relative inline-flex items-center justify-center">
      <input
        ref="box"
        type="checkbox"
        :checked="modelValue"
        :disabled="disabled"
        class="peer sr-only"
        @change="onChange"
      />
      <span :class="boxClasses">
        <span v-if="indeterminate" class="w-2 h-0.5 rounded bg-indigo-300" />
        <Icon
          v-else-if="modelValue"
          name="mingcute:check-fill"
          :width="iconSize"
          class="text-indigo-200"
        />
      </span>
    </span>

    <!-- Switch mode -->
    <span v-else class="relative inline-flex items-center justify-center">
      <input
        ref="box"
        type="checkbox"
        :checked="modelValue"
        :disabled="disabled"
        class="peer sr-only"
        @change="onChange"
      />
      <span :class="switchTrackClasses" :style="switchTrackStyle">
        <span :class="switchThumbClasses" :style="switchThumbStyle" />
      </span>
    </span>
    <span v-if="$slots.default" class="select-none" :class="labelClasses"><slot /></span>
  </label>
</template>
<script setup lang="ts">
import { interactiveColors, InteractiveColorName } from '@components/ui/colors/uiColors';
import { computed, ref, watch } from 'vue';

import Icon from '@/components/common/Icon.vue';

interface Props {
  modelValue: boolean;
  disabled?: boolean;
  size?: 'xs' | 'sm' | 'md' | 'lg';
  indeterminate?: boolean;
  color?: InteractiveColorName;
  dense?: boolean; // compact mode reduces box size & label spacing
  switch?: boolean; // renders as toggle switch instead of box
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  size: 'sm',
  indeterminate: false,
  color: 'indigo',
  dense: false,
  switch: false,
});
const emit = defineEmits<{ (e: 'update:modelValue', v: boolean): void }>();

const box = ref<HTMLInputElement | null>(null);

watch(
  () => props.indeterminate,
  (v) => {
    if (box.value) box.value.indeterminate = v;
  },
  { immediate: true }
);

const sizeMap: Record<string, { normal: string; dense: string; label: string }> = {
  xs: { normal: 'w-3.5 h-3.5 text-[10px]', dense: 'w-3 h-3 text-[10px]', label: 'text-[10px]' },
  sm: { normal: 'w-4 h-4 text-[11px]', dense: 'w-3.5 h-3.5 text-[11px]', label: 'text-[11px]' },
  md: { normal: 'w-5 h-5 text-[12px]', dense: 'w-4.5 h-4.5 text-[12px]', label: 'text-[12px]' },
  lg: { normal: 'w-6 h-6 text-[13px]', dense: 'w-5 h-5 text-[13px]', label: 'text-[13px]' },
};
const iconMap: Record<string, { normal: number; dense: number }> = {
  xs: { normal: 10, dense: 9 },
  sm: { normal: 12, dense: 11 },
  md: { normal: 14, dense: 13 },
  lg: { normal: 16, dense: 15 },
};

// Switch size mapping (track width/height, thumb size, translate distance, label size)
const switchSizeMap: Record<
  string,
  { w: number; h: number; thumb: number; translate: number; label: string }
> = {
  xs: { w: 30, h: 16, thumb: 12, translate: 14, label: 'text-[10px]' },
  sm: { w: 38, h: 20, thumb: 16, translate: 16, label: 'text-[11px]' },
  md: { w: 46, h: 24, thumb: 20, translate: 20, label: 'text-[12px]' },
  lg: { w: 56, h: 30, thumb: 26, translate: 26, label: 'text-[13px]' },
};

// derive classes from shared color map
const colorSet = computed(() => interactiveColors[props.color]);

const switchMode = computed(() => props.switch);

const wrapperClasses = computed(() =>
  [
    'inline-flex items-center cursor-pointer select-none group',
    // légère variation de gap: un peu plus large pour le switch
    switchMode.value ? 'gap-2' : 'gap-1.5',
    props.dense ? 'gap-1' : '',
    props.disabled ? 'opacity-50 cursor-not-allowed' : '',
  ].join(' ')
);

const boxClasses = computed(() =>
  [
    'relative flex items-center justify-center rounded border transition',
    'bg-neutral-800/80 peer-checked:shadow-inner peer-focus-visible:outline-none peer-focus-visible:ring-2',
    props.dense ? sizeMap[props.size].dense : sizeMap[props.size].normal,
    colorSet.value.ring,
    `peer-checked:${colorSet.value.on} peer-checked:${colorSet.value.border || ''}`,
    colorSet.value.border,
    props.disabled ? 'grayscale' : 'hover:border-indigo-300',
  ].join(' ')
);

// Switch track & thumb
const switchTrackClasses = computed(() => {
  const s = switchSizeMap[props.size] || switchSizeMap.sm;
  const c = colorSet.value;
  return [
    'relative rounded-full transition-colors border flex-shrink-0 border-neutral-500/60',
    'peer-focus-visible:outline-none peer-focus-visible:ring-2',
    'flex items-center px-0.5', // center thumb vertically, 2px padding sides
    props.modelValue ? c.on : c.off,
    c.ring,
    `w-[${s.w}px] h-[${s.h}px]`,
  ].join(' ');
});

const switchThumbClasses = computed(() => {
  const s = switchSizeMap[props.size] || switchSizeMap.sm;
  const c = colorSet.value;
  return [
    'rounded-full shadow transition-transform transition-colors duration-200',
    props.modelValue ? 'scale-95' : 'scale-90',
    `w-[${s.thumb}px] h-[${s.thumb}px]`,
    c.thumb,
  ].join(' ');
});

// Inline styles to ensure arbitrary sizes always render even if Tailwind purge misses dynamic classes
const switchTrackStyle = computed(() => {
  const s = switchSizeMap[props.size] || switchSizeMap.sm;
  return {
    width: s.w + 'px',
    height: s.h + 'px',
  } as Record<string, string>;
});

const switchThumbStyle = computed(() => {
  const s = switchSizeMap[props.size] || switchSizeMap.sm;
  const padding = 2; // px each side (px-0.5)
  const travel = s.w - s.thumb - padding * 2;
  const x = props.modelValue ? travel : 0;
  return {
    width: s.thumb + 'px',
    height: s.thumb + 'px',
    transform: `translateX(${x}px)`,
  } as Record<string, string>;
});

const iconSize = computed(() =>
  props.dense ? iconMap[props.size].dense : iconMap[props.size].normal
);
const labelClasses = computed(() => {
  if (switchMode.value) {
    const s = switchSizeMap[props.size];
    // léger offset vertical pour centrer visuellement selon la taille
    const offset = props.size === 'xs' ? 'top-[1px]' : props.size === 'sm' ? 'top-[0.5px]' : '';
    return [
      'text-neutral-300 group-hover:text-neutral-100 select-none relative',
      s.label,
      offset,
      'leading-tight',
    ].join(' ');
  }
  return [
    'text-neutral-300 group-hover:text-neutral-100 select-none relative',
    sizeMap[props.size].label,
    'leading-tight',
  ].join(' ');
});

function onChange(e: Event) {
  if (props.disabled) return;
  const target = e.target as HTMLInputElement;
  emit('update:modelValue', target.checked);
}
</script>
<style scoped>
/* Additional subtle transition */
input + span {
  transition:
    background-color 0.15s,
    border-color 0.15s,
    box-shadow 0.15s;
}
</style>
