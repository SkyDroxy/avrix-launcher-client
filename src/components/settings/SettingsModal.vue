<template>
  <BaseModal
    v-model="open"
    width="2xl"
    placement="center"
    :closable="true"
    :close-on-backdrop="true"
    :body-scrollable="false"
    panel-height="70vh"
  >
    <template #title>
      <div class="flex items-center gap-2">
        <Icon name="mingcute:settings-5-fill" :width="18" />
        <span>Paramètres</span>
      </div>
    </template>

    <div class="flex min-h-[320px] h-full">
      <!-- Categories -->
      <aside class="w-56 pr-4 border-r border-neutral-700/60 custom-scrollbar min-h-0">
        <nav class="space-y-1">
          <button
            v-for="cat in categories"
            :key="cat.key"
            class="w-full flex items-center gap-2 px-2 py-2 rounded-md text-[13px] hover:bg-neutral-800/60"
            :class="cat.key === current ? 'bg-neutral-800/70 text-neutral-100' : 'text-neutral-300'"
            @click="current = cat.key"
          >
            <Icon :name="cat.icon" :width="18" />
            <span class="truncate">{{ cat.label }}</span>
          </button>
        </nav>
      </aside>

      <!-- Content -->
      <section class="flex-1 pl-4 custom-scrollbar min-h-0">
        <SettingsSectionGame v-if="current === 'game'" />
        <SettingsSectionPlugins v-else-if="current === 'plugins'" />
        <SettingsSectionUpdates v-else-if="current === 'updates'" />
        <SettingsSectionAbout v-else />
      </section>
    </div>
  </BaseModal>
</template>
<script setup lang="ts">
import Icon from '@components/common/Icon.vue';
import SettingsSectionAbout from '@components/settings/sections/SettingsSectionAbout.vue';
import SettingsSectionGame from '@components/settings/sections/SettingsSectionGame.vue';
import SettingsSectionPlugins from '@components/settings/sections/SettingsSectionPlugins.vue';
import SettingsSectionUpdates from '@components/settings/sections/SettingsSectionUpdates.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import { computed, ref } from 'vue';

const props = defineProps<{ modelValue: boolean }>();
const emit = defineEmits<{ (e: 'update:modelValue', v: boolean): void }>();
const open = computed({
  get: () => props.modelValue,
  set: (v: boolean) => emit('update:modelValue', v),
});

type CatKey = 'game' | 'plugins' | 'updates' | 'about';
const categories: Array<{ key: CatKey; label: string; icon: string }> = [
  { key: 'game', label: 'Jeu', icon: 'mingcute:game-2-fill' },
  { key: 'plugins', label: 'Plugins', icon: 'mingcute:plugin-2-fill' },
  { key: 'updates', label: 'Mises à jour', icon: 'mingcute:refresh-2-fill' },
  { key: 'about', label: 'À propos', icon: 'mingcute:information-fill' },
];
const current = ref<CatKey>('game');
</script>

<style scoped>
/* No local styles; uses global .custom-scrollbar from src/style.css */
</style>
