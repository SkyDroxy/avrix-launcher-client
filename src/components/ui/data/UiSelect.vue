<template>
  <div class="relative inline-block text-[11px]" ref="root" @keydown.stop.prevent="onKey($event)">
    <button
      type="button"
      class="flex items-center gap-1 pl-2 pr-6 h-7 rounded-md border border-neutral-700/70 bg-neutral-800/70 text-neutral-200 hover:bg-neutral-700/70 focus:outline-none focus:ring-2 focus:ring-indigo-500/50 focus:ring-offset-0 transition select-none relative"
      :class="disabled ? 'opacity-50 cursor-not-allowed' : ''"
      :disabled="disabled"
      @click="toggle"
      :aria-expanded="open ? 'true' : 'false'"
      :aria-disabled="disabled ? 'true' : 'false'"
    >
      <span v-if="label" class="opacity-60 tracking-wide">{{ label }}</span>
      <span class="font-medium truncate">{{ activeOption?.label || placeholder }}</span>
      <Icon
        :name="open ? 'mingcute:up-line' : 'mingcute:down-line'"
        :width="14"
        class="opacity-60 absolute right-1 top-1/2 -translate-y-1/2 pointer-events-none transition-transform"
        :class="{ 'rotate-180': open }"
      />
    </button>
    <transition name="fade-scale">
      <ul
        v-if="open"
        class="absolute z-50 mt-1 min-w-[160px] max-h-60 overflow-auto rounded-md border border-neutral-700/70 bg-neutral-900/95 backdrop-blur-sm shadow-[0_4px_20px_-4px_rgba(0,0,0,0.6)] py-1 focus:outline-none select-none ring-1 ring-black/40"
        role="listbox"
        :aria-activedescendant="activeId"
        @mousedown.prevent
      >
        <li
          v-for="(opt, idx) in options"
          :key="opt.value"
          :id="idPrefix + '-' + idx"
          @click="select(opt)"
          class="px-3 py-1.5 cursor-pointer flex items-center justify-between gap-3 text-[11px]"
          :class="[
            idx === focusIndex
              ? 'bg-indigo-600/20 text-neutral-100'
              : 'text-neutral-300 hover:bg-neutral-700/40',
            modelValue === opt.value && idx !== focusIndex ? 'text-indigo-300' : '',
          ]"
          role="option"
          :aria-selected="modelValue === opt.value"
        >
          <span class="truncate">{{ opt.label }}</span>
          <Icon
            v-if="modelValue === opt.value"
            name="mingcute:check-fill"
            :width="14"
            class="text-indigo-300 opacity-90"
          />
        </li>
      </ul>
    </transition>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, computed } from 'vue';

import Icon from '@/components/common/Icon.vue';

interface Option {
  value: string;
  label: string;
}

const props = withDefaults(
  defineProps<{
    modelValue: string;
    options: Option[];
    label?: string;
    placeholder?: string;
    disabled?: boolean;
  }>(),
  { disabled: false }
);
const emit = defineEmits<{ (e: 'update:modelValue', v: string): void }>();

const open = ref(false);
const root = ref<HTMLElement | null>(null);
const focusIndex = ref(-1);
const idPrefix = 'sel-' + Math.random().toString(36).slice(2);

const activeOption = computed(() => props.options.find((o) => o.value === props.modelValue));
const activeId = computed(() => {
  const idx = props.options.findIndex((o) => o.value === props.modelValue);
  return idx >= 0 ? idPrefix + '-' + idx : undefined;
});

function toggle() {
  if (props.disabled) return;
  open.value = !open.value;
  if (open.value) {
    setTimeout(() => {
      const idx = props.options.findIndex((o) => o.value === props.modelValue);
      focusIndex.value = idx >= 0 ? idx : 0;
      scrollFocusedIntoView();
    }, 0);
  }
}
function close() {
  open.value = false;
}
function select(opt: Option) {
  emit('update:modelValue', opt.value);
  close();
}
function onClickOutside(e: Event) {
  if (!root.value) return;
  const target = e.target as Node | null;
  if (!target) return;
  if (!root.value.contains(target)) {
    close();
  }
}
function onKey(e: KeyboardEvent) {
  if (props.disabled) return;
  if (!open.value && (e.key === 'ArrowDown' || e.key === 'Enter' || e.key === ' ')) {
    open.value = true;
    e.preventDefault();
    setTimeout(() => scrollFocusedIntoView(), 0);
    return;
  }
  if (!open.value) return;
  switch (e.key) {
    case 'Escape':
      close();
      break;
    case 'ArrowDown':
      move(1);
      break;
    case 'ArrowUp':
      move(-1);
      break;
    case 'Home':
      focusIndex.value = 0;
      scrollFocusedIntoView();
      break;
    case 'End':
      focusIndex.value = props.options.length - 1;
      scrollFocusedIntoView();
      break;
    case 'Enter':
    case ' ':
      if (focusIndex.value >= 0) select(props.options[focusIndex.value]);
      break;
  }
}
function move(delta: number) {
  const len = props.options.length;
  if (len === 0) return;
  let idx = focusIndex.value + delta;
  if (idx < 0) idx = len - 1;
  else if (idx >= len) idx = 0;
  focusIndex.value = idx;
  scrollFocusedIntoView();
}
function scrollFocusedIntoView() {
  if (!open.value) return;
  requestAnimationFrame(() => {
    const menu = root.value?.querySelector('ul');
    const id = idPrefix + '-' + focusIndex.value;
    const el = menu?.querySelector('#' + id) as HTMLElement | null;
    if (el && menu) {
      const top = el.offsetTop;
      const bottom = top + el.offsetHeight;
      if (top < menu.scrollTop) menu.scrollTop = top;
      else if (bottom > menu.scrollTop + menu.clientHeight)
        menu.scrollTop = bottom - menu.clientHeight;
    }
  });
}

watch(
  () => props.modelValue,
  () => {
    if (open.value) {
      const idx = props.options.findIndex((o) => o.value === props.modelValue);
      focusIndex.value = idx;
    }
  }
);

onMounted(() => {
  document.addEventListener('mousedown', onClickOutside as EventListener);
  document.addEventListener('focusin', onClickOutside as EventListener);
});
onBeforeUnmount(() => {
  document.removeEventListener('mousedown', onClickOutside as EventListener);
  document.removeEventListener('focusin', onClickOutside as EventListener);
});
</script>
<style scoped>
.fade-scale-enter-active,
.fade-scale-leave-active {
  transition:
    opacity 0.12s ease,
    transform 0.12s ease;
}
.fade-scale-enter-from,
.fade-scale-leave-to {
  opacity: 0;
  transform: translateY(-4px) scale(0.97);
}
</style>
