import { openUrl } from '@tauri-apps/plugin-opener';
import { relaunch } from '@tauri-apps/plugin-process';
import { check, type Update } from '@tauri-apps/plugin-updater';
import { ref, watch, shallowRef, markRaw } from 'vue';
import { Command } from '@tauri-apps/plugin-shell';
import { BaseDirectory, writeFile } from '@tauri-apps/plugin-fs';
import { join, tempDir } from '@tauri-apps/api/path';

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
      const msg = (e && (e.message || String(e))) || "Échec de l'installation";
      errorMsg.value = msg;
      toastError(`Échec de la mise à jour: ${msg}`);
      // Fallback: try update-only installer if available in release assets/S3
      const ok = await fallbackUpdateInstaller();
      return ok;
    }
  }

  async function fallbackUpdateInstaller(): Promise<boolean> {
    try {
      // Construct S3 URL for update-only installer. Matches CI naming
      const ver = available.value?.version;
      if (!ver) return false;
      const fileName = `Avrix-Update-${ver}.exe`;
      const url = `https://s3.storage.skymunt.com/avrix-launcher/v${ver}/${fileName}`;

      // Write directly to temp root to avoid dir creation
      const relPath = fileName;

      // Download file
      const res = await fetch(url);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const buf = new Uint8Array(await res.arrayBuffer());
      await writeFile(relPath, buf, { baseDir: BaseDirectory.Temp });

      // Run installer silently and relaunch
      const fullTemp = await tempDir();
      const fullPath = await join(fullTemp, relPath);
      const cmd = Command.create('powershell', [
        '-NoProfile',
        '-NonInteractive',
        '-Command',
        `Start-Process -FilePath "${fullPath}" -ArgumentList "/VERYSILENT" -Wait`
      ]);
      await cmd.execute();
      try { await relaunch(); } catch {}
      return true;
    } catch (err) {
      // noop: keep previous error
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
    fallbackUpdateInstaller,
    openReleases,
  };
}
