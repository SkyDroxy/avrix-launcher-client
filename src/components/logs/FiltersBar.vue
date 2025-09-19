<template>
  <div class="flex items-center gap-2 flex-wrap text-[11px]">
    <div class="flex items-center gap-1">
      <UiCheckbox
        :model-value="autoFollow"
        switch
        size="xs"
        dense
        color="indigo"
        @update:model-value="$emit('toggle-follow')"
      />
      <span
        class="uppercase tracking-tight"
        :class="autoFollow ? 'text-indigo-300' : 'text-neutral-400'"
      >auto</span>
    </div>
    <UiButton size="xs" variant="ghost" @click="$emit('copy')" title="Copier tout">
      <Icon name="mingcute:copy-2-fill" :width="16" />
    </UiButton>
    <UiButton size="xs" variant="ghost" @click="$emit('export')" title="Exporter (.log)">
      <Icon name="mingcute:save-2-fill" :width="16" />
    </UiButton>
    <div class="h-4 w-px bg-neutral-600 mx-1" />
    <label v-for="lvl in levelOrder" :key="lvl" class="flex items-center gap-1 cursor-pointer">
      <UiCheckbox
        :model-value="levelEnabled[lvl]"
        size="xs"
        @update:model-value="$emit('toggle-level', lvl)"
      />
      <span :class="'uppercase tracking-tight ' + levelColorClass(lvl)">{{ lvl }}</span>
    </label>
    <div class="h-4 w-px bg-neutral-600 mx-1" />
    <span class="opacity-50">{{ shown }}/{{ total }}</span>
  </div>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiCheckbox from '@components/ui/input/UiCheckbox.vue';

import Icon from '@/components/common/Icon.vue';

import type { LogLevel } from '@interfaces/logs';

const props = defineProps<{
  levelOrder: LogLevel[];
  levelEnabled: Record<LogLevel, boolean>;
  autoFollow: boolean;
  shown: number;
  total: number;
}>();

const emit = defineEmits(['toggle-level', 'toggle-follow', 'copy', 'export']);

function levelColorClass(l: LogLevel) {
  switch (l) {
    case 'ERROR':
      return 'text-red-400';
    case 'WARN':
      return 'text-amber-300';
    case 'DEBUG':
      return 'text-fuchsia-300';
    default:
      return 'text-neutral-300';
  }
}
</script>
