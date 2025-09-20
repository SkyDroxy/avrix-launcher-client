<template>
  <Teleport to="body">
    <transition name="modal-fade-scale">
      <div
        v-if="modelValue"
        class="fixed inset-0 z-50 flex justify-center p-6 md:p-10"
        :class="[containerAlign, outerScrollClass]"
      >
        <!-- Backdrop -->
        <div class="fixed inset-0 modal-backdrop" @click="onBackdrop" />
        <!-- Panel -->
        <div
          ref="panelRef"
          class="relative mt-14 md:mt-4 rounded-xl border modal-panel flex flex-col max-h-[86vh] md:max-h-[88vh] overflow-hidden"
          :class="widthClass"
          :style="panelStyle"
          role="dialog"
          aria-modal="true"
          :aria-labelledby="computedLabelId"
          :aria-describedby="descriptionId"
        >
          <header
            v-if="$slots.title || title"
            class="px-5 pt-4 pb-3 flex items-start gap-2 border-b modal-header backdrop-blur-sm"
          >
            <div
              class="flex-1 font-semibold text-[15px] leading-tight tracking-wide"
              :id="computedLabelId"
            >
              <slot name="title">{{ title }}</slot>
            </div>
            <UiButton
              v-if="closable"
              variant="ghost"
              size="xs"
              square
              :title="closeLabel"
              :aria-label="closeLabel"
              class="modal-close"
              :focus-ring="false"
              :press="false"
              @click="close"
            >
              <Icon name="mingcute:close-line" :width="18" />
            </UiButton>
          </header>
          <div
            class="px-5 py-5 text-[13px] leading-relaxed space-y-4 custom-scrollbar modal-body flex-1 min-h-0"
            :class="props.bodyScrollable ? 'overflow-auto' : 'overflow-hidden'"
          >
            <p
              v-if="$slots.description"
              class="text-[12px] text-neutral-400 -mt-1"
              :id="descriptionId"
            >
              <slot name="description" />
            </p>
            <slot />
          </div>
          <footer
            v-if="$slots.footer"
            class="px-5 py-4 border-t flex justify-end gap-2 text-[12px] modal-footer"
          >
            <slot name="footer" />
          </footer>
        </div>
      </div>
    </transition>
  </Teleport>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import { computed, ref, watch, onMounted, onBeforeUnmount, nextTick } from 'vue';

import Icon from '@/components/common/Icon.vue';

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    title?: string;
    width?: 'sm' | 'md' | 'lg' | 'xl' | '2xl';
    placement?: 'top' | 'center';
    closable?: boolean;
    closeOnBackdrop?: boolean;
    closeLabel?: string;
    bodyScrollable?: boolean;
    panelHeight?: string; // e.g., '70vh' or '600px' for a fixed-height panel
  }>(),
  {
    width: 'sm',
    placement: 'top',
    closable: true,
    closeOnBackdrop: true,
    closeLabel: 'Fermer',
    title: undefined,
    bodyScrollable: true,
    panelHeight: undefined,
  }
);

const emit = defineEmits<{ (e: 'update:modelValue', v: boolean): void; (e: 'close'): void }>();

const widthClass = computed(() => {
  switch (props.width) {
    case 'sm':
      return 'w-full max-w-sm';
    case 'md':
      return 'w-full max-w-md';
    case 'lg':
      return 'w-full max-w-lg';
    case 'xl':
      return 'w-full max-w-xl';
    case '2xl':
      return 'w-full max-w-3xl';
    default:
      return 'w-full max-w-sm';
  }
});

const containerAlign = computed(() =>
  props.placement === 'center' ? 'items-center' : 'items-start'
);

const outerScrollClass = computed(() =>
  props.panelHeight ? 'overflow-hidden' : 'overflow-y-auto'
);

const panelStyle = computed(() => {
  if (props.panelHeight) {
    return { height: props.panelHeight } as Record<string, string>;
  }
  return {} as Record<string, string>;
});

const computedLabelId = `modal-title-${Math.random().toString(36).slice(2)}`;
const descriptionId = `modal-desc-${Math.random().toString(36).slice(2)}`;
const panelRef = ref<HTMLElement | null>(null);
let lastFocused: HTMLElement | null = null;

function storeFocus() {
  lastFocused = document.activeElement as HTMLElement | null;
}
function restoreFocus() {
  if (lastFocused) {
    try {
      lastFocused.focus();
    } catch (_) {}
  }
}

