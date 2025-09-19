<template>
  <div
    class="group/launch relative overflow-hidden rounded-2xl border border-neutral-800/70 bg-neutral-950/40 backdrop-blur-sm shadow-xl transition hover:border-neutral-500/70"
  >
    <div class="relative h-64 w-full">
      <img
        :src="image"
        alt=""
        class="absolute inset-0 h-full w-full object-cover transition-all duration-700 ease-[cubic-bezier(.2,.8,.2,1)] group-hover/launch:scale-[1.04] group-hover/launch:blur-sm"
      />
      <div
        class="absolute inset-0 bg-gradient-to-t from-neutral-950/85 via-neutral-950/30 to-transparent"
      />

      <div class="absolute left-6 right-6 bottom-6 z-40">
        <div class="max-w-xl space-y-3">
          <h2 class="text-2xl font-extrabold text-white drop-shadow-[0_1px_0_rgba(0,0,0,0.55)]">
            {{ title }}
          </h2>

          <div class="text-sm text-white/85">
            <slot name="subtitle" />
          </div>

          <div class="flex items-center gap-3 pt-1">
            <UiCheckbox v-model="steamModel" switch size="md" color="emerald">
              Lancer avec Steam
            </UiCheckbox>
          </div>

          <GlowButton :disabled="disabled" size="xxl" icon="mingcute:game-2-fill" @click="onLaunch">
            <slot>LANCER LE JEU</slot>
          </GlowButton>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import GlowButton from '@components/ui/buttons/GlowButton.vue';
import UiCheckbox from '@components/ui/input/UiCheckbox.vue';
import { computed, ref, watch } from 'vue';

const emit = defineEmits<{
  (e: 'launch', payload?: { steam: boolean }): void;
  (e: 'update:steam', value: boolean): void;
}>();

const props = defineProps({
  image: { type: String, required: true },
  title: { type: String, required: true },
  disabled: { type: Boolean, default: false },
  steam: { type: Boolean, default: true },
});

const localSteam = ref<boolean>(props.steam);
watch(
  () => props.steam,
  (v) => (localSteam.value = v)
);
const steamModel = computed<boolean>({
  get: () => localSteam.value,
  set: (v) => {
    localSteam.value = v;
    emit('update:steam', v);
  },
});

function onLaunch() {
  emit('launch', { steam: steamModel.value });
}
</script>

<style scoped></style>
