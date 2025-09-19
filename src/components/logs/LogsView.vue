<template>
  <div class="flex flex-col h-full min-h-0">
    <div
      class="h-8 flex items-center px-2 gap-2 border-b border-neutral-700 bg-neutral-800/70 text-[11px] select-none"
    >
      <span class="opacity-70">Logs</span>
      <div class="flex items-center gap-2 ml-2">
        <FiltersBar
          :level-order="levelOrder"
          :level-enabled="levelEnabled"
          :auto-follow="autoFollow"
          :shown="shownFiltered.length"
          :total="shownLogs.length"
          @toggle-level="toggleLevel"
          @toggle-follow="toggleFollow"
          @copy="copyAll"
          @export="exportAll"
        />
      </div>
      <div class="ml-auto flex items-center gap-2">
        <div class="w-48">
          <UiSearchInput v-model="search" size="md" width-class="w-48" />
        </div>
        <UiButton size="xs" variant="ghost" @click="clearAll" title="Vider les logs">
          <Icon name="mingcute:delete-2-fill" :width="16" />
        </UiButton>
        <span class="opacity-60">{{ status }}</span>
      </div>
    </div>
    <div ref="scrollEl" class="flex-1 overflow-auto p-4 text-sm">
      <LogEntryRow v-for="(l, i) in shownFiltered" :key="i" :raw="l" />
      <div ref="logEnd" />
    </div>
  </div>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiSearchInput from '@components/ui/input/UiSearchInput.vue';
import { listen } from '@tauri-apps/api/event';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { ref, reactive, onMounted, onBeforeUnmount, watch, computed, nextTick } from 'vue';

import Icon from '@/components/common/Icon.vue';

import FiltersBar from './FiltersBar.vue';
import LogEntryRow from './LogEntryRow.vue';

import type { LogLevel } from '@interfaces/logs';

const logs = ref<string[]>([]);
const status = ref<'running' | 'finished'>('running');
const autoFollow = ref<boolean>(true);
const scrollEl = ref<HTMLDivElement | null>(null);
const MAX_LOGS = 10000;
const programmaticScroll = ref(false);

const levelOrder: LogLevel[] = ['ERROR', 'WARN', 'INFO', 'DEBUG'];
const levelEnabled = reactive<Record<LogLevel, boolean>>({
  ERROR: true,
  WARN: true,
  INFO: true,
  DEBUG: true,
});

const shownLogs = computed<string[]>(() =>
  logs.value.filter((l) => levelEnabled[extractLevel(l)] !== false)
);

const search = ref('');
const shownFiltered = computed<string[]>(() => {
  if (!search.value.trim()) return shownLogs.value;
  const q = search.value.toLowerCase();
  return shownLogs.value.filter((l) => l.toLowerCase().includes(q));
});

onMounted(async () => {
  await listen<string>('launch-log', (e) => {
    const msg = e.payload;
    logs.value.push(msg);
    if (logs.value.length > MAX_LOGS) {
      logs.value.splice(0, logs.value.length - MAX_LOGS);
    }
  });
  await listen<number>('launch-exit', (e) => {
    logs.value.push('[Process terminé] code=' + e.payload);
    status.value = 'finished';
  });
});

function scrollToBottom(force = false) {
  nextTick(() => {
    const el = scrollEl.value;
    if (!el) return;
    if (!force && !autoFollow.value) return;
    programmaticScroll.value = true;
    requestAnimationFrame(() => {
      el.scrollTop = el.scrollHeight;
      requestAnimationFrame(() => {
        programmaticScroll.value = false;
      });
    });
  });
}

function handleScroll() {
  if (programmaticScroll.value) return;
  const el = scrollEl.value;
  if (!el) return;
  const delta = el.scrollHeight - el.scrollTop - el.clientHeight;
  const atBottom = delta <= 4;
  if (atBottom) {
    autoFollow.value = true;
  } else if (autoFollow.value) {
    if (delta > 16) autoFollow.value = false;
  }
}

onMounted(() => {
  const el = scrollEl.value;
  if (el) el.addEventListener('scroll', handleScroll, { passive: true });
});

onBeforeUnmount(() => {
  const el = scrollEl.value;
  if (el) el.removeEventListener('scroll', handleScroll as any);
});

watch(
  () => logs.value.length,
  () => scrollToBottom()
);

function toggleFollow() {
  autoFollow.value = !autoFollow.value;
  if (autoFollow.value) scrollToBottom(true);
}

function extractLevel(line: string): LogLevel {
  if (/\bERROR\b/.test(line)) return 'ERROR';
  if (/\bWARN\b/.test(line)) return 'WARN';
  if (/\bDEBUG\b/.test(line)) return 'DEBUG';
  return 'INFO';
}

function toggleLevel(lvl: LogLevel) {
  levelEnabled[lvl] = !levelEnabled[lvl];
}

async function copyAll() {
  try {
    await writeText(shownLogs.value.join('\n'));
  } catch (_e) {
    console.error('Echec copie logs', _e);
  }
}

async function exportAll() {
  try {
    const filePath = await save({
      filters: [{ name: 'Log', extensions: ['log', 'txt'] }],
      defaultPath: 'avrix.log',
    });
    if (filePath) {
      await writeTextFile(filePath, shownLogs.value.join('\n'));
    }
  } catch (_e) {
    console.error('Echec export logs', _e);
  }
}

function clearAll() {
  logs.value = [];
}
</script>
