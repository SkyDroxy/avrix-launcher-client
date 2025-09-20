<template>
  <BaseModal
    :model-value="modelValue"
    title="Plugins Workshop"
    width="lg"
    placement="center"
    :body-scrollable="false"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <div class="space-y-4">
      <div v-if="!items.length" class="opacity-60 text-[12px]">Aucun résultat.</div>
      <div v-else class="max-h-[60vh] custom-scrollbar pr-2 relative">
        <div
          v-if="localLoading"
          class="absolute inset-0 bg-neutral-900/40 backdrop-blur-[1px] flex items-center justify-center z-10"
        >
          <div class="flex items-center gap-2 text-[12px] opacity-90">
            <svg
              class="animate-spin -ml-1 mr-1.5 h-4 w-4 text-neutral-200"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"
              ></path>
            </svg>
            Installation en cours…
          </div>
        </div>
        <div class="space-y-2">
          <div
            v-for="p in items"
            :key="p"
            class="rounded-lg border border-neutral-600 bg-neutral-800/40 hover:bg-neutral-800/60 transition cursor-pointer focus:outline-none"
            :class="
              selectionMap[p]
                ? 'border-transparent ring-2 ring-inset ring-indigo-500 ring-offset-0'
                : ''
            "
            @click="toggle(p)"
          >
            <div class="p-3">
              <div class="flex items-start justify-between gap-2">
                <div class="min-w-0">
                  <div class="font-semibold leading-tight truncate">{{ titleOf(p) }}</div>
                  <div class="text-[11px] opacity-70 flex items-center gap-2">
                    <UiBadge size="xs">v{{ metaMap[p]?.version || '?' }}</UiBadge>
                    <UiBadge v-if="metaMap[p]?.environment" class="uppercase">
                      {{ metaMap[p]?.environment }}
                    </UiBadge>
                    <span class="opacity-60">{{ humanSize(metaMap[p]?.size || 0) }}</span>
                  </div>
                </div>
              </div>
              <div class="text-[11px] opacity-60 mt-1">
                <span class="truncate block" :title="p">{{ shortPath(p) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="flex items-center justify-between gap-2 pt-1">
        <div class="text-[12px] opacity-70 h-7 flex items-center" v-if="loading || localLoading">
          <svg
            class="animate-spin -ml-1 mr-2 h-4 w-4 text-neutral-300"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"
            ></path>
          </svg>
          Installation en cours…
        </div>
        <div class="flex justify-end gap-2 ml-auto">
          <UiButton variant="ghost" @click="installAll" :disabled="loading || localLoading">
            Tout
          </UiButton>
          <UiButton variant="primary" @click="installSelected" :disabled="loading || localLoading">
            Installer sélection
          </UiButton>
        </div>
      </div>
    </div>
  </BaseModal>
</template>
<script setup lang="ts">
import UiBadge from '@components/ui/buttons/UiBadge.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import { invoke } from '@tauri-apps/api/core';
import { reactive, watch, onMounted, ref } from 'vue';

interface ValidationMetadata {
  valid: boolean;
  name?: string | null;
  version?: string | null;
  environment?: string | null;
  size: number; // bytes
  sha256?: string | null;
  message: string;
}

const props = defineProps<{
  modelValue: boolean;
  items: string[];
  loading?: boolean;
  manageInstall?: boolean;
}>();
const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
  (e: 'install', v: { all: boolean; selected: string[] }): void;
  (e: 'installed', v: { success: number; failed: number }): void;
}>();

const selectionMap = reactive<Record<string, boolean>>({});
const metaMap = reactive<Record<string, ValidationMetadata | undefined>>({});
const localLoading = ref(false);

function shortPath(p: string) {
  return p
    .split(/[/\\\\]/)
    .slice(-2)
    .join('/');
}
function titleOf(p: string) {
  return metaMap[p]?.name || shortPath(p);
}
function humanSize(bytes: number) {
  if (!bytes || bytes <= 0) return '0 B';
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / 1024 / 1024).toFixed(2) + ' MB';
}

async function loadMeta(list: string[]) {
  const tasks = list.map(async (p) => {
    try {
      const m = await invoke<ValidationMetadata>('validate_plugin_local', { path: p });
      metaMap[p] = m;
    } catch (_) {
      metaMap[p] = undefined;
    }
  });
  await Promise.all(tasks);
}

function toggle(p: string) {
  selectionMap[p] = !selectionMap[p];
}

watch(
  () => props.items,
  async (list) => {
    list.forEach((p) => (selectionMap[p] = selectionMap[p] ?? true));
    await loadMeta(list);
  },
  { immediate: true }
);

onMounted(async () => {
  if (props.items?.length) await loadMeta(props.items);
});

function installAll() {
  if (props.manageInstall === false) {
    emit('install', { all: true, selected: [] });
    return;
  }
  doInstall(props.items);
}
function installSelected() {
  const selected = props.items.filter((p) => selectionMap[p]);
  if (props.manageInstall === false) {
    emit('install', { all: false, selected });
    return;
  }
  doInstall(selected);
}

async function doInstall(list: string[]) {
  if (!list?.length) return;
  localLoading.value = true;
  let success = 0;
  let failed = 0;
  for (const p of list) {
    try {
      await invoke<string>('install_plugin_local', { path: p });
      success++;
    } catch (_) {
      failed++;
    }
  }
  localLoading.value = false;
  emit('installed', { success, failed });
}
</script>
