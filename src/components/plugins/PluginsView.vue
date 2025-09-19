<template>
  <div class="flex flex-col h-full min-h-0 gap-4">
    <!-- Action bar: Install from file | Scan Workshop | Install from URL -->
    <div class="flex items-center gap-2">
      <UiButton
        variant="primary"
        size="sm"
        class="px-3"
        :disabled="scanningWorkshop || installing"
        @click="scanWorkshop"
      >
        <div class="flex items-center gap-2">
          <Icon name="mingcute:radar-fill" :width="16" />
          <span>Scanner Workshop</span>
        </div>
      </UiButton>
      <UiButton
        variant="secondary"
        size="sm"
        class="px-3"
        :disabled="installing || validating"
        @click="pickLocalJar"
      >
        <div class="flex items-center gap-2">
          <Icon name="mingcute:folder-open-fill" :width="16" />
          <span>Installer depuis un fichier</span>
        </div>
      </UiButton>

      <UiButton
        variant="secondary"
        size="sm"
        class="px-3"
        :disabled="validating || installing"
        @click="promptUrlInstall"
      >
        <div class="flex items-center gap-2">
          <Icon name="mingcute:link-2-fill" :width="16" />
          <span>Installer depuis une URL</span>
        </div>
      </UiButton>
    </div>
    <!-- Header: Rescan + Search + Sort (left, spans main column) | Logs (right, above Env panel) -->
    <div class="grid grid-cols-12 gap-4 items-center">
      <!-- Left side: align to plugin list width (col-span-9) -->
      <div class="col-span-9">
        <div class="flex items-center gap-3 flex-nowrap">
          <UiButton size="sm" variant="ghost" class="px-3" :title="'Rescanner'" @click="refresh">
            <div class="flex items-center gap-2">
              <Icon name="mingcute:refresh-3-fill" :width="16" />
              <span>Rescanner</span>
            </div>
          </UiButton>
          <UiSearchInput
            v-model="search"
            placeholder="Rechercher un plugin…"
            class="min-w-0 flex-1"
            :dense="false"
          />
          <div class="flex items-center gap-2 shrink-0">
            <span class="text-[12px] opacity-70">Trier par</span>
            <UiSelect v-model="sortKey" :options="sortOptions" />
            <UiButton
              size="sm"
              variant="ghost"
              :title="sortDir === 'asc' ? 'Ascendant' : 'Descendant'"
              @click="toggleDir"
            >
              <div class="flex items-center gap-2">
                <Icon
                  :name="
                    sortDir === 'asc'
                      ? 'mingcute:sort-descending-fill'
                      : 'mingcute:sort-ascending-fill'
                  "
                  :width="16"
                />
                <span>{{ sortDir === 'asc' ? 'Ascendant' : 'Descendant' }}</span>
              </div>
            </UiButton>
          </div>
        </div>
      </div>
      <!-- Right side: Logs aligned above Environment panel (col-span-3) -->
      <div class="col-span-3 flex justify-end">
        <UiButton
          v-if="isDev"
          size="sm"
          variant="ghost"
          class="px-3"
          :title="'Logs'"
          @click="showPluginLogsModal = true"
        >
          <div class="flex items-center gap-2">
            <Icon name="mingcute:book-4-fill" :width="16" />
            <span>Logs</span>
          </div>
        </UiButton>
      </div>
    </div>

    <div class="flex-1 overflow-auto min-h-0 grid grid-cols-12 gap-4">
      <div class="col-span-9 min-h-0">
        <div
          v-if="loading"
          class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5"
        >
          <div
            v-for="i in 8"
            :key="i"
            class="h-28 rounded-md bg-neutral-800/40 border border-neutral-700/60 relative overflow-hidden"
          >
            <div
              class="absolute inset-0 animate-pulse bg-[linear-gradient(110deg,#262626_8%,#2e2e2e_18%,#262626_33%)] bg-[length:200%_100%]"
            ></div>
          </div>
        </div>
        <div v-else-if="!filteredSorted.length" class="text-xs opacity-60 px-1 py-2">
          Aucun plugin.
        </div>
        <div v-else class="flex flex-col gap-2">
          <div v-if="coreChildren.length" class="flex flex-col">
            <div class="px-3 py-2 text-[12px] opacity-70">Avrix Core</div>
            <template v-for="(p, idx) in coreChildren" :key="(p.id || p.name) + ':' + p.name">
              <PluginListItem :plugin="p" @delete="confirmDelete" />
              <hr
                v-if="idx < coreChildren.length - 1"
                class="my-1 border-t border-neutral-700/50 mx-2"
              />
            </template>
          </div>
          <div class="flex flex-col">
            <template v-for="(p, idx) in externalPlugins" :key="(p.id || p.name) + ':' + p.name">
              <PluginListItem :plugin="p" @delete="confirmDelete" />
              <hr
                v-if="idx < externalPlugins.length - 1"
                class="my-1 border-t border-neutral-700/50 mx-2"
              />
            </template>
          </div>
          <div v-if="internalStandalone.length" class="">
            <details class="rounded-lg border-neutral-600 bg-neutral-800/40" :open="showInternal">
              <summary
                class="cursor-pointer select-none px-3 py-2 text-[12px] flex items-center gap-2"
              >
                <Icon name="mingcute:box-3-fill" :width="16" class="opacity-80" />
                <span class="font-semibold"
                  >Plugins internes ({{ internalStandalone.length }})</span
                >
              </summary>
              <div class="p-3 flex flex-col">
                <template
                  v-for="(p, idx) in internalStandalone"
                  :key="(p.id || p.name) + ':' + p.name"
                >
                  <PluginListItem :plugin="p" @delete="confirmDelete" />
                  <hr
                    v-if="idx < internalStandalone.length - 1"
                    class="my-1 border-t border-neutral-700/50 mx-2"
                  />
                </template>
              </div>
            </details>
          </div>
        </div>
      </div>
      <div class="col-span-3">
        <PluginsEnvPanel v-model="envFilter" :items="envOptions" />
      </div>
    </div>
    <PluginUrlModal
      :model-value="showUrlModal"
      :url="urlInput"
      :loading="validating"
      @update:model-value="showUrlModal = $event"
      @update:url="urlInput = $event"
      @validate="validateUrl"
    />
    <PluginValidationModal
      :model-value="showValidationModal"
      :validation="validation"
      :loading="installing"
      @update:model-value="showValidationModal = $event"
      @cancel="
        () => {
          showValidationModal = false;
          validation = null;
          localPendingPath = null;
          urlInput = '';
        }
      "
      @confirm="installFromPreview"
    />
    <WorkshopSelectionModal
      :model-value="showWorkshopModal"
      :items="workshopFound"
      :loading="installing"
      @update:model-value="showWorkshopModal = $event"
      @install="onWorkshopInstall"
    />
    <WorkshopScanModal
      :model-value="showScanModal"
      @update:model-value="showScanModal = $event"
      @cancel="cancelScanModal"
    />
    <PluginLogsModal
      :model-value="showPluginLogsModal"
      @update:model-value="showPluginLogsModal = $event"
    />
    <!-- Confirm delete modal -->
    <ConfirmModal
      :model-value="showConfirmModal"
      title="Supprimer le plugin"
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
import { Environment, EnvironmentLabels } from '@common/enums/Environment';
import PluginListItem from '@components/plugins/list/PluginListItem.vue';
import PluginLogsModal from '@components/plugins/modals/PluginLogsModal.vue';
import PluginUrlModal from '@components/plugins/modals/PluginUrlModal.vue';
import PluginValidationModal from '@components/plugins/modals/PluginValidationModal.vue';
import WorkshopScanModal from '@components/plugins/modals/WorkshopScanModal.vue';
import WorkshopSelectionModal from '@components/plugins/modals/WorkshopSelectionModal.vue';
import PluginsEnvPanel from '@components/plugins/sidebar/PluginsEnvPanel.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiSelect from '@components/ui/data/UiSelect.vue';
import UiSearchInput from '@components/ui/input/UiSearchInput.vue';
import ConfirmModal from '@components/ui/overlays/ConfirmModal.vue';
import { useLogs } from '@composables/useLogs';
import { useToasts } from '@composables/useToasts';
import { effectiveDisplayName, keyFor, sortedDeps, slugFromName } from '@helpers/pluginFormat';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { ref, onMounted, computed } from 'vue';

