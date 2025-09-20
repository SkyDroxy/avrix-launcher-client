<template>
  <div
    class="flex items-start border border-neutral-600 gap-4 rounded-lg p-3 bg-neutral-800/40 hover:bg-neutral-800/60 transition"
  >
    <!-- Icon/Image 1:1 -->
    <div
      class="w-16 h-16 md:w-20 md:h-20 shrink-0 rounded-md overflow-hidden bg-neutral-900/60 grid place-items-center"
    >
      <img
        v-if="imgSrc"
        :src="imgSrc"
        alt="plugin"
        class="w-full h-full object-cover"
        @error="onImgError"
        @load="onImgLoad"
      />
      <Icon v-else name="mingcute:plugin-2-fill" :width="58" class="opacity-60" />
    </div>
    <!-- Content -->
    <div class="min-w-0 flex-1">
      <div class="flex items-start justify-between gap-2">
        <div class="min-w-0">
          <div class="font-semibold leading-tight truncate">{{ dName }}</div>
          <div class="text-[11px] opacity-70 flex items-center gap-2">
            <UiBadge size="xs">v{{ vName }}</UiBadge>
            <span :title="fullDate(plugin.modified)">{{ shortDate(plugin.modified) }}</span>
            <span class="opacity-60">{{ formatSize(plugin.sizeKB) }}</span>
          </div>
        </div>
        <UiButton
          v-if="!plugin.internal"
          square
          size="xs"
          variant="ghost"
          class="text-neutral-400 hover:text-red-400"
          :title="'Supprimer ' + dName"
          @click.stop="$emit('delete', plugin)"
        >
          <Icon name="mingcute:close-circle-fill" :width="16" />
        </UiButton>
      </div>
      <div class="text-[11px] opacity-70" v-if="plugin.description">
        <span v-if="!expanded" class="line-clamp-2">{{ plugin.description }}</span>
        <span v-else class="whitespace-pre-line">{{ plugin.description }}</span>
        <UiButton
          v-if="showToggle"
          size="xs"
          variant="ghost"
          class="ml-1 text-[10px] align-top text-indigo-300 hover:text-indigo-200 underline"
          :focus-ring="false"
          :press="false"
          @click="expanded = !expanded"
        >
          <div class="flex items-center gap-1">
            <Icon :name="expanded ? 'mingcute:up-line' : 'mingcute:down-line'" :width="14" />
            <span>{{ expanded ? 'Réduire' : 'Lire plus' }}</span>
          </div>
        </UiButton>
      </div>
      <div class="flex items-center justify-between opacity-60 text-[10px] mt-1">
        <div class="flex items-center gap-2 min-w-0">
          <span v-if="plugin.author" class="truncate" :title="plugin.author"
            ><Icon name="mingcute:user-4-fill" :width="14" /> {{ plugin.author }}</span
          >
          <span v-else class="italic text-neutral-500">Auteur ?</span>
          <UiBadge
            v-if="plugin.workshopId"
            variant="indigo"
            size="xs"
            class="uppercase tracking-wide cursor-pointer"
            @click="openWorkshop(plugin.workshopId)"
            title="Ouvrir l'item Workshop"
          >
            WORKSHOP
          </UiBadge>
        </div>
        <div class="flex items-center gap-2">
          <UiBadge
            v-if="plugin.environment"
            :variant="
              plugin.environment == 'client'
                ? 'info'
                : plugin.environment == 'server'
                  ? 'warning'
                  : 'gray'
            "
            size="xs"
            class="uppercase tracking-wide"
          >
            {{ plugin.environment }}
          </UiBadge>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { displayName, versionOf, formatSize, fullDate, shortDate } from '@helpers/pluginFormat';
import { invoke } from '@tauri-apps/api/core';
import { computed, ref, watchEffect } from 'vue';

import Icon from '@/components/common/Icon.vue';
import UiBadge from '@/components/ui/buttons/UiBadge.vue';
import UiButton from '@/components/ui/buttons/UiButton.vue';

import type { PluginInfo } from '@interfaces/plugins';

const props = defineProps<{ plugin: PluginInfo }>();
defineEmits(['delete']);

const dName = computed(() => displayName(props.plugin));
const vName = computed(() => versionOf(props.plugin));
const imgSrc = ref<string | null>(null);
const expanded = ref(false);
const showToggle = computed(() => (props.plugin.description || '').length > 140);

watchEffect(() => {
  imgSrc.value = props.plugin.image || props.plugin.imageUrl || null;
});

function onImgError(e: Event) {
  const el = e.target as HTMLImageElement;
  if (!el || !el.complete) return;
  imgSrc.value = null;
}

function onImgLoad(e: Event) {
  const el = e.target as HTMLImageElement;
  if (!el) return;
}

async function openWorkshop(id?: string) {
  if (!id) return;
  try {
    await invoke('open_external', { url: `steam://url/CommunityFilePage/${id}` });
  } catch (_) {
    // Fallback to web URL if Steam protocol fails
    try {
      await invoke('open_external', {
        url: `https://steamcommunity.com/sharedfiles/filedetails/?id=${id}`,
      });
    } catch (_) {}
  }
}
</script>
<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
