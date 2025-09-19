import { ref } from 'vue';

// Simple global modal controller (module-scoped ref)
const isOpen = ref(false);

export function useSettingsModal() {
  return {
    isOpen,
    open: () => (isOpen.value = true),
    close: () => (isOpen.value = false),
    toggle: () => (isOpen.value = !isOpen.value),
  };
}