import Icon from '@/components/common/Icon.vue';

import type { PluginInfo, ScanPluginsResult } from '@interfaces/plugins';

const plugins = ref<PluginInfo[]>([]);
const dir = ref('');
const loading = ref(false);
const search = ref('');
const sortKey = ref<'name' | 'modified' | 'sizeKB'>('name');
const sortOptions = [
  { value: 'name', label: 'Nom' },
  { value: 'modified', label: 'Date' },
  { value: 'sizeKB', label: 'Taille' },
];
const sortDir = ref<'asc' | 'desc'>('asc');
const envFilter = ref<'all' | Environment>('all');
const envOptions: Array<{ value: 'all' | Environment; label: string }> = [
  { value: 'all', label: 'Tous' },
  { value: Environment.CLIENT, label: EnvironmentLabels[Environment.CLIENT] },
  { value: Environment.SERVER, label: EnvironmentLabels[Environment.SERVER] },
  { value: Environment.BOTH, label: EnvironmentLabels[Environment.BOTH] },
];
const highlighted = ref<Set<string>>(new Set());
const cardRefs = ref<Record<string, HTMLElement | null>>({});
const { success: toastSuccess, error: toastError, info: toastInfo } = useToasts();
const workshopFound = ref<string[]>([]);
const showUrlModal = ref(false);
const urlInput = ref('');
const installing = ref(false);
const showWorkshopModal = ref(false);
const scanningWorkshop = ref(false);
const showScanModal = ref(false);
const workshopSelection = ref<Record<string, boolean>>({});
interface ValidationMetadata {
  valid: boolean;
  name?: string;
  version?: string;
  environment?: string;
  size: number;
  sha256?: string;
  message: string;
}
interface InstallFromUrlResult {
  message: string;
  size: number;
  sha256: string;
  name?: string;
  version?: string;
  environment?: string;
}
const validation = ref<ValidationMetadata | null>(null);
const showValidationModal = ref(false);
const localPendingPath = ref<string | null>(null);
const validating = ref(false);
const { addLog } = useLogs();
const isDev = import.meta.env.DEV;
const showPluginLogsModal = ref(false);
const showConfirmModal = ref(false);
const pendingDelete = ref<PluginInfo | null>(null);
const deleting = ref(false);

