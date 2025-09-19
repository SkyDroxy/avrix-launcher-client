<template>
  <BaseModal
    :model-value="modelValue"
    title="Logs"
    width="lg"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <div class="flex flex-col gap-3">
      <div class="flex items-center gap-3 text-[11px]">
        <UiSelect v-model="levelFilter" :options="levelOptions" label="Niveau" />
        <UiSelect v-model="sourceFilter" :options="sourceOptions" label="Source" />
        <UiButton
          size="xs"
          variant="ghost"
          class="ml-auto"
          :disabled="!filtered.length"
          @click="clear"
        >
          Vider
        </UiButton>
      </div>
      <div
        class="h-80 overflow-auto rounded border border-neutral-700/70 bg-neutral-900/40 text-[11px] font-mono"
      >
        <div v-if="!filtered.length" class="p-3 opacity-60 italic">Aucun log.</div>
        <div
          v-for="l in filtered"
          :key="l.id"
          class="px-3 py-1 flex gap-2 border-b border-neutral-800/60 last:border-b-0"
        >
          <span class="text-neutral-500 tabular-nums w-32 shrink-0">{{ formatTs(l.ts) }}</span>
          <span :class="levelClass(l.level)" class="font-semibold w-16 shrink-0">{{
            l.level
          }}</span>
          <span class="text-indigo-300 w-20 shrink-0 truncate">{{ l.source }}</span>
          <span class="flex-1 whitespace-pre-wrap">{{ l.message }}</span>
        </div>
      </div>
    </div>
  </BaseModal>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiSelect from '@components/ui/data/UiSelect.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import { useLogs, type LogEntry } from '@composables/useLogs';
import { computed, ref } from 'vue';

defineProps<{ modelValue: boolean }>();
defineEmits<{ (e: 'update:modelValue', v: boolean): void }>();

const { entries, clearLogs, addLog } = useLogs();
const levelFilter = ref<'all' | 'info' | 'warn' | 'error' | 'success'>('all');
const sourceFilter = ref<'all' | string>('all');

const levelOptions = [
  { value: 'all', label: 'Tous niveaux' },
  { value: 'info', label: 'Info' },
  { value: 'success', label: 'Succès' },
  { value: 'warn', label: 'Warn' },
  { value: 'error', label: 'Erreur' },
];
const sourceOptions = computed(() => [
  { value: 'all', label: 'Toutes sources' },
  ...sources.value.map((s) => ({ value: s, label: s })),
]);

const sources = computed<string[]>(() =>
  Array.from(new Set(entries.value.map((e: LogEntry) => e.source)))
);

const filtered = computed<LogEntry[]>(() =>
  entries.value.filter(
    (e: LogEntry) =>
      (levelFilter.value === 'all' || e.level === levelFilter.value) &&
      (sourceFilter.value === 'all' || e.source === sourceFilter.value)
  )
);

function formatTs(ts: number) {
  const d = new Date(ts);
  return d.toLocaleDateString() + ' ' + d.toLocaleTimeString();
}
function levelClass(l: string) {
  switch (l) {
    case 'error':
      return 'text-red-400';
    case 'warn':
      return 'text-yellow-300';
    case 'success':
      return 'text-green-400';
    default:
      return 'text-neutral-300';
  }
}
function clear() {
  clearLogs();
  addLog({ level: 'info', source: 'ui', message: 'Logs vidés' });
}
</script>
