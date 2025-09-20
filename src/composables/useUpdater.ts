import { openUrl } from '@tauri-apps/plugin-opener';
import { relaunch } from '@tauri-apps/plugin-process';
import { check, type Update } from '@tauri-apps/plugin-updater';
import { ref, watch, shallowRef, markRaw } from 'vue';

import { useSettings } from './useSettings';
import { useToasts } from './useToasts';

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
const available = shallowRef<Update | null>(null);
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
    if (
      status.value === 'checking' ||
      status.value === 'downloading' ||
      status.value === 'installing'
    )
      return null;
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
      // IMPORTANT: avoid wrapping the Update object in a Vue proxy
      available.value = markRaw(upd) as Update;
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
      const upd = available.value;
      // guard: ensure we are not dealing with a reactive proxy
      await upd!.downloadAndInstall((event) => {
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
      // Sur Windows, l'installeur NSIS ferme l'app et peut la relancer si /R est passé.
      // Éviter d'appeler relaunch() pour ne pas créer une course qui rouvre puis referme.
      const isWindows = navigator.userAgent.includes('Windows');
      if (!isWindows) {
        setTimeout(async () => {
          try {
            await relaunch();
          } catch {}
        }, 50);
      }
      return true;
    } catch (e: any) {
      status.value = 'error';
      const msg = (e && (e.message || String(e))) || "Échec de l'installation";
      errorMsg.value = msg;
      toastError(`Échec de la mise à jour: ${msg}`);
      return false;
    }
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
  };
}