function onUpdateSortKey(v: string) {
  if (v === 'name' || v === 'modified' || v === 'sizeKB') sortKey.value = v;
}
function onUpdateSortDir(v: 'asc' | 'desc') {
  sortDir.value = v;
}
function onUpdateEnvFilter(v: any) {
  envFilter.value = v;
}
function onUpdateSearch(v: string) {
  search.value = v;
}

async function refresh() {
  loading.value = true;
  try {
    const res = await invoke<ScanPluginsResult>('scan_plugins');
    if (res && typeof res === 'object') {
      plugins.value = res.plugins ?? [];
      dir.value = res.dir ?? '';
    } else {
      plugins.value = [];
    }
  } catch (e) {
    console.error('scan_plugins error', e);
  } finally {
    loading.value = false;
  }
}

const filteredSorted = computed(() => {
  let list = plugins.value;
  if (envFilter.value !== 'all') {
    list = list.filter((p) => {
      const env = (p.environment || Environment.BOTH).toLowerCase();
      if (envFilter.value === Environment.BOTH) return env === Environment.BOTH;
      return env === envFilter.value;
    });
  }
  const q = search.value.trim().toLowerCase();
  if (q) list = list.filter((p) => p.name.toLowerCase().includes(q));
  const dirMul = sortDir.value === 'asc' ? 1 : -1;
  return [...list].sort((a, b) => {
    const k = sortKey.value;
    let av: number | string = a[k];
    let bv: number | string = b[k];
    if (k === 'name') {
      return av.toString().localeCompare(bv.toString()) * dirMul;
    }
    return (Number(av) - Number(bv)) * dirMul;
  });
});

