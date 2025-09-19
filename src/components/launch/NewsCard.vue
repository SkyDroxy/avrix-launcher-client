<template>
  <div
    class="relative group rounded-xl border border-neutral-700/70 bg-neutral-900/60 overflow-hidden shadow-inner backdrop-blur-sm transition hover:border-neutral-500/70 cursor-pointer"
    @click="$emit('open')"
  >
    <div
      class="absolute inset-0 pointer-events-none opacity-0 group-hover:opacity-100 transition duration-600 z-30"
    >
      <div class="absolute inset-0 bg-neutral-900/18"></div>
      <div
        class="absolute inset-0 mix-blend-screen bg-[radial-gradient(circle_at_78%_30%,rgba(16,185,129,0.38),transparent_65%)]"
      ></div>
      <div
        class="absolute inset-0 mix-blend-screen bg-[radial-gradient(circle_at_25%_70%,rgba(99,102,241,0.34),transparent_65%)]"
      ></div>
    </div>
    <span
      v-if="statusDot"
      class="absolute top-2 left-2 w-3 h-3 rounded-full bg-emerald-400 ring-4 ring-emerald-400/20"
    ></span>

    <div class="h-40 w-full overflow-hidden relative z-10">
      <img
        :src="image"
        alt=""
        class="w-full h-full object-cover transform will-change-transform group-hover:scale-[1.10] transition duration-[1600ms] ease-[cubic-bezier(.19,1,.22,1)] group-hover:brightness-[0.86]"
      />
      <div
        class="pointer-events-none absolute inset-0 opacity-0 group-hover:opacity-30 transition duration-600 bg-[radial-gradient(circle_at_50%_50%,rgba(255,255,255,0.045),transparent_70%)] z-10"
      ></div>
    </div>

    <div class="absolute inset-0 flex flex-col p-4 z-40">
      <div
        class="flex items-start justify-between text-[11px] font-semibold tracking-wide uppercase opacity-90"
      >
        <span>{{ month }}</span>
        <UiBadge v-if="tag" size="xs" class="uppercase tracking-wide">
          <slot name="tag">{{ tag }}</slot>
        </UiBadge>
      </div>
      <div class="mt-auto flex items-end">
        <h3 class="text-2xl font-extrabold drop-shadow leading-7 pr-6">
          <slot name="title">{{ title }}</slot>
        </h3>
        <div
          class="ml-auto relative flex items-center justify-center w-8 h-8 rounded-full bg-neutral-900/70 border border-neutral-600/60 group-hover:bg-emerald-500 group-hover:border-emerald-400 transition"
        >
          <svg viewBox="0 0 24 24" class="w-4 h-4" fill="currentColor">
            <path d="M9 6l6 6-6 6" />
          </svg>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
defineEmits<{ (e: 'open'): void }>();
import { isNewsTagValue } from '@common/enums/NewsTag';
import UiBadge from '@components/ui/buttons/UiBadge.vue';

defineProps({
  image: { type: String, required: true },
  title: { type: String, required: true },
  month: { type: String, required: true },
  tag: { type: String, default: '', validator: (v: string) => v === '' || isNewsTagValue(v) },
  statusDot: { type: Boolean, default: true },
});
</script>
