<template>
  <BaseModal
    v-model="open"
    width="md"
    placement="center"
    :closable="true"
    :close-on-backdrop="true"
  >
    <template #title>
      <div class="flex items-center gap-2">
        <Icon name="mingcute:refresh-2-fill" :width="18" />
        <span>Mise à jour disponible</span>
      </div>
    </template>
    <div class="space-y-3">
      <div class="text-sm text-neutral-300">
        <div v-if="upd">
          Version: <b>v{{ upd.version }}</b>
        </div>
        <div class="text-xs text-neutral-400 whitespace-pre-line" v-if="upd?.body">
          {{ upd.body }}
        </div>
      </div>
      <div v-if="status === 'downloading'" class="space-y-1">
        <div class="text-xs text-neutral-400">Téléchargement… {{ percent }}%</div>
        <div class="h-2 bg-neutral-800 rounded">
          <div class="h-2 bg-blue-500 rounded" :style="{ width: percent + '%' }" />
        </div>
      </div>
      <div v-if="status === 'error'" class="text-xs text-red-400">
        {{ errorMsg || 'Erreur pendant la mise à jour' }}
      </div>
    </div>
    <template #footer>
      <div class="flex items-center gap-2 justify-end">
        <UiButton variant="ghost" @click="close">Plus tard</UiButton>
        <UiButton
          :disabled="status === 'downloading' || status === 'installing'"
          @click="onInstall"
        >
          Installer & redémarrer
        </UiButton>
      </div>
    </template>
  </BaseModal>
</template>
<script setup lang="ts">
import Icon from '@components/common/Icon.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import { useUpdater } from '@composables/useUpdater';
import { computed } from 'vue';

import BaseModal from './BaseModal.vue';

const props = defineProps<{ modelValue: boolean }>();
const emit = defineEmits<{ (e: 'update:modelValue', v: boolean): void }>();
const open = computed({
  get: () => props.modelValue,
  set: (v: boolean) => emit('update:modelValue', v),
});

const { available: update, status, errorMsg, progress, downloadAndInstall } = useUpdater();
const upd = computed(() => update.value);
const percent = computed(() => {
  const total = progress.value.total || 0;
  const d = progress.value.downloaded || 0;
  return total > 0 ? Math.min(100, Math.round((d / total) * 100)) : 0;
});

function close() {
  emit('update:modelValue', false);
}
async function onInstall() {
  await downloadAndInstall();
}
</script>

<style scoped></style>
