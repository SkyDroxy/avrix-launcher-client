<template>
  <div
    class="h-8 flex items-center select-none pl-2 pr-1 bg-neutral-900 border-b border-neutral-700"
  >
    <div class="flex items-center gap-2 flex-1 min-w-0" data-tauri-drag-region>
      <img v-if="iconSrc" :src="iconSrc" alt="icon" class="w-4 h-4 pointer-events-none" />
      <span class="text-xs tracking-wide opacity-80 truncate">Avrix Launcher</span>
      <span class="text-[11px] opacity-50 mx-2">|</span>
      <RouteBreadcrumb class="flex-1 min-w-0" />
    </div>
    <div class="flex items-center gap-1 pl-2">
      <!-- <UiButton
        :border="false"
        tooltip-placement="bottom"
        :tooltip="theme === 'dark' ? 'Mode clair' : 'Mode sombre'"
        size="sm"
        variant="ghost"
        square
        class="w-8 h-6"
        :title="theme === 'dark' ? 'Mode clair' : 'Mode sombre'"
        :icon="theme === 'dark' ? 'mingcute:sun-fill' : 'mingcute:moon-stars-fill'"
        @click="toggleTheme"
      /> -->
      <UiButton
        :border="false"
        tooltip-placement="bottom"
        tooltip="Paramètres"
        size="sm"
        variant="ghost"
        square
        class="w-8 h-6"
        :title="'Paramètres'"
        icon="mingcute:settings-5-fill"
        @click="openSettings()"
      />
    </div>
    <div class="flex items-center gap-1 pl-2">
      <UiButton
        :border="false"
        tooltip-placement="bottom"
        tooltip="Rechercher des mises à jour"
        size="sm"
        variant="ghost"
        square
        class="w-8 h-6"
        :title="'Rechercher des mises à jour'"
        icon="mingcute:refresh-2-fill"
        @click="onCheckUpdates"
      />
      <UiButton
        :border="false"
        tooltip-placement="bottom"
        tooltip="Minimiser"
        size="sm"
        variant="ghost"
        square
        class="w-8 h-6"
        :title="'Minimiser'"
        icon="mingcute:minimize-fill"
        @click="minimize"
      />
      <UiButton
        :border="false"
        tooltip-placement="bottom"
        tooltip="Agrandir"
        size="sm"
        variant="ghost"
        square
        class="w-8 h-6"
        :title="isMaximized ? 'Restaurer' : 'Agrandir'"
        :icon="isMaximized ? 'mingcute:fullscreen-exit-2-fill' : 'mingcute:fullscreen-2-fill'"
        @click="toggleMax"
      />
      <UiButton
        :border="false"
        tooltip-placement="bottom"
        tooltip="Fermer"
        size="sm"
        variant="ghost"
        square
        class="w-8 h-6"
        :title="'Fermer'"
        icon="mingcute:close-square-fill"
        @click="closeWindow"
      />
    </div>
    <SettingsModal v-model="settingsOpen" />
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';
import iconImage from '@assets/AvrixLauncher.ico';
import RouteBreadcrumb from '@components/navigation/RouteBreadcrumb.vue';
import SettingsModal from '@components/settings/SettingsModal.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import { useSettingsModal } from '@composables/useSettingsModal';
import { useUpdater } from '@composables/useUpdater';
import { getCurrentWindow } from '@tauri-apps/api/window';

const iconSrc = ref(iconImage);
function detectInitialTheme(): 'dark' | 'light' {
  try {
    const saved = (localStorage.getItem('avrix-theme') || '').toLowerCase();
    if (saved === 'dark' || saved === 'light') return saved as 'dark' | 'light';
  } catch {}
  const prefersDark =
    typeof window !== 'undefined' &&
    window.matchMedia &&
    window.matchMedia('(prefers-color-scheme: dark)').matches;
  return prefersDark ? 'dark' : 'light';
}
const theme = ref<'dark' | 'light'>(detectInitialTheme());

function applyTheme(next: 'dark' | 'light') {
  const root = document.documentElement;
  root.setAttribute('data-theme', next);
  try {
    localStorage.setItem('avrix-theme', next);
  } catch {}
}
function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark';
  applyTheme(theme.value);
}
async function minimize() {
  try {
    await getCurrentWindow().minimize();
  } catch (e) {
    console.error('Echec minimize', e);
  }
}
async function closeWindow() {
  try {
    await getCurrentWindow().close();
  } catch (e) {
    console.error('Echec close', e);
  }
}
const isMaximized = ref(false);
let unlisten: (() => void) | null = null;
const { isOpen: settingsOpen, open: openSettings } = useSettingsModal();
const { checkNow } = useUpdater();
async function onCheckUpdates() {
  await checkNow();
}
async function updateState() {
  try {
    const win = getCurrentWindow();
    isMaximized.value = await win.isMaximized();
  } catch (e) {}
}
async function toggleMax() {
  const win = getCurrentWindow();
  try {
    if (await win.isMaximized()) {
      await win.unmaximize();
    } else {
      await win.maximize();
    }
    await updateState();
  } catch (e) {
    console.error('Echec maximize', e);
  }
}
onMounted(async () => {
  await updateState();
  try {
    const win = getCurrentWindow();
    unlisten = await win.listen('tauri://resize', () => updateState());
  } catch (e) {}
  applyTheme(theme.value);
});
onBeforeUnmount(() => {
  if (unlisten) {
    try {
      unlisten();
    } catch {}
  }
});
</script>
<style scoped>
[data-tauri-drag-region] {
  -webkit-app-region: drag;
}
button {
  -webkit-app-region: no-drag;
}
</style>
