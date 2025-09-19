<template>
  <BaseModal
    :model-value="modelValue"
    title="Prévisualisation Plugin"
    width="sm"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <div v-if="validation" class="space-y-5">
      <div class="grid gap-1 text-[12px]">
        <div class="flex justify-between gap-2">
          <span class="opacity-60">Nom</span><span class="font-medium truncate">{{ validation.name || '-' }}</span>
        </div>
        <div class="flex justify-between gap-2">
          <span class="opacity-60">Version</span><span class="truncate">{{ validation.version || '-' }}</span>
        </div>
        <div class="flex justify-between gap-2">
          <span class="opacity-60">Environnement</span><span class="uppercase">{{ validation.environment || '-' }}</span>
        </div>
        <div class="flex justify-between gap-2">
          <span class="opacity-60">Taille</span><span>{{ humanSize(validation.size) }}</span>
        </div>
        <div class="flex justify-between gap-2">
          <span class="opacity-60">SHA-256</span><span class="font-mono text-[10px]">{{
            validation.sha256 ? shortHash(validation.sha256) : '-'
          }}</span>
        </div>
        <div class="flex justify-between gap-2" v-if="!validation.valid">
          <span class="text-red-400">Statut</span><span class="text-red-300">{{ validation.message }}</span>
        </div>
      </div>
      <div
        v-if="logs.length"
        class="rounded-md bg-neutral-900/60 border border-neutral-700/70 p-2 max-h-40 overflow-auto text-[10px] font-mono leading-relaxed space-y-0.5"
      >
        <div v-for="(l, i) in logs" :key="i" class="flex items-start gap-1">
          <span class="text-neutral-500">#</span>
          <span class="whitespace-pre-wrap flex-1">{{ l }}</span>
        </div>
      </div>
      <div class="flex justify-end gap-2">
        <UiButton variant="ghost" @click="cancel">Annuler</UiButton>
        <UiButton variant="success" :loading="loading" :disabled="loading" @click="confirm">
          {{ loading ? 'Installation...' : 'Installer' }}
        </UiButton>
      </div>
    </div>
  </BaseModal>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import { humanSize, shortHash } from '@helpers/sizeHash';
import { ref, watch } from 'vue';

const props = defineProps<{ modelValue: boolean; validation: any; loading?: boolean }>();
const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
  (e: 'confirm'): void;
  (e: 'cancel'): void;
}>();

const logs = ref<string[]>([]);
function addLog(line: string) {
  logs.value.push(line);
  setTimeout(scrollBottom, 0);
}
function scrollBottom() {
  const c = document.querySelector('.plugin-install-log-scroll') as HTMLElement | null;
  if (c) c.scrollTop = c.scrollHeight;
}

if (typeof window !== 'undefined') {
  watch(
    () => props.modelValue,
    (v) => {
      if (v) {
        logs.value = [];
        // @ts-ignore
        const unlistenPromise = window.__TAURI__?.event?.listen?.(
          'plugin-install-log',
          (e: any) => {
            if (e?.payload) addLog(String(e.payload));
          }
        );
        (unlistenPromise as any)?.then?.((un: any) => {
          watch(
            () => props.modelValue,
            (nv) => {
              if (!nv)
                try {
                  un();
                } catch (_) {}
            }
          );
        });
      }
    },
    { immediate: true }
  );
}

function cancel() {
  emit('cancel');
  emit('update:modelValue', false);
}
function confirm() {
  emit('confirm');
}
</script>