const coreChildren = computed(() =>
  filteredSorted.value.filter((p) => p.internal && p.parentId === 'avrix-core')
);
const coreParent = computed<PluginInfo | null>(() => {
  const existing = filteredSorted.value.find((p) => p.id === 'avrix-core');
  if (existing) return existing;
  const kids = coreChildren.value;
  if (!kids.length) return null;
  const version = kids.find((k) => k.id === 'avrix-loader')?.version || kids[0].version;
  const modified = kids.reduce((m, k) => Math.max(m, k.modified || 0), 0);
  const sizeKB = kids.reduce((s, k) => s + (k.sizeKB || 0), 0);
  const environment = (() => {
    const all = Array.from(new Set(kids.map((k) => k.environment).filter(Boolean)));
    return all.length === 1 ? all[0] : undefined;
  })();
  const author = kids.find((k) => k.author)?.author;
  const parent: PluginInfo = {
    id: 'avrix-core',
    name: 'avrix-core',
    displayName: 'Avrix Core',
    version,
    internal: true,
    description: 'Module principal Avrix regroupant les composants internes.',
    sizeKB,
    modified,
    environment,
    author,
  };
  return parent;
});
const filteredSortedNoCore = computed(() =>
  filteredSorted.value.filter((p) => {
    if (p.id === 'avrix-core') return false;
    if (p.internal && p.parentId === 'avrix-core') return false;
    return true;
  })
);
const internalStandalone = computed(() => filteredSortedNoCore.value.filter((p) => p.internal));
const externalPlugins = computed(() => filteredSortedNoCore.value.filter((p) => !p.internal));
const showInternal = ref(false);

function toggleDir() {
  sortDir.value = sortDir.value === 'asc' ? 'desc' : 'asc';
}

function displayName(p: PluginInfo) {
  return effectiveDisplayName(p);
}
function registerCardRef(p: PluginInfo, el: any) {
  if (!el || !(el instanceof HTMLElement)) return;
  const k = keyFor(p);
  cardRefs.value[k] = el;
}
function findPluginByIdOrName(dep: string) {
  const lower = dep.toLowerCase();
  return plugins.value.find(
    (pl) =>
      (pl.id && pl.id.toLowerCase() === lower) ||
      pl.name.toLowerCase() === lower ||
      slugFromName(pl.name) === lower
  );
}
function highlightDependency(dep: string, scroll = true) {
  const target = findPluginByIdOrName(dep);
  if (!target) return;
  const k = keyFor(target);
  highlighted.value.add(k);
  highlighted.value = new Set(highlighted.value);
  if (scroll) {
    requestAnimationFrame(() => {
      const el = cardRefs.value[k];
      if (el) {
        el.scrollIntoView({ behavior: 'smooth', block: 'center', inline: 'center' });
        el.classList.add('animate-pulse-once');
        setTimeout(() => el.classList.remove('animate-pulse-once'), 1400);
      }
    });
  }
}
function highlightAllDeps(p: PluginInfo) {
  highlighted.value = new Set();
  const deps = sortedDeps(p).map(([dep]) => dep);
  let firstScrolled = false;
  deps.forEach((dep) => {
    highlightDependency(dep, !firstScrolled);
    if (!firstScrolled) firstScrolled = true;
  });
  setTimeout(() => {
    highlighted.value = new Set();
  }, 2000);
}
function isHighlighted(p: PluginInfo) {
  return highlighted.value.has(keyFor(p));
}

