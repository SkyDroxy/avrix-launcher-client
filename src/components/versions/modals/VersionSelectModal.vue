<template>
  <BaseModal
    :model-value="modelValue"
    title="Sélectionner une version"
    @update:model-value="onModel"
  >
    <div class="flex flex-col gap-3">
      <div v-if="loading" class="text-[12px] opacity-70">Chargement…</div>
      <template v-else>
        <div
          v-if="latest && !isInstalled(latest.version)"
          class="rounded border border-amber-600/40 bg-amber-900/20 p-2 text-[12px] flex items-center gap-2"
        >
          <Icon name="mingcute:archive-fill" :width="16" />
          <div class="flex-1">
            Installez la dernière version: <b>v{{ latest.version }}</b>
          </div>
          <UiButton size="xs" variant="primary" :disabled="installing" @click="installLatest">
            Installer
          </UiButton>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-[12px] opacity-70">Version</span>
          <UiSelect v-model="selectedId" :options="versionOptions" />
        </div>
      </template>
    </div>
    <template #footer>
      <div class="flex gap-2 ml-auto">
        <UiButton size="sm" variant="ghost" @click="$emit('update:modelValue', false)">
          Annuler
        </UiButton>
        <UiButton
          size="sm"
          variant="primary"
          :disabled="!selectedId || installing || loading"
          @click="confirm"
        >
          Sélectionner
        </UiButton>
      </div>
    </template>
  </BaseModal>
</template>

<script setup lang="ts">
import Icon from '@components/common/Icon.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiSelect from '@components/ui/data/UiSelect.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import { invoke } from '@tauri-apps/api/core';
import { computed, onMounted, ref, watch } from 'vue';

const props = defineProps<{ modelValue: boolean }>();
const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
  (e: 'confirm', id: string): void;
}>();

interface VersionEntry {
  id: string;
  version?: string;
  displayName?: string;
}
interface VersionsResult {
  versions: VersionEntry[];
  selectedId?: string | null;
}
interface AvailableVersion {
  tag: string;
  version: string;
  coreUrl: string;
  jreUrl?: string | null;
  publishedAt?: string | null;
}

const loading = ref(false);
const installing = ref(false);
const versions = ref<VersionEntry[]>([]);
const selectedId = ref<string>('');
const available = ref<AvailableVersion[]>([]);
const latest = computed(() => available.value[0] || null);

const versionOptions = computed(() =>
  versions.value.map((v) => ({
    value: v.id,
    label: (v.displayName || v.id) + (v.version ? ' (v' + v.version + ')' : ''),
  }))
);

function isInstalled(version: string): boolean {
  const id = 'v' + version.replace(/^v/i, '');
  return versions.value.some((v) => v.id.toLowerCase() === id.toLowerCase());
}

async function loadData() {
  loading.value = true;
  try {
    const res = await invoke<VersionsResult>('list_versions');
    versions.value = res.versions || [];
    selectedId.value = (res.selectedId as string) || '';
    available.value = await invoke<AvailableVersion[]>('list_available_versions');
  } finally {
    loading.value = false;
  }
}

async function installLatest() {
  if (!latest.value) return;
  installing.value = true;
  try {
    await invoke<string>('install_version_from_release', { version: latest.value.version });
    await loadData();
    // Auto-sélectionne la nouvelle version installée
    selectedId.value = 'v' + latest.value.version.replace(/^v/i, '');
  } finally {
    installing.value = false;
  }
}

function confirm() {
  if (!selectedId.value) return;
  emit('confirm', selectedId.value);
}

watch(
  () => props.modelValue,
  (v) => {
    if (v) loadData();
  }
);
onMounted(() => {
  if (props.modelValue) loadData();
});

function onModel(v: boolean) {
  emit('update:modelValue', v);
  if (!v) {
    // Tell all UiSelects to close menus when modal closes
    window.dispatchEvent(new CustomEvent('avrix:close-all-selects'));
  }
}
</script>

<style scoped></style>
