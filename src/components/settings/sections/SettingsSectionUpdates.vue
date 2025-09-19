<template>
  <section class="space-y-4">
    <div>
      <h2 class="text-2xl font-extrabold tracking-tight">Mises à jour</h2>
      <p class="text-xs text-neutral-400 mt-1">
        Paramètres liés aux mises à jour du lanceur et des ressources.
      </p>
    </div>
    <div class="p-4 rounded-xl bg-neutral-900/40 border border-neutral-800/70 space-y-3">
      <div class="flex items-center justify-between">
        <div>
          <div class="text-sm font-medium">Vérifier au démarrage</div>
          <div class="text-xs text-neutral-400">
            Recherche automatiquement des mises à jour au lancement.
          </div>
        </div>
        <UiCheckbox v-model="autoCheck" :switch="true" size="sm" />
      </div>
      <div class="flex items-center justify-between">
        <div>
          <div class="text-sm font-medium">Rechercher des mises à jour</div>
          <div class="text-xs text-neutral-400">Déclenche une vérification immédiate.</div>
        </div>
        <UiButton size="sm" @click="onCheck">Rechercher</UiButton>
      </div>
      <div v-if="status === 'available'" class="text-xs text-emerald-400">
        Mise à jour disponible: v{{ available?.version }}
      </div>
      <div v-else-if="status === 'not-available'" class="text-xs text-neutral-400">
        Aucune mise à jour disponible.
      </div>
      <div v-else-if="status === 'error'" class="text-xs text-red-400">Échec: {{ errorMsg }}</div>
    </div>
  </section>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiCheckbox from '@components/ui/input/UiCheckbox.vue';
import { useSettings } from '@composables/useSettings';
import { useUpdater } from '@composables/useUpdater';
import { watch } from 'vue';

const { checkNow, status, available, errorMsg, autoCheckOnStartup: autoCheck } = useUpdater();
const { load, save } = useSettings();
load();
watch(autoCheck, () => save());
async function onCheck() {
  await checkNow({ silent: false });
}
</script>