function focusFirst() {
  nextTick(() => {
    const root = panelRef.value;
    if (!root) return;
    const focusable = root.querySelectorAll<HTMLElement>(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    if (focusable.length) focusable[0].focus();
  });
}

function handleKey(e: KeyboardEvent) {
  if (e.key === 'Escape' && props.closable) {
    e.preventDefault();
    close();
  }
  if (e.key === 'Tab') {
    const root = panelRef.value;
    if (!root) return;
    const focusable = Array.from(
      root.querySelectorAll<HTMLElement>(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      )
    ).filter((el) => !el.hasAttribute('disabled'));
    if (!focusable.length) {
      e.preventDefault();
      return;
    }
    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    if (e.shiftKey && document.activeElement === first) {
      e.preventDefault();
      last.focus();
    } else if (!e.shiftKey && document.activeElement === last) {
      e.preventDefault();
      first.focus();
    }
  }
}

watch(
  () => props.modelValue,
  (v) => {
    if (v) {
      storeFocus();
      focusFirst();
      window.addEventListener('keydown', handleKey);
    } else {
      window.removeEventListener('keydown', handleKey);
      restoreFocus();
    }
  }
);
onMounted(() => {
  if (props.modelValue) {
    storeFocus();
    focusFirst();
    window.addEventListener('keydown', handleKey);
  }
});
onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKey);
  restoreFocus();
});

function close() {
  emit('update:modelValue', false);
  emit('close');
}
function onBackdrop() {
  if (props.closeOnBackdrop) close();
}
</script>
<style scoped>
.modal-backdrop {
  /* Stronger backdrop to avoid washed-out light theme */
  background: color-mix(in oklab, black 55%, transparent);
  backdrop-filter: blur(2px);
}

.modal-panel {
  /* Dark theme default */
  background: linear-gradient(180deg, #18191b 0%, #141516 100%);
  color: var(--color-neutral-200);
  border-color: color-mix(in oklab, #2a2b2e 70%, black 30%);
  box-shadow: 0 10px 34px -8px rgba(0, 0, 0, 0.35);
}

.modal-header {
  background: color-mix(in oklab, #0e0f10 60%, transparent);
  border-color: color-mix(in oklab, #2a2b2e 60%, transparent);
}

.modal-footer {
  background: color-mix(in oklab, #0e0f10 40%, transparent);
  border-color: color-mix(in oklab, #2a2b2e 60%, transparent);
}

.modal-close {
  color: var(--color-neutral-400);
}
.modal-close:hover {
  color: var(--color-neutral-200);
  background: color-mix(in oklab, #2a2b2e 40%, transparent);
}

/* Light theme overrides using root data attribute (no dark: classes) */
:global(:root[data-theme='light']) .modal-panel {
  background: #ffffff; /* Opaque */
  color: var(--color-neutral-900);
  border-color: var(--color-neutral-200);
  box-shadow: 0 20px 44px -16px rgba(15, 23, 42, 0.28);
}

:global(:root[data-theme='light']) .modal-header {
  background: #ffffff; /* Opaque header */
  border-color: var(--color-neutral-200);
}

:global(:root[data-theme='light']) .modal-footer {
  background: #fafafa;
  border-color: var(--color-neutral-200);
}

:global(:root[data-theme='light']) .modal-close {
  color: var(--color-neutral-500);
}

:global(:root[data-theme='light']) .modal-close:hover {
  color: var(--color-neutral-800);
  background: color-mix(in oklab, #000 10%, transparent);
}

.modal-fade-scale-enter-active {
  animation: modalIn 0.18s ease forwards;
}
.modal-fade-scale-leave-active {
  animation: modalOut 0.16s ease forwards;
}
@keyframes modalIn {
  0% {
    opacity: 0;
    transform: translateY(10px) scale(0.97);
  }
  100% {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
@keyframes modalOut {
  0% {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  100% {
    opacity: 0;
    transform: translateY(4px) scale(0.97);
  }
}

.custom-scroll {
  scrollbar-width: thin;
}
.custom-scroll::-webkit-scrollbar {
  width: 10px;
}
.custom-scroll::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scroll::-webkit-scrollbar-thumb {
  background: #2d2f33;
  border-radius: 6px;
  border: 2px solid #18191b;
}
.custom-scroll::-webkit-scrollbar-thumb:hover {
  background: #3a3d42;
}
</style>