async function pickLocalJar() {
  if (installing.value || validating.value) return;
  try {
    addLog({ level: 'info', source: 'ui', message: 'Sélection fichier local…' });
    const sel = await open({ multiple: false, filters: [{ name: 'JAR', extensions: ['jar'] }] });
    if (!sel) return;
    const path = Array.isArray(sel) ? sel[0] : sel;
    validating.value = true;
    addLog({ level: 'info', source: 'validation', message: `Validation locale: ${path}` });
    validation.value = await invoke<ValidationMetadata>('validate_plugin_local', { path });
    localPendingPath.value = path;
    if (!validation.value.valid) {
      toastError('Invalide: ' + validation.value.message);
      addLog({
        level: 'error',
        source: 'validation',
        message: `Echec validation locale: ${validation.value.message}`,
      });
      validation.value = null;
      localPendingPath.value = null;
      return;
    }
    addLog({
      level: 'success',
      source: 'validation',
      message: `Validation locale OK (${validation.value.name || '-'})`,
    });
    showValidationModal.value = true;
  } catch (e: any) {
    toastError('Erreur validation: ' + String(e));
    addLog({
      level: 'error',
      source: 'validation',
      message: `Exception validation locale: ${String(e)}`,
    });
  } finally {
    validating.value = false;
  }
}
function promptUrlInstall() {
  urlInput.value = '';
  showUrlModal.value = true;
  validation.value = null;
}
async function validateUrl() {
  if (!urlInput.value.trim()) {
    toastError('URL vide');
    return;
  }
  if (validating.value) return;
  validating.value = true;
  try {
    addLog({
      level: 'info',
      source: 'validation',
      message: `Validation URL: ${urlInput.value.trim()}`,
    });
    validation.value = await invoke<ValidationMetadata>('validate_plugin_from_url', {
      url: urlInput.value.trim(),
    });
    if (!validation.value.valid) {
      toastError('Invalide: ' + validation.value.message);
      addLog({
        level: 'error',
        source: 'validation',
        message: `Echec validation URL: ${validation.value.message}`,
      });
      return;
    }
    showUrlModal.value = false;
    localPendingPath.value = null;
    showValidationModal.value = true;
    addLog({
      level: 'success',
      source: 'validation',
      message: `Validation URL OK (${validation.value.name || '-'})`,
    });
  } catch (e: any) {
    toastError('Erreur validation URL: ' + String(e));
    addLog({
      level: 'error',
      source: 'validation',
      message: `Exception validation URL: ${String(e)}`,
    });
  } finally {
    validating.value = false;
  }
}
async function installFromPreview() {
  if (!validation.value || !validation.value.valid) {
    toastError('Pas de validation valide');
    return;
  }
  if (installing.value) return;
  installing.value = true;
  try {
    addLog({ level: 'info', source: 'install', message: 'Installation depuis prévisualisation…' });
    if (localPendingPath.value) {
      const res = await invoke<string>('install_plugin_local', { path: localPendingPath.value });
      toastSuccess(res, { meta: metaFromValidation() });
      addLog({ level: 'success', source: 'install', message: `Install locale OK: ${res}` });
    } else if (urlInput.value) {
      const res = await invoke<InstallFromUrlResult>('install_plugin_from_url', {
        url: urlInput.value.trim(),
      });
      toastSuccess(res.message, {
        meta: {
          size: humanSize(res.size),
          sha256: shortHash(res.sha256),
          name: res.name || '-',
          version: res.version || '-',
          env: res.environment || '-',
        },
      });
      addLog({ level: 'success', source: 'install', message: `Install URL OK: ${res.message}` });
    } else {
      toastError('Aucune source');
      return;
    }
    showValidationModal.value = false;
    validation.value = null;
    localPendingPath.value = null;
    urlInput.value = '';
    await refresh();
  } catch (e: any) {
    toastError('Erreur installation: ' + String(e));
    addLog({ level: 'error', source: 'install', message: `Exception install: ${String(e)}` });
  } finally {
    installing.value = false;
  }
}
function metaFromValidation() {
  if (!validation.value) return {} as Record<string, string>;
  return {
    name: validation.value.name || '-',
    version: validation.value.version || '-',
    env: validation.value.environment || '-',
    size: humanSize(validation.value.size),
    sha256: validation.value.sha256 ? shortHash(validation.value.sha256) : '-',
  };
}
function humanSize(size: number) {
  if (size < 1024) return size + ' B';
  if (size < 1024 * 1024) return (size / 1024).toFixed(1) + ' KB';
  return (size / 1024 / 1024).toFixed(2) + ' MB';
}
function shortHash(h: string) {
  return h.slice(0, 10) + '…';
}
async function scanWorkshop() {
  if (scanningWorkshop.value || installing.value) return;
  scanningWorkshop.value = true;
  showScanModal.value = true;
  workshopFound.value = [];
  try {
    toastInfo('Scan Workshop...');
    addLog({
      level: 'info',
      source: 'scan',
      message: 'Scan workshop démarré',
    });
    const res = await invoke<{ found: string[] }>('scan_workshop');
    workshopFound.value = res.found || [];
    workshopSelection.value = Object.fromEntries(workshopFound.value.map((p) => [p, true]));
    if (!workshopFound.value.length) {
      toastInfo('Aucun plugin Workshop trouvé.');
      addLog({ level: 'warn', source: 'scan', message: 'Aucun plugin workshop trouvé' });
    } else {
      showWorkshopModal.value = true;
      addLog({
        level: 'success',
        source: 'scan',
        message: `Scan workshop OK (${workshopFound.value.length} trouvé(s))`,
      });
    }
  } catch (e: any) {
    toastError('Erreur scan workshop: ' + String(e));
    addLog({ level: 'error', source: 'scan', message: `Erreur scan: ${String(e)}` });
  } finally {
    scanningWorkshop.value = false;
    showScanModal.value = false;
  }
}
function cancelScanModal() {
  if (showScanModal.value) {
    showScanModal.value = false;
    toastInfo('Scan en arrière-plan...');
  }
}
async function installWorkshopSelected(all = false) {
  const targets = all
    ? workshopFound.value
    : workshopFound.value.filter((p) => workshopSelection.value[p]);
  if (!targets.length) {
    toastError('Aucune sélection');
    return;
  }
  if (installing.value) return;
  installing.value = true;
  let ok = 0;
  try {
    for (const p of targets) {
      try {
        await invoke<string>('install_plugin_local', { path: p });
        ok++;
      } catch (_) {
        /* skip */
      }
    }
    toastSuccess(`Workshop: ${ok}/${targets.length} installés`);
    addLog({
      level: 'success',
      source: 'install',
      message: `Install workshop: ${ok}/${targets.length}`,
    });
    showWorkshopModal.value = false;
    await refresh();
  } finally {
    installing.value = false;
  }
}
function onWorkshopInstall(payload: { all: boolean; selected: string[] }) {
  if (payload.all) {
    installWorkshopSelected(true);
  } else {
    workshopSelection.value = Object.fromEntries(
      workshopFound.value.map((p) => [p, payload.selected.includes(p)])
    );
    installWorkshopSelected(false);
  }
}

