<template>
  <Teleport to="body">
    <div
      class="fixed z-[9999] inset-0 pointer-events-none flex flex-col items-end gap-2 p-4 sm:p-6"
    >
      <transition-group
        name="toast-fade"
        tag="div"
        class="flex flex-col gap-2 w-full sm:w-80 max-w-full"
      >
        <div
          v-for="t in toasts"
          :key="t.id"
          class="pointer-events-auto relative rounded-md border px-3 py-2 text-[12px] leading-snug shadow bg-neutral-800/95 backdrop-blur flex items-center gap-2"
          :class="[
            'before:absolute before:-inset-px before:rounded-md before:pointer-events-none before:ring-2 before:ring-offset-0',
            t.type === 'error'
              ? 'border-red-500/50 before:ring-red-500/50'
              : t.type === 'success'
                ? 'border-emerald-500/50 before:ring-emerald-500/50'
                : t.type === 'warn'
                  ? 'border-amber-400/60 before:ring-amber-400/50'
                  : t.type === 'info'
                    ? 'border-indigo-400/50 before:ring-indigo-400/50'
                    : 'border-neutral-500/50 before:ring-neutral-500/40',
          ]"
        >
          <Icon
            :name="iconOf(t.type)"
            :width="16"
            class="mt-0.5"
            :class="{
              'text-red-400': t.type === 'error',
              'text-emerald-400': t.type === 'success',
              'text-indigo-300': t.type === 'info',
              'text-amber-400': t.type === 'warn',
            }"
          />
          <div class="flex-1 min-w-0 flex flex-col">
            <div class="flex items-center gap-2">
              <p class="font-medium truncate" v-if="t.title">{{ t.title }}</p>
              <p v-else class="font-medium truncate opacity-80">{{ t.message }}</p>
            </div>
            <p v-if="t.title" class="opacity-80 whitespace-pre-wrap break-words">{{ t.message }}</p>
            <div
              v-if="t.meta && Object.keys(t.meta).length"
              class="mt-1 text-[10px] grid gap-0.5 opacity-70"
            >
              <div v-for="(v, k) in t.meta" :key="k" class="flex justify-between gap-2">
                <span class="truncate">{{ k }}</span
                ><span class="truncate font-mono">{{ v }}</span>
              </div>
            </div>
          </div>
          <UiButton
            @click="close(t.id)"
            size="xs"
            variant="ghost"
            square
            class="text-neutral-400 hover:text-neutral-200 ml-1 !px-1 h-auto"
            :focus-ring="false"
            :press="false"
            :border="false"
            aria-label="Fermer"
            title="Fermer"
          >
            <Icon name="mingcute:close-circle-line" :width="16" />
          </UiButton>
        </div>
      </transition-group>
    </div>
  </Teleport>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import { useToasts } from '@composables/useToasts';

import Icon from '@/components/common/Icon.vue';

const { toasts, remove } = useToasts();
function close(id: number) {
  remove(id);
}
function iconOf(type: string) {
  switch (type) {
    case 'error':
      return 'mingcute:alert-octagon-fill';
    case 'success':
      return 'mingcute:check-circle-fill';
    case 'info':
      return 'mingcute:information-fill';
    case 'warn':
      return 'mingcute:alert-fill';
    default:
      return 'mingcute:information-fill';
  }
}
</script>
<style scoped>
.toast-fade-enter-active,
.toast-fade-leave-active {
  transition: all 0.18s ease;
}
.toast-fade-enter-from,
.toast-fade-leave-to {
  opacity: 0;
  transform: translateY(4px) scale(0.98);
}
</style>
