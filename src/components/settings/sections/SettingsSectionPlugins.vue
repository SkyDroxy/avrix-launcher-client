<template>
  <div class="space-y-4">
    <h2 class="text-xl font-semibold">Plugins</h2>
    <p class="text-[12px] opacity-70 -mt-2">
      Paramètres liés au Workshop et à l'installation/maj des plugins.
    </p>

    <div class="rounded-xl border border-neutral-700/60 bg-neutral-800/40 p-4 space-y-3">
      <div class="flex items-center justify-between">
        <div>
          <div class="font-semibold">Scan Workshop au démarrage</div>
          <div class="text-[12px] opacity-70">
            Recherche automatiquement les plugins Workshop au lancement.
          </div>
        </div>
        <UiCheckbox v-model="autoWorkshopScanOnStartup" :switch="true" size="sm" />
      </div>
      <div class="flex items-center justify-between">
        <div>
          <div class="font-semibold">Installation automatique</div>
          <div class="text-[12px] opacity-70">
            Installe automatiquement les plugins Workshop détectés.
          </div>
        </div>
        <UiCheckbox v-model="autoWorkshopInstall" :switch="true" size="sm" />
      </div>
      <div class="flex items-center justify-between">
        <div>
          <div class="font-semibold">Mise à jour automatique</div>
          <div class="text-[12px] opacity-70">
            Met automatiquement à jour les plugins si une version supérieure est disponible.
          </div>
        </div>
        <UiCheckbox v-model="autoWorkshopUpdate" :switch="true" size="sm" />
      </div>
      <div class="pt-2 text-[12px] opacity-70">Les options sont sauvegardées automatiquement.</div>
    </div>
  </div>
</template>
<script setup lang="ts">
import UiCheckbox from '@components/ui/input/UiCheckbox.vue';
import { useSettings } from '@composables/useSettings';
import { watch } from 'vue';

const { autoWorkshopScanOnStartup, autoWorkshopInstall, autoWorkshopUpdate, save } = useSettings();

watch([autoWorkshopScanOnStartup, autoWorkshopInstall, autoWorkshopUpdate], () => {
  save();
});
</script>
