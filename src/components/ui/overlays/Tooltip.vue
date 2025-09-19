<template>
  <span
    ref="triggerRef"
    :class="['relative', blockTarget ? 'block w-full' : 'inline-flex']"
    @mouseenter="onEnter"
    @mouseleave="onLeave"
    @focusin="onEnter"
    @focusout="onLeave"
  >
    <slot />
    <teleport to="body" v-if="mounted">
      <transition name="avx-tooltip-fade">
        <div
          v-if="visible"
          ref="tooltipRef"
          class="pointer-events-none z-[1000] fixed"
          :style="styleObject"
        >
          <div
            class="rounded-md px-2.5 py-1.5 text-[11px] font-medium leading-snug shadow-lg border bg-neutral-800/95 backdrop-blur-sm border-neutral-600/60 text-neutral-50 flex items-center gap-1"
          >
            <slot name="icon" />
            <span class="whitespace-nowrap"><slot name="content">{{ text }}</slot></span>
          </div>
        </div>
      </transition>
    </teleport>
  </span>
</template>
<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount, nextTick, computed } from 'vue';

interface Props {
  text?: string;
  placement?: 'auto' | 'top' | 'bottom' | 'left' | 'right';
  openDelay?: number; // ms
  closeDelay?: number; // ms
  blockTarget?: boolean; // when true wrapper becomes block w-full (for buttons with block style)
}
const props = withDefaults(defineProps<Props>(), {
  text: '',
  placement: 'auto',
  openDelay: 140,
  closeDelay: 80,
  blockTarget: false,
});

const triggerRef = ref<HTMLElement | null>(null);
const tooltipRef = ref<HTMLElement | null>(null);
const visible = ref(false);
const mounted = ref(false);
let openTimer: number | null = null;
let closeTimer: number | null = null;

function clearTimers() {
  if (openTimer) window.clearTimeout(openTimer);
  if (closeTimer) window.clearTimeout(closeTimer);
  openTimer = null;
  closeTimer = null;
}

function onEnter() {
  clearTimers();
  openTimer = window.setTimeout(() => {
    visible.value = true;
    nextTick(updatePosition);
  }, props.openDelay);
}
function onLeave() {
  clearTimers();
  closeTimer = window.setTimeout(() => (visible.value = false), props.closeDelay);
}

const styleObject = computed(() => ({
  top: position.value.top + 'px',
  left: position.value.left + 'px',
  transform: 'translate(-50%, -50%)',
}));

const position = ref({ top: -9999, left: -9999 });

function getAutoPlacement(rect: DOMRect, tRect: DOMRect) {
  const vw = window.innerWidth;
  if (rect.right + tRect.width + 16 < vw) return 'right';
  if (rect.left - tRect.width - 16 > 0) return 'left';
  if (rect.top - tRect.height - 12 > 0) return 'top';
  return 'bottom';
}

function updatePosition() {
  const el = triggerRef.value;
  const tip = tooltipRef.value;
  if (!el || !tip) return;
  const rect = el.getBoundingClientRect();
  const tRect = tip.getBoundingClientRect();
  let place: string = props.placement;
  if (place === 'auto') place = getAutoPlacement(rect, tRect);

  let top = 0;
  let left = 0;
  const gap = 10;
  switch (place) {
    case 'right':
      top = rect.top + rect.height / 2;
      left = rect.right + tRect.width / 2 + gap;
      break;
    case 'left':
      top = rect.top + rect.height / 2;
      left = rect.left - tRect.width / 2 - gap;
      break;
    case 'top':
      top = rect.top - tRect.height / 2 - gap;
      left = rect.left + rect.width / 2;
      break;
    case 'bottom':
    default:
      top = rect.bottom + tRect.height / 2 + gap;
      left = rect.left + rect.width / 2;
      break;
  }
  position.value = { top, left };
}

function onWindowEvents() {
  if (visible.value) updatePosition();
}

onMounted(() => {
  mounted.value = true;
  window.addEventListener('scroll', onWindowEvents, true);
  window.addEventListener('resize', onWindowEvents, true);
});
onBeforeUnmount(() => {
  clearTimers();
  window.removeEventListener('scroll', onWindowEvents, true);
  window.removeEventListener('resize', onWindowEvents, true);
});

watch(visible, (v) => {
  if (v) nextTick(updatePosition);
});
</script>
<style scoped>
.avx-tooltip-fade-enter-active,
.avx-tooltip-fade-leave-active {
  transition:
    opacity 120ms ease,
    transform 120ms ease;
}
.avx-tooltip-fade-enter-from,
.avx-tooltip-fade-leave-to {
  opacity: 0;
  transform: scale(0.92);
}
</style>
