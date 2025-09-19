<template>
  <UiButton
    :disabled="disabled"
    class="relative cursor-pointer font-bold tracking-wide uppercase rounded-md overflow-hidden transition group select-none border-0"
    :class="[fullWidth ? 'w-full' : 'inline-flex', sizeClasses.wrapper]"
    :press="true"
    size="xl"
    :focus-ring="false"
    variant="ghost"
    @click="$emit('click')"
  >
    <span
      class="absolute inset-0 rounded-md bg-gradient-to-br from-emerald-400 via-emerald-500 to-emerald-600 opacity-80"
    ></span>
    <span
      class="absolute -inset-1 rounded-md bg-gradient-to-r from-transparent via-white/40 to-transparent opacity-0 group-hover:opacity-60 transition-opacity duration-700 [mask-image:radial-gradient(circle_at_30%_50%,white,transparent_65%)]"
    ></span>
    <span
      class="absolute inset-0 rounded-md ring-2 ring-emerald-400/40 group-hover:ring-emerald-300/70 transition"
    ></span>
    <span
      class="absolute inset-0 rounded-md shadow-[0_0_24px_-4px_rgba(16,185,129,0.65),0_0_64px_-12px_rgba(16,185,129,0.45)] opacity-70 group-hover:opacity-90 transition"
    ></span>
    <span class="relative z-10 flex items-center" :class="sizeClasses.inner">
      <span v-if="icon" class="relative flex items-center justify-center">
        <Icon
          :name="icon"
          :width="sizeClasses.icon"
          :height="sizeClasses.icon"
          class="animate-glow-pulse text-white drop-shadow-[0_0_6px_rgba(255,255,255,0.65)]"
        />
      </span>
      <span class="font-semibold tracking-wide text-white"> <slot /> </span>
    </span>
  </UiButton>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import { computed } from 'vue';

import Icon from '@/components/common/Icon.vue';

defineEmits<{ (e: 'click'): void }>();
const props = withDefaults(
  defineProps<{
    disabled?: boolean;
    size?: 'md' | 'lg' | 'xl' | 'xxl';
    fullWidth?: boolean;
    icon?: string;
  }>(),
  {
    disabled: false,
    size: 'lg',
    fullWidth: false,
    icon: undefined,
  }
);

const sizeMap = {
  md: { wrapper: 'text-sm gap-2', inner: 'gap-2', icon: 18 },
  lg: { wrapper: 'text-[15px] gap-3', inner: 'gap-3', icon: 22 },
  xl: { wrapper: 'text-[17px] gap-4', inner: 'gap-4', icon: 28 },
  xxl: { wrapper: 'text-[19px] gap-5', inner: 'gap-5', icon: 34 },
};

const sizeClasses = computed(() => sizeMap[props.size]);
</script>
<style scoped>
@keyframes glowPulse {
  0%,
  100% {
    transform: scale(1);
    filter: drop-shadow(0 0 4px rgba(16, 185, 129, 0.6))
      drop-shadow(0 0 8px rgba(16, 185, 129, 0.35));
  }
  50% {
    transform: scale(1.14);
    filter: drop-shadow(0 0 8px rgba(16, 185, 129, 0.9))
      drop-shadow(0 0 18px rgba(16, 185, 129, 0.55));
  }
}
@keyframes glowAura {
  0%,
  100% {
    transform: scale(1);
    opacity: 0.55;
  }
  50% {
    transform: scale(1.35);
    opacity: 0.25;
  }
}
.animate-glow-pulse {
  animation: glowPulse 2.2s ease-in-out infinite;
}
.animate-glow-aura {
  animation: glowAura 2.2s ease-in-out infinite;
}
</style>
