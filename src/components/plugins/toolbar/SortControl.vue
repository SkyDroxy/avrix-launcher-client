<template>
  <div class="flex items-center gap-2">
    <UiSelect
      :model-value="modelValue"
      :options="options"
      label="Tri"
      size="xs"
      @update:model-value="$emit('update:modelValue', $event)"
    />
    <UiButton
      square
      size="xs"
      variant="ghost"
      :title="dir === 'asc' ? 'Ordre croissant' : 'Ordre décroissant'"
      @click="$emit('update:dir', dir === 'asc' ? 'desc' : 'asc')"
    >
      <Icon
        :name="dir === 'asc' ? 'mingcute:arrow-up-line' : 'mingcute:arrow-down-line'"
        :width="16"
      />
    </UiButton>
  </div>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiSelect from '@components/ui/data/UiSelect.vue';

import Icon from '@/components/common/Icon.vue';

export interface SortOption {
  value: string;
  label: string;
}

defineProps<{
  modelValue: string;
  dir: 'asc' | 'desc';
  options: SortOption[];
}>();

defineEmits<{
  (e: 'update:modelValue', v: string): void;
  (e: 'update:dir', v: 'asc' | 'desc'): void;
}>();
</script>
<style scoped></style>
