import { invoke } from '@tauri-apps/api/core';
import { LazyStore } from '@tauri-apps/plugin-store';
import { ref } from 'vue';

// Kept for backward compatibility in stored settings; backend no longer uses presets
export type MemPreset = 'auto' | 'low' | 'mid' | 'high';

export interface SettingsModel {
  memPreset: MemPreset;
  memoryMB: number; // exact memory amount in MB
  autoCheckUpdates?: boolean;
}

// Use the settings file next to the executable; path is provided by backend
let store: LazyStore | null = null;
async function ensureStore(): Promise<LazyStore> {
  if (store) return store;
  let path: string | null = null;
  try {
    path = (await invoke<string>('get_settings_path')) || null;
  } catch (_) {
    path = 'avrix-settings.json';
  }
  store = new LazyStore(path || 'avrix-settings.json');
  return store;
}

// Back-compat preset plus new numeric value
const memPreset = ref<MemPreset>('auto');
const memoryMB = ref<number>(0);
const autoCheckUpdates = ref<boolean>(true);

// Helpers to translate between presets and MB for migration/UI convenience
function presetToMb(p: MemPreset): number {
  switch (p) {
    case 'low':
      return 1024;
    case 'mid':
      return 2048;
    case 'high':
      return 4096;
    case 'auto':
    default:
      return 3072; // default ~3GB
  }
}

function mbToPreset(mb: number): MemPreset {
  // Only used to keep a friendly label in the store; pick a rough bucket
  if (mb <= 1280) return 'low';
  if (mb <= 3072) return 'mid';
  if (mb <= 5120) return 'high';
  return 'high';
}

export function useSettings() {
  async function load() {
    try {
      const s = await ensureStore();
      const hasNewMem = (await s.get('memoryMB')) !== null;
      const hasNewPreset = (await s.get('memPreset')) !== null;

      const v = (await s.get<MemPreset>('memPreset')) as MemPreset | null;
      if (v) memPreset.value = v;

      const m = (await s.get<number>('memoryMB')) as number | null;
      if (typeof m === 'number' && !Number.isNaN(m) && m > 0) {
        memoryMB.value = m;
      } else {
        // migrate from preset if numeric value missing
        memoryMB.value = presetToMb(memPreset.value);
        await s.set('memoryMB', memoryMB.value);
      }

      // Ensure preset key exists as well for older versions
      if (!v && hasNewMem && !hasNewPreset) {
        await s.set('memPreset', memPreset.value);
      }

  const ac = (await s.get<boolean>('autoCheckUpdates')) as boolean | null;
  if (typeof ac === 'boolean') autoCheckUpdates.value = ac;

  await s.save();
    } catch (e) {
      // ignore; use defaults
      if (!memoryMB.value) memoryMB.value = 3072;
    }
  }

  async function save() {
    const s = await ensureStore();
    await s.set('memPreset', memPreset.value);
    await s.set('memoryMB', memoryMB.value || presetToMb(memPreset.value));
    await s.set('autoCheckUpdates', autoCheckUpdates.value);
    await s.save();
  }

  return {
    memPreset,
    memoryMB,
    autoCheckUpdates,
    load,
    save,
    mbToPreset,
    presetToMb,
  };
}
