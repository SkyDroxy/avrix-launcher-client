<template>
  <div class="flex flex-col h-full min-h-0 gap-4">
    <div class="flex items-center gap-2">
      <UiButton
        variant="secondary"
        size="sm"
        class="px-3"
        :disabled="installing"
        @click="pickLocal"
      >
        <div class="flex items-center gap-2">
          <Icon name="mingcute:folder-open-fill" :width="16" />
          <span>Installer une version (fichier/dossier)</span>
        </div>
      </UiButton>
      <UiButton
        variant="secondary"
        size="sm"
        class="px-3"
        :disabled="installing || validating"
        @click="promptUrl"
      >
        <div class="flex items-center gap-2">
          <Icon name="mingcute:link-2-fill" :width="16" />
          <span>Installer depuis une URL</span>
        </div>
      </UiButton>
      <UiButton size="sm" variant="ghost" class="px-3 ml-auto" @click="refresh">
        <div class="flex items-center gap-2">
          <Icon name="mingcute:refresh-3-fill" :width="16" />
          <span>Rafraîchir</span>
        </div>
      </UiButton>
    </div>

    <div class="grid grid-cols-12 gap-4 min-h-0 flex-1">
      <div class="col-span-9 min-h-0 overflow-auto">
        <div v-if="loading" class="text-xs opacity-70 px-1 py-2">Chargement…</div>
        <div v-else-if="!versions.length" class="text-xs opacity-60 px-1 py-2">
          Aucune version installée.
        </div>
        <div v-else class="flex flex-col gap-2">
          <div
            v-for="v in versions"
            :key="v.id"
            class="rounded-md border border-neutral-700/60 bg-neutral-800/40 p-3 flex gap-3 items-center"
          >
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <span class="font-semibold">{{ v.displayName || v.id }}</span>
                <UiBadge size="xs" v-if="v.version">v{{ v.version }}</UiBadge>
                <UiBadge size="xs" :variant="v.hasJre ? 'success' : 'neutral'">
                  {{ v.hasJre ? 'JRE inclus' : 'JRE externe' }}
                </UiBadge>
              </div>
              <div class="text-[12px] opacity-70 truncate">{{ v.dir }}</div>
            </div>
            <div class="flex items-center gap-2">
              <UiButton
                size="xs"
                :variant="selectedId === v.id ? 'primary' : 'secondary'"
                @click="select(v.id)"
              >
                <div class="flex items-center gap-2">
                  <Icon
                    :name="selectedId === v.id ? 'mingcute:check-fill' : 'mingcute:check-line'"
                    :width="14"
                  />
                  <span>{{ selectedId === v.id ? 'Sélectionnée' : 'Sélectionner' }}</span>
                </div>
              </UiButton>
              <UiButton size="xs" variant="danger" @click="del(v.id)" :disabled="deleting">
                <div class="flex items-center gap-2">
                  <Icon name="mingcute:delete-2-fill" :width="14" />
                  <span>Supprimer</span>
                </div>
              </UiButton>
            </div>
          </div>
        </div>
      </div>
      <div class="col-span-3">
        <div class="rounded-md border border-neutral-700/60 bg-neutral-800/40 p-3 text-[12px]">
          <div class="font-semibold mb-2">Aide</div>
          <ul class="list-disc pl-4 space-y-1 opacity-80">
            <li>Chaque version est isolée dans un dossier sous avrix/versions.</li>
            <li>Si un dossier jre/ est présent dans la version, il sera utilisé au lancement.</li>
            <li>Sinon, le lanceur utilisera le JRE embarqué global, JAVA_HOME ou le PATH.</li>
          </ul>
        </div>
        <div class="rounded-md border border-neutral-700/60 bg-neutral-800/40 p-3 text-[12px] mt-3">
          <div class="flex items-center gap-2 mb-2">
            <div class="font-semibold">Versions disponibles</div>
            <UiButton
              size="xs"
              variant="ghost"
              class="ml-auto"
              :disabled="loadingAvail"
              @click="loadAvailable"
            >
              Rafraîchir
            </UiButton>
          </div>
          <div v-if="loadingAvail" class="opacity-70">Chargement…</div>
          <div v-else-if="!available.length" class="opacity-60">Rien à afficher.</div>
          <div v-else class="flex flex-col gap-1 max-h-64 overflow-auto pr-1">
            <div
              v-if="installingFrom"
              class="flex flex-col gap-1 p-2 rounded bg-neutral-900/40 border border-neutral-700/50 animate-pulse"
            >
              <div class="text-[12px] font-semibold">Installation en cours…</div>
              <div class="text-[11px] opacity-70 truncate">Avrix Core {{ installingFrom }}</div>
            </div>
            <div v-for="a in available" :key="a.tag" class="flex items-center gap-2">
              <span class="truncate">{{ a.tag }}</span>
              <template v-if="isInstalled(a.version)">
                <UiButton
                  size="xs"
                  variant="secondary"
                  class="ml-auto"
                  :disabled="installing"
                  @click="repairRelease(a.version)"
                >
                  <div class="flex items-center gap-1">
                    <Icon name="mingcute:tool-fill" :width="14" />
                    <span>Réparer</span>
                  </div>
                </UiButton>
              </template>
              <template v-else>
                <UiButton
                  size="xs"
                  variant="primary"
                  class="ml-auto"
                  :disabled="installing"
                  @click="installFromRelease(a.version)"
                >
                  <div class="flex items-center gap-1">
                    <Icon name="mingcute:archive-fill" :width="14" />
                    <span>Installer</span>
                  </div>
                </UiButton>
              </template>
            </div>
          </div>
        </div>
      </div>
    </div>

    <BaseModal
      :model-value="showUrl"
      title="Installer depuis une URL"
      @update:model-value="showUrl = $event"
    >
      <template #body>
        <UiInput v-model="url" placeholder="https://… (.zip ou .jar)" />
      </template>
      <template #footer>
        <div class="flex gap-2 ml-auto">
          <UiButton size="sm" variant="ghost" @click="showUrl = false">Annuler</UiButton>
          <UiButton
            size="sm"
            variant="primary"
            :disabled="!url || installing"
            @click="installFromUrl"
          >
            Installer
          </UiButton>
        </div>
      </template>
    </BaseModal>
    <ConfirmModal
      :model-value="showConfirmModal"
      title="Supprimer la version"
      :message="confirmMessage"
      variant="danger"
      :loading="deleting"
      @update:model-value="showConfirmModal = $event"
      @cancel="onCancelDelete"
      @confirm="onConfirmDelete"
    />
  </div>
