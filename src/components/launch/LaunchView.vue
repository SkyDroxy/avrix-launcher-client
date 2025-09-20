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

    <!-- Version selection modal (reusable component) -->
    <VersionSelectModal v-model="showVersionModal" @confirm="onVersionModalConfirm" />

    <!-- Missing game modal -->
    <ConfirmModal
      v-model="showMissingGameModal"
      title="Jeu introuvable"
      :message="missingGameMessage"
      confirm-label="Ouvrir dans Steam"
      cancel-label="Fermer"
      @cancel="showMissingGameModal = false"
      @update:model-value="onMissingModalUpdate"
      @confirm="openSteamStore"
    />
  </div>
</template>
<script setup lang="ts">
import heroPlaceholder from '@assets/hero-placeholder.png';
import menuImage from '@assets/menu.png';
import { NewsTag } from '@common/enums/NewsTag';
import LaunchCard from '@components/launch/LaunchCard.vue';
import NewsCard from '@components/launch/NewsCard.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import ConfirmModal from '@components/ui/overlays/ConfirmModal.vue';
import VersionSelectModal from '@components/versions/modals/VersionSelectModal.vue';
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
// Version selection modal if none selected
const showVersionModal = ref(false);
const pendingLaunchAfterSelect = ref(false);
// Missing game modal state
const showMissingGameModal = ref(false);
const missingGameMessage =
  "Le jeu Project Zomboid n'est pas installé à cet emplacement. Vous pouvez sélectioner un autre emplacement dans les paramètres.\n\nSi vous ne possédez pas le jeu, vous pouvez l'acheter et l'installer via Steam.";

async function ensureVersionSelected() {
  try {
    const res = await invoke<{ selectedId?: string | null }>('list_versions');
    const selected = (res?.selectedId as string) || '';
    if (!selected) {
      showVersionModal.value = true;
    }
  } catch {}
}

async function onVersionModalConfirm(id: string) {
  if (!id) return;
  await invoke<string>('select_version', { id });
  showVersionModal.value = false;
  if (pendingLaunchAfterSelect.value) {
    loading.value = true;
    await doLaunchGame();
  }
}
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

async function doLaunchGame() {
  try {
    const res = await invoke('launch_game', { steam: steam.value, mem_mb: memoryMB.value || 3072 });
    if (res && typeof res === 'string' && res.startsWith('[Erreur]')) error.value = res;
    else emit('launched');
  } catch (e) {
    const msg = String(e);
    if (msg.includes('[GameMissing]')) {
      // Show modal instead of toast
      showMissingGameModal.value = true;
      error.value = '';
    } else {
      error.value = msg;
    }
  } finally {
    loading.value = false;
    pendingLaunchAfterSelect.value = false;
  }
}

async function launch() {
  if (loading.value) return;
  loading.value = true;
  error.value = '';
  await ensureVersionSelected();
  if (showVersionModal.value) {
    pendingLaunchAfterSelect.value = true;
    loading.value = false;
    return;
  }
  await doLaunchGame();
}

const emit = defineEmits(['launched']);

const { error: toastError } = useToasts();
onMounted(async () => {
  await loadSettings();
  // Do not auto-open the modal on view mount; it will open on launch if needed.
});
watch(
  () => error.value,
  (msg: string) => {
    if (msg) toastError(msg);
  }
);

async function openSteamStore() {
  try {
    await invoke('open_external', { url: 'steam://store/108600' });
  } catch (_) {
    // Fallback to web URL if Steam protocol fails
    try {
      await invoke('open_external', {
        url: 'https://store.steampowered.com/app/108600/Project_Zomboid/',
      });
    } catch (_) {}
  } finally {
    showMissingGameModal.value = false;
  }
}

function onMissingModalUpdate(v: boolean) {
  if (!v) error.value = '';
}
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
