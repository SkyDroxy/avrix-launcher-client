<template>
  <div class="relative h-full overflow-hidden flex flex-col">
    <div class="pt-4 pb-12">
      <LaunchCard
        :image="launchCardImage"
        title="Lancement du jeu"
        :disabled="loading"
        v-model:steam="steam"
        @launch="launch"
      />
    </div>

    <section class="news-scroll space-y-4 pr-2 flex-1 overflow-y-auto">
      <div class="flex items-center">
        <h2 class="text-lg font-extrabold tracking-wide">DERNIÈRES ACTUALITÉS</h2>
        <UiButton
          class="ml-auto text-[11px] uppercase tracking-wide text-neutral-400 hover:text-neutral-200 !px-2 h-auto"
          size="xs"
          variant="ghost"
          :focus-ring="false"
          :press="false"
        >
          Toute l'actualité
        </UiButton>
      </div>
      <div class="grid gap-5 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 pb-6">
        <NewsCard
          v-for="n in news"
          :key="n.id"
          :image="n.image"
          :title="n.title"
          :month="n.month"
          :tag="n.tag"
          :status-dot="n.statusDot"
        />
      </div>
    </section>
  </div>
</template>
<script setup lang="ts">
import heroPlaceholder from '@assets/hero-placeholder.png';
import menuImage from '@assets/menu.png';
import { NewsTag } from '@common/enums/NewsTag';
import LaunchCard from '@components/launch/LaunchCard.vue';
import NewsCard from '@components/launch/NewsCard.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import { useSettings } from '@composables/useSettings';
import { useToasts } from '@composables/useToasts';
import { invoke } from '@tauri-apps/api/core';
import { ref, watch, onMounted } from 'vue';

import type { NewsItem } from '@interfaces/news';

const loading = ref(false);
const error = ref('');
const steam = ref(true);
const launchCardImage = ref(menuImage);
const heroImage = ref(heroPlaceholder);
const { memoryMB, load: loadSettings } = useSettings();
const news = ref<NewsItem[]>([
  {
    id: 1,
    image: heroPlaceholder,
    title: 'NEW PATCH',
    month: 'AUGUST',
    tag: NewsTag.PATCH,
    statusDot: true,
  },
  {
    id: 2,
    image: heroPlaceholder,
    title: 'NEW EVENT',
    month: 'JULY',
    tag: NewsTag.EVENT,
    statusDot: true,
  },
  {
    id: 3,
    image: heroPlaceholder,
    title: 'NEW UPDATE',
    month: 'JUNE',
    tag: NewsTag.UPDATE,
    statusDot: true,
  },
  {
    id: 4,
    image: heroPlaceholder,
    title: 'NEW FEATURE',
    month: 'MAY',
    tag: NewsTag.FEATURE,
    statusDot: false,
  },
]);

async function launch() {
  if (loading.value) return;
  loading.value = true;
  error.value = '';
  try {
    const res = await invoke('launch_game', { steam: steam.value, mem_mb: memoryMB.value || 3072 });
    if (res && typeof res === 'string' && res.startsWith('[Erreur]')) error.value = res;
    else emit('launched');
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

const emit = defineEmits(['launched']);

const { error: toastError } = useToasts();
onMounted(async () => {
  await loadSettings();
});
watch(
  () => error.value,
  (msg: string) => {
    if (msg) toastError(msg);
  }
);
</script>
<style scoped>
@keyframes pulseSlow {
  0%,
  100% {
    opacity: 0.5;
  }
  50% {
    opacity: 0.9;
  }
}
@keyframes pulseSlower {
  0%,
  100% {
    opacity: 0.3;
  }
  50% {
    opacity: 0.8;
  }
}
.animate-pulse-slow {
  animation: pulseSlow 8s ease-in-out infinite;
}
.animate-pulse-slower {
  animation: pulseSlower 12s ease-in-out infinite;
}

.news-scroll {
  overflow-y: auto;
  overscroll-behavior: contain;
  scrollbar-width: thin;
  scrollbar-color: color-mix(in srgb, var(--color-neutral-400) 75%, transparent) transparent;
}
.news-scroll::-webkit-scrollbar {
  width: 10px;
}
.news-scroll::-webkit-scrollbar-track {
  background: transparent;
}
.news-scroll::-webkit-scrollbar-thumb {
  background-color: color-mix(in srgb, var(--color-neutral-400) 75%, transparent);
  border-radius: 9999px;
  border: 2px solid color-mix(in srgb, black 25%, transparent);
}
.news-scroll::-webkit-scrollbar-thumb:hover {
  background-color: color-mix(in srgb, var(--color-neutral-300) 90%, transparent);
}
</style>