</template>
<script setup lang="ts">
import Icon from '@components/common/Icon.vue';
import UiBadge from '@components/ui/buttons/UiBadge.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiInput from '@components/ui/input/UiInput.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import ConfirmModal from '@components/ui/overlays/ConfirmModal.vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { ref, onMounted } from 'vue';

import { useToasts } from '@/composables/useToasts';

interface VersionEntry {
  id: string;
  version?: string;
  displayName?: string;
  dir: string;
  hasJre: boolean;
  modified: number;
  sizeKB: number;
}
interface VersionsResult {
  versions: VersionEntry[];
  root: string;
  selectedId?: string | null;
}
interface AvailableVersion {
  tag: string;
  version: string;
  coreUrl: string;
  jreUrl?: string | null;
  publishedAt?: string | null;
}

const versions = ref<VersionEntry[]>([]);
const selectedId = ref<string | undefined>('');
const loading = ref(false);
const installing = ref(false);
const deleting = ref(false);
const validating = ref(false);
const showUrl = ref(false);
const url = ref('');
const available = ref<AvailableVersion[]>([]);
const loadingAvail = ref(false);
const installingFrom = ref<string | null>(null);
const showConfirmModal = ref(false);
const confirmMessage = ref('');
const pendingDeleteId = ref<string | null>(null);

async function refresh() {
  loading.value = true;
  try {
    const res = await invoke<VersionsResult>('list_versions');
    versions.value = res.versions || [];
    selectedId.value = res.selectedId || undefined;
  } finally {
    loading.value = false;
  }
}

async function pickLocal() {
  if (installing.value) return;
  const sel = await open({ multiple: false, canCreateDirectories: false, directory: false });
  if (!sel || typeof sel !== 'string') return;
  installing.value = true;
  try {
    await invoke<string>('install_version_local', { path: sel });
    await refresh();
  } finally {
    installing.value = false;
  }
}

function promptUrl() {
  showUrl.value = true;
}

async function installFromUrl() {
  if (!url.value) return;
  installing.value = true;
  try {
    await invoke<string>('install_version_from_url', { url: url.value });
    showUrl.value = false;
    url.value = '';
    await refresh();
  } finally {
    installing.value = false;
  }
}

async function loadAvailable() {
  loadingAvail.value = true;
  try {
    available.value = await invoke<AvailableVersion[]>('list_available_versions');
  } catch (e) {
    useToasts().error('Échec du chargement des versions disponibles.');
    available.value = [];
  } finally {
    loadingAvail.value = false;
  }
}

async function installFromRelease(version: string) {
  installing.value = true;
  installingFrom.value = version;
  try {
    await invoke<string>('install_version_from_release', { version });
    await refresh();
    await loadAvailable();
  } finally {
    installing.value = false;
    installingFrom.value = null;
  }
}

function isInstalled(version: string): boolean {
  const id = 'v' + version.replace(/^v/i, '');
  return versions.value.some((v) => v.id.toLowerCase() === id.toLowerCase());
}

async function repairRelease(version: string) {
  installing.value = true;
  installingFrom.value = version;
  try {
    await invoke<string>('repair_version_from_release', { version });
    await refresh();
    await loadAvailable();
  } finally {
    installing.value = false;
    installingFrom.value = null;
  }
}

async function select(id: string) {
  await invoke<string>('select_version', { id });
  selectedId.value = id;
}

async function del(id: string) {
  if (deleting.value) return;
  pendingDeleteId.value = id;
  confirmMessage.value = `Supprimer la version ${id} ?`;
  showConfirmModal.value = true;
}

async function onConfirmDelete() {
  if (!pendingDeleteId.value) return;
  deleting.value = true;
  try {
    await invoke<string>('delete_version', { id: pendingDeleteId.value });
    if (selectedId.value === pendingDeleteId.value)
      await invoke<string>('select_version', { id: null });
    showConfirmModal.value = false;
    pendingDeleteId.value = null;
    await refresh();
    await loadAvailable();
  } finally {
    deleting.value = false;
  }
}

function onCancelDelete() {
  showConfirmModal.value = false;
  pendingDeleteId.value = null;
}

onMounted(async () => {
  await refresh();
  await loadAvailable();
});
</script>
