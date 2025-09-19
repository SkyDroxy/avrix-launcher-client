<template>
  <aside
    class="w-24 bg-neutral-900/95 backdrop-blur-sm border-r border-neutral-800 flex flex-col py-4 select-none items-center"
  >
    <div class="mb-4 mt-1">
      <img
        :src="iconSrc"
        alt="Avrix"
        class="w-12 h-12 drop-shadow-sm opacity-90 cursor-pointer"
        @click="go('launch')"
        @error="onIconError"
      />
    </div>
    <nav class="flex flex-col gap-2 w-full px-0 mt-1 items-center">
      <SidebarItem
        label="Jeu"
        icon="mingcute:game-1-fill"
        section="launch"
        :current="routeName"
        @activate="go"
      />
      <SidebarItem
        label="Plugins"
        icon="mingcute:plugin-2-fill"
        section="plugins"
        :current="routeName"
        @activate="go"
      />
      <SidebarItem
        label="Versions"
        icon="mingcute:version-fill"
        section="versions"
        :current="routeName"
        @activate="go"
      />
      <SidebarItem
        label="Logs"
        icon="mingcute:terminal-box-fill"
        section="logs"
        :current="routeName"
        @activate="go"
      />
      <SidebarItem
        v-if="devMode"
        label="PlayTest"
        icon="mingcute:test-tube-fill"
        section="playtest"
        :current="routeName"
        @activate="go"
      />
    </nav>
    <div class="mt-auto flex flex-col gap-2 w-full items-center">
      <SidebarItem
        label="Paramètres"
        icon="mingcute:settings-5-fill"
        section="settings"
        :current="routeName"
        @activate="go"
      />
      <div class="text-[10px] tracking-tight text-neutral-500 pt-1 select-text">v{{ version }}</div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import iconImage from '@assets/AvrixLauncher.ico';
import { useSettingsModal } from '@composables/useSettingsModal';
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import SidebarItem from './SidebarItem.vue';

declare const __APP_VERSION__: string | undefined;
const route = useRoute();
const router = useRouter();
const routeName = computed(() => (route.name ? String(route.name) : ''));
function go(section: string) {
  if (section === 'settings') {
    const { open } = useSettingsModal();
    open();
    return;
  }
  if (section === routeName.value) return;
  router.push({ name: section });
}

const iconSrc = ref(iconImage);

const version: string = (typeof __APP_VERSION__ !== 'undefined' && __APP_VERSION__) || '0.0.0';

function onIconError(e: Event) {
  const target = e.target as HTMLImageElement;
  target.replaceWith(
    Object.assign(document.createElement('div'), {
      className: 'w-12 h-12 flex items-center justify-center text-xl',
      textContent: '🕹️',
    })
  );
}

const devMode = import.meta.env.DEV;
</script>
