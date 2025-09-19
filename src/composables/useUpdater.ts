import { ref, watch } from 'vue';
import { check, type Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { openUrl } from '@tauri-apps/plugin-opener';
import { useToasts } from './useToasts';
import { useSettings } from './useSettings';

export type UpdateStatus =
  | 'idle'
  | 'checking'
  | 'available'
  | 'not-available'
  | 'downloading'
  | 'downloaded'
  | 'installing'
  | 'error';

const status = ref<UpdateStatus>('idle');
const errorMsg = ref<string | null>(null);
const available = ref<Update | null>(null);
const progress = ref<{ downloaded: number; total: number }>({ downloaded: 0, total: 0 });
const lastCheckAt = ref<number | null>(null);

const settings = useSettings();
const autoCheckOnStartup = ref<boolean>(true);
// initialize from settings when available
(async () => {
  try {
    await settings.load();
    if (typeof settings.autoCheckUpdates.value === 'boolean') {
      autoCheckOnStartup.value = settings.autoCheckUpdates.value;
    }
  } catch {}
})();
// mirror changes back to settings
watch(autoCheckOnStartup, (v) => {
  settings.autoCheckUpdates.value = v;
  settings.save();
});

export function useUpdater() {
  const { info: toastInfo, error: toastError, success: toastSuccess } = useToasts();

  async function checkNow(opts?: { silent?: boolean }) {
    if (status.value === 'checking' || status.value === 'downloading' || status.value === 'installing') return null;
    status.value = 'checking';
    errorMsg.value = null;
    lastCheckAt.value = Date.now();
    try {
      // In dev, skip real checks
      if (import.meta.env.DEV) {
        status.value = 'not-available';
        if (!opts?.silent) toastInfo('En dev: le check de mise à jour est ignoré.');
        return null;
      }
      const upd = await check();
      if (!upd) {
        status.value = 'not-available';
        if (!opts?.silent) toastSuccess('Aucune mise à jour disponible.');
        available.value = null;
        return null;
      }
      available.value = upd;
      status.value = 'available';
      if (!opts?.silent) {
        toastInfo(`Mise à jour disponible: v${upd.version}`);
      }
      return upd;
    } catch (e: any) {
      status.value = 'error';
      const msg = (e && (e.message || String(e))) || 'Échec de la vérification';
      errorMsg.value = msg;
      if (!opts?.silent) toastError(`Échec de la vérification: ${msg}`);
      return null;
    }
  }

  async function downloadAndInstall() {
    if (!available.value) return false;
    try {
      status.value = 'downloading';
      progress.value = { downloaded: 0, total: 0 };
      let total = 0;
      await available.value.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            total = (event as any).data?.contentLength || 0;
            progress.value = { downloaded: 0, total };
            break;
          case 'Progress':
            progress.value = {
              downloaded: progress.value.downloaded + (event.data.chunkLength || 0),
              total,
            };
            break;
          case 'Finished':
            status.value = 'downloaded';
            break;
        }
      });
      status.value = 'installing';
      // On Windows the app will exit automatically when installing
      // Await a small delay to make sure UI updates before exit
      setTimeout(async () => {
        try {
          await relaunch();
        } catch {}
      }, 50);
      return true;
    } catch (e: any) {
      status.value = 'error';
      const msg = (e && (e.message || String(e))) || 'Échec de l\'installation';
      errorMsg.value = msg;
      toastError(`Échec de la mise à jour: ${msg}`);
      return false;
    }
  }

  async function openReleases() {
    try {
      await openUrl('https://github.com/SkyDroxy/avrix-launcher-client/releases');
    } catch {}
  }

  return {
    status,
    errorMsg,
    available,
    progress,
    lastCheckAt,
    autoCheckOnStartup,
    checkNow,
    downloadAndInstall,
    openReleases,
  };
}
