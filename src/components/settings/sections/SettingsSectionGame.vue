<template>
  <section class="space-y-4">
    <div>
      <h2 class="text-2xl font-extrabold tracking-tight">Paramètres du jeu</h2>
    </div>

    <!-- Card: Game folder selection -->
    <div class="p-4 rounded-xl bg-neutral-900/40 border border-neutral-800/70">
      <div>
        <div class="flex items-center gap-2 mb-1">
          <Icon name="mingcute:folder-2-fill" :width="18" />
          <h3 class="text-sm font-extrabold tracking-wide">Dossier du jeu</h3>
        </div>
        <div class="text-xs text-neutral-400 mb-2">
          Dossier actuel utilisé par Avrix pour Project Zomboid.
        </div>
        <div
          class="rounded-lg border border-neutral-800/60 bg-neutral-950/30 p-3 text-[12px] space-y-1"
        >
          <div>
            <span class="text-neutral-400">Actuel:</span>
            <span class="ml-2 select-all break-all">{{ gameRootInfo.effective }}</span>
          </div>
          <div class="text-neutral-400">
            <span>Détecté:</span>
            <span class="ml-2 select-all break-all text-neutral-300">{{
              gameRootInfo.detected ? gameRootInfo.detected : '(introuvable)'
            }}</span>
          </div>
          <div v-if="gameRootInfo.overridePath" class="text-amber-300/90">
            Override: {{ gameRootInfo.overridePath }}
            <span v-if="!gameRootInfo.isOverrideValid" class="ml-2 text-red-400">(invalide)</span>
          </div>
        </div>
        <div class="mt-2 flex gap-2">
          <UiButton size="xs" variant="primary" @click="pickGameRoot">Changer…</UiButton>
          <UiButton
            size="xs"
            variant="ghost"
            @click="resetGameRoot"
            :disabled="!gameRootInfo.overridePath"
          >
            Réinitialiser
          </UiButton>
        </div>
      </div>
    </div>

    <!-- Card: Memory allocation -->
    <div class="p-4 rounded-xl bg-neutral-900/40 border border-neutral-800/70">
      <div class="flex items-center gap-2 mb-1">
        <Icon name="mingcute:server-2-fill" :width="18" />
        <h3 class="text-sm font-extrabold tracking-wide">Mémoire allouée</h3>
      </div>
      <div class="text-xs text-neutral-400 mb-2">
        Définissez la mémoire à allouer à l'instance de jeu
      </div>
      <div class="flex items-center gap-3 mb-2">
        <label class="text-xs text-neutral-400">Préréglage</label>
        <UiSelect
          v-model="selectedPreset"
          :options="presetOptions"
          placeholder="Choisir"
          :disabled="customEnabled"
        />
      </div>
      <div class="mb-3">
        <UiCheckbox v-model="customEnabled" :switch="true" size="sm">Personnalisé</UiCheckbox>
      </div>
      <RangeInput
        v-model="memoryMB"
        :min="500"
        :max="maxMB"
        :step="256"
        :min-label="'500 Mo'"
        :max-label="format(maxMB)"
        unit="GB"
        :disabled="!customEnabled"
      />
      <div class="mt-2 text-[11px] text-neutral-400">
        Vous avez
        <UiBadge>
          {{ format(availableMB) }} <Icon class="ml-1" name="mingcute:server-2-fill" width="16" />
        </UiBadge>
        libres à allouer.
      </div>
    </div>
  </section>
</template>
<script setup lang="ts">
import Icon from '@components/common/Icon.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiSelect from '@components/ui/data/UiSelect.vue';
import RangeInput from '@components/ui/input/RangeInput.vue';
import UiCheckbox from '@components/ui/input/UiCheckbox.vue';
import { useSettings } from '@composables/useSettings';
import { useToasts } from '@composables/useToasts';
import { invoke } from '@tauri-apps/api/core';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { onMounted, ref, watch } from 'vue';

import UiBadge from '@/components/ui/buttons/UiBadge.vue';

const { memPreset, memoryMB, load, save, mbToPreset, presetToMb } = useSettings();
const { error: toastError, success: toastSuccess } = useToasts();

