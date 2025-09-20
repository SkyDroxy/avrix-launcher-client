<template>
  <div class="h-screen flex flex-col bg-neutral-800 text-neutral-200">
    <TitleBar />
    <div class="flex flex-1 min-h-0">
      <Sidebar />
      <div class="flex-1 flex flex-col min-w-0">
        <main class="flex-1 min-h-0 flex flex-col px-6 pt-5 pb-8 overflow-y-auto custom-scrollbar">
          <RouterView v-slot="{ Component }">
            <component :is="Component" @launched="onLaunched" />
          </RouterView>
        </main>
      </div>
    </div>
    <ToastContainer />
    <UpdateModal v-model="showUpdate" />
  </div>
</template>

<script setup lang="ts">
import Sidebar from '@components/layout/sidebar/Sidebar.vue';
import TitleBar from '@components/layout/TitleBar.vue';
import ToastContainer from '@components/ui/feedback/ToastContainer.vue';
import UpdateModal from '@components/ui/overlays/UpdateModal.vue';
import { useSettings } from '@composables/useSettings';
import { useUpdater } from '@composables/useUpdater';
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref, watch } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const { checkNow, autoCheckOnStartup, status } = useUpdater();
const { load: loadSettings, autoWorkshopScanOnStartup, autoWorkshopInstall } = useSettings();
const showUpdate = ref(false);
onMounted(async () => {
  await loadSettings();
  if (autoCheckOnStartup.value && !import.meta.env.DEV) {
    const upd = await checkNow({ silent: true });
    if (upd) showUpdate.value = true;
  }
  // Auto scan Workshop on startup
  if (autoWorkshopScanOnStartup.value) {
    try {
      const res = (await invoke('scan_workshop')) as { found: string[] };
      const found = res?.found || [];
      if (Array.isArray(found) && found.length && autoWorkshopInstall.value) {
        for (const p of found) {
          try {
            await invoke<string>('install_plugin_local', { path: p });
          } catch (_) {
            /* continue */
          }
        }
      }
    } catch (_) {
      /* ignore */
    }
  }
});
watch(status, (s) => {
  if (s === 'available') showUpdate.value = true;
});
function onLaunched() {
  router.push({ name: 'logs' });
}
</script>
