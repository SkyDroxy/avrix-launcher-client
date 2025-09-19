<template>
  <BaseModal
    :model-value="modelValue"
    title="Scan Workshop"
    width="md"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <div class="space-y-4">
      <div class="flex items-center gap-3 text-[13px]">
        <span
          class="inline-flex items-center justify-center w-6 h-6 rounded-full bg-indigo-600/30 border border-indigo-500/40"
        >
          <span
            class="w-4 h-4 border-2 border-indigo-300 border-t-transparent rounded-full animate-spin"
          />
        </span>
        <div class="font-medium">Scan en cours...</div>
      </div>
      <div
        v-if="logs.length"
        class="rounded-md bg-neutral-900/60 border border-neutral-700/70 p-2 max-h-56 overflow-auto text-[11px] font-mono leading-relaxed space-y-0.5"
      >
        <div v-for="(l, i) in logs" :key="i" class="whitespace-pre-wrap">{{ l }}</div>
      </div>
      <div v-else class="text-[11px] opacity-60 italic">Initialisation...</div>
      <div class="flex justify-end">
        <UiButton variant="ghost" size="sm" @click="cancel">Annuler</UiButton>
      </div>
    </div>
  </BaseModal>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import { ref, watch } from 'vue';

const props = defineProps<{ modelValue: boolean }>();
const emit = defineEmits<{ (e: 'update:modelValue', v: boolean): void; (e: 'cancel'): void }>();

const logs = ref<string[]>([]);
function add(line: string) {
  logs.value.push(line);
}

watch(
  () => props.modelValue,
  (v) => {
    if (v) {
      logs.value = [];
      // @ts-expect-error
      const listen = window.__TAURI__?.event?.listen?.('workshop-scan-log', (e: any) => {
        if (e?.payload) add(String(e.payload));
      });
      listen?.then((un: any) => {
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

function cancel() {
  emit('cancel');
  emit('update:modelValue', false);
}
</script>