let scanListenersAttached = false;
onMounted(() => {
  // @ts-ignore
  if (!scanListenersAttached && window.__TAURI__?.event?.listen) {
    const tauriEvent = (window as any).__TAURI__?.event;
    if (tauriEvent?.listen) {
      scanListenersAttached = true;
      tauriEvent.listen('plugin-install-log', (e: any) => {
        if (e?.payload) {
          const msg = String(e.payload);
          addLog({ level: classify(msg), source: 'backend', message: msg });
          toastFrom(msg);
        }
      });
      tauriEvent.listen('plugin-scan-log', (e: any) => {
        if (e?.payload) {
          const msg = String(e.payload);
          addLog({ level: classify(msg), source: 'scan', message: msg });
        }
      });
    }
  }
  refresh();
});

function classify(msg: string): 'info' | 'success' | 'error' | 'warn' {
  const m = msg.toLowerCase();
  if (m.includes('erreur') || m.includes('failed') || m.includes('invalide')) return 'error';
  if (m.includes('terminé') || m.includes('termine') || m.includes('installé') || m.includes('ok'))
    return 'success';
  if (m.includes('warn') || m.includes('aucun')) return 'warn';
  return 'info';
}
function toastFrom(msg: string) {
  const lvl = classify(msg);
  switch (lvl) {
    case 'error':
      toastError(msg);
      break;
    case 'success':
      toastSuccess(msg);
      break;
    case 'warn':
      toastInfo(msg);
      break;
    default:
      /* info: optional subtle -> skip to reduce noise */ break;
  }
}
function confirmDelete(p: PluginInfo) {
  pendingDelete.value = p;
  showConfirmModal.value = true;
}

const confirmMessage = computed(() =>
  pendingDelete.value ? `Supprimer le plugin "${displayName(pendingDelete.value)}" ?` : ''
);

async function onConfirmDelete() {
  if (!pendingDelete.value) {
    showConfirmModal.value = false;
    return;
  }
  try {
    deleting.value = true;
    const msg = await invoke<string>('delete_plugin', { name: pendingDelete.value.name });
    toastSuccess(msg);
    addLog({ level: 'info', source: 'install', message: `Suppression: ${msg}` });
    showConfirmModal.value = false;
    pendingDelete.value = null;
    await refresh();
  } catch (e: any) {
    toastError('Erreur suppression: ' + String(e));
    addLog({ level: 'error', source: 'install', message: `Erreur suppression: ${String(e)}` });
  } finally {
    deleting.value = false;
  }
}

function onCancelDelete() {
  showConfirmModal.value = false;
  pendingDelete.value = null;
}
</script>
<style scoped>
/* line-clamp without adding extra dependency */
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
@keyframes highlightPulse {
  0% {
    box-shadow: 0 0 0 0 rgba(129, 140, 248, 0.6);
  }
  70% {
    box-shadow: 0 0 0 4px rgba(129, 140, 248, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(129, 140, 248, 0);
  }
}
.animate-highlight-pulse {
  animation: highlightPulse 1.2s ease-out;
}
</style>
