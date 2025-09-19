<template>
  <UiSelect
    :model-value="modelValue"
    :options="options"
    label="Env"
    size="xs"
    @update:model-value="$emit('update:modelValue', $event)"
  />
</template>
<script setup lang="ts">
import UiSelect from '@components/ui/data/UiSelect.vue';

export interface EnvOption {
  value: string;
  label: string;
}

defineProps<{
  modelValue: string;
  options: EnvOption[];
}>();

defineEmits<{ (e: 'update:modelValue', v: string): void }>();
</script>
<style scoped></style>