const availableMB = ref(0);
const totalMB = ref(0);
const maxMB = ref(2048);
const selectedPreset = ref<string>('3072');
const customEnabled = ref(false);

const presetOptions = [
  { value: '3072', label: 'Auto (~3 Go)' },
  { value: '1024', label: '1 Go' },
  { value: '2048', label: '2 Go' },
  { value: '4096', label: '4 Go' },
  { value: '6144', label: '6 Go' },
  { value: '8192', label: '8 Go' },
];

function format(mb: number) {
  return `${(mb / 1024).toFixed(1)} Go`;
}

watch(
  () => selectedPreset.value,
  (v) => {
    const num = Number(v || '0');
    if (!Number.isFinite(num) || num <= 0) return;
    memoryMB.value = Math.min(maxMB.value, num);
  },
  { immediate: false }
);

onMounted(async () => {
  await load();
  await refreshGameRootInfo();
  try {
    const info = await invoke<{ totalMb: number; availableMb: number }>('get_memory_info');
    const total = (info as any).totalMb ?? (info as any).total_mb;
    const avail = (info as any).availableMb ?? (info as any).available_mb;
    totalMB.value = Number(total) || 0;
    availableMB.value = Number(avail) || 0;
    maxMB.value = Math.max(1024, Math.floor(availableMB.value / 256) * 256);
    memoryMB.value = Math.min(maxMB.value, memoryMB.value || presetToMb(memPreset.value));
    selectedPreset.value = String(
      presetOptions
        .map((p) => Number(p.value))
        .reduce(
          (closest, cur) =>
            Math.abs(cur - memoryMB.value) < Math.abs(closest - memoryMB.value) ? cur : closest,
          Number(presetOptions[0].value)
        )
    );
  } catch (_) {
    maxMB.value = 4096;
    availableMB.value = 4096;
    memoryMB.value = Math.min(maxMB.value, memoryMB.value || presetToMb(memPreset.value));
    selectedPreset.value = String(
      presetOptions
        .map((p) => Number(p.value))
        .reduce(
          (closest, cur) =>
            Math.abs(cur - memoryMB.value) < Math.abs(closest - memoryMB.value) ? cur : closest,
          Number(presetOptions[0].value)
        )
    );
  }
});

watch(
  () => memoryMB.value,
  async (mb) => {
    memPreset.value = mbToPreset(mb);
    await save();
  }
);

type GameRootInfo = {
  effective: string;
  detected: string;
  overridePath?: string | null;
  isOverrideValid: boolean;
};

const gameRootInfo = ref<GameRootInfo>({
  effective: '',
  detected: '',
  overridePath: null,
  isOverrideValid: true,
});

async function refreshGameRootInfo() {
  try {
    const info = (await invoke('get_game_root_info')) as any;
    gameRootInfo.value = {
      effective: info.effective || '',
      detected: info.detected || '',
      overridePath: info.overridePath ?? info.override_path ?? null,
      isOverrideValid: Boolean(info.isOverrideValid ?? info.is_override_valid),
    };
  } catch (_) {
    gameRootInfo.value = { effective: '', detected: '', overridePath: null, isOverrideValid: true };
  }
}

async function pickGameRoot() {
  try {
    const dir = await openDialog({
      directory: true,
      multiple: false,
      title: 'Sélectionner le dossier Project Zomboid',
    } as any);
    const chosen = Array.isArray(dir) ? dir[0] : dir;
    if (!chosen) return;
    const path = String(chosen);
    try {
      await invoke('validate_game_root', { path });
    } catch (e) {
      toastError(String(e) || 'Dossier Project Zomboid invalide');
      return;
    }
    await invoke('set_game_root', { path });
    await refreshGameRootInfo();
    toastSuccess('Dossier du jeu mis à jour');
  } catch (e) {
    // ignore errors; invalid choices are rejected by backend
  }
}

async function resetGameRoot() {
  try {
    await invoke('clear_game_root_override');
    await refreshGameRootInfo();
  } catch (_) {}
}
</script>
