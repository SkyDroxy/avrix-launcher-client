<template>
  <UiButton
    :border="false"
    variant="ghost"
    block
    :focus-ring="false"
    :press="false"
    :disable-hover="true"
    class="group relative flex items-center justify-center w-full py-2 rounded-r-full !h-auto !px-0 text-left"
    :class="active ? 'text-neutral-50' : 'text-neutral-400'"
    @click="$emit('activate', section)"
    :tooltip="label"
    tooltip-placement="right"
  >
    <span
      :class="[
        'absolute left-0 top-1/2 -translate-y-1/2 rounded-r-full transition-all',
        active
          ? 'h-9 w-1.5 bg-indigo-500 shadow-[0_0_0_1px_rgba(99,102,241,0.4)]'
          : 'h-7 w-1 bg-transparent',
      ]"
    />
    <span
      class="relative z-10 flex items-center transition-transform duration-200 ease-out group-hover:scale-[1.08] group-hover:translate-x-[2px] will-change-transform"
    >
      <Icon :name="icon" :width="36" :height="36" />
    </span>
  </UiButton>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import { computed } from 'vue';

import Icon from '@/components/common/Icon.vue';
const props = defineProps({
  icon: { type: String, required: true },
  section: { type: String, required: true },
  current: { type: String, required: true },
  label: { type: String, required: true },
});
defineEmits<{ (e: 'activate', section: string): void }>();
const active = computed(() => props.current === props.section);
</script>
