<template>
  <BaseModal
    :model-value="modelValue"
    title="Plugins Workshop"
    width="lg"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <div class="space-y-4">
      <div v-if="!items.length" class="opacity-60 text-[12px]">Aucun résultat.</div>
      <div v-else class="max-h-[45vh] overflow-auto pr-1">
        <ul class="space-y-1 text-[11px]">
          <li v-for="p in items" :key="p" class="flex items-center gap-2">
            <input type="checkbox" v-model="selectionMap[p]" class="accent-indigo-500" />
            <span class="truncate" :title="p">{{
              p
                .split(/[/\\\\]/)
                .slice(-2)
                .join('/')
            }}</span>
          </li>
        </ul>
      </div>
      <div class="flex justify-end gap-2 pt-1">
        <UiButton variant="ghost" @click="installAll" :disabled="loading">Tout</UiButton>
        <UiButton variant="primary" @click="installSelected" :disabled="loading">
          Installer sélection
        </UiButton>
      </div>
    </div>
  </BaseModal>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import { reactive, watch } from 'vue';

const props = defineProps<{ modelValue: boolean; items: string[]; loading?: boolean }>();
const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
  (e: 'install', v: { all: boolean; selected: string[] }): void;
}>();

const selectionMap = reactive<Record<string, boolean>>({});
watch(
  () => props.items,
  (list) => {
    list.forEach((p) => (selectionMap[p] = selectionMap[p] ?? true));
  }
);

function installAll() {
  emit('install', { all: true, selected: [] });
}
function installSelected() {
  const selected = props.items.filter((p) => selectionMap[p]);
  emit('install', { all: false, selected });
}
</script>
