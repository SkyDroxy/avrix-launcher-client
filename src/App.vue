<template>
  <div class="h-screen flex flex-col bg-neutral-800 text-neutral-200">
    <TitleBar />
    <div class="flex flex-1 min-h-0">
      <Sidebar />
      <div class="flex-1 flex flex-col min-w-0">
        <main class="flex-1 min-h-0 flex flex-col px-6 pt-5 pb-8 overflow-y-auto custom-scrollbar">
          <RouterView v-slot="{ Component }">
            <component :is="Component" @launched="onLaunched" />
          </RouterView>
        </main>
      </div>
    </div>
    <ToastContainer />
  </div>
</template>

<script setup>
import Sidebar from '@components/layout/sidebar/Sidebar.vue';
import TitleBar from '@components/layout/TitleBar.vue';
import ToastContainer from '@components/ui/feedback/ToastContainer.vue';
import { useRouter } from 'vue-router';

const router = useRouter();
function onLaunched() {
  router.push({ name: 'logs' });
}
</script>
