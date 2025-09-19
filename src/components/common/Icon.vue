<template>
  <svg
    v-if="icon"
    :viewBox="`0 0 ${icon.width || 24} ${icon.height || 24}`"
    :style="computedStyle"
    v-html="icon.body"
    fill="currentColor"
    aria-hidden="true"
  />
  <div v-else class="text-[8px] leading-none">?</div>
</template>

<script setup lang="ts">
import raw from '@iconify-json/mingcute/icons.json';
import { computed } from 'vue';

const props = defineProps({
  name: { type: String, required: true },
  width: { type: [Number, String], default: 24 },
  height: { type: [Number, String], default: null },
});

const innerName = computed(() => props.name.replace(/^mingcute:/, ''));
const icon = computed(() => {
  type RawIcons = typeof raw.icons;
  type RawIconName = keyof RawIcons;
  const data = raw.icons[innerName.value as RawIconName] as RawIcons[RawIconName] & {
    width?: number;
    height?: number;
  };
  if (!data) console.warn('Icon non trouvée:', props.name);
  return data;
});

const computedStyle = computed(() => {
  const w = typeof props.width === 'number' ? `${props.width}px` : props.width;
  const hRaw = props.height || props.width;
  const h = typeof hRaw === 'number' ? `${hRaw}px` : hRaw;
  return {
    width: w,
    height: h,
    display: 'inline-block',
  };
});
</script>
