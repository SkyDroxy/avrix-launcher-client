import { ref } from 'vue';

export interface Toast {
  id: number;
  type: 'info' | 'success' | 'error' | 'warn';
  message: string;
  title?: string;
  meta?: Record<string, string | number | boolean>;
  timeout?: number;
}

const toasts = ref<Toast[]>([]);
let idCounter = 1;

function push(t: Omit<Toast, 'id'>) {
  const id = idCounter++;
  const toast: Toast = { timeout: 5000, ...t, id };
  toasts.value.push(toast);
  if (toast.timeout && toast.timeout > 0) {
    setTimeout(() => remove(id), toast.timeout + 50);
  }
  return id;
}
function info(message: string, opts: Partial<Omit<Toast, 'id' | 'type' | 'message'>> = {}) {
  return push({ type: 'info', message, ...opts });
}
function success(message: string, opts: Partial<Omit<Toast, 'id' | 'type' | 'message'>> = {}) {
  return push({ type: 'success', message, ...opts });
}
function error(message: string, opts: Partial<Omit<Toast, 'id' | 'type' | 'message'>> = {}) {
  return push({ type: 'error', message, ...opts });
}
function warn(message: string, opts: Partial<Omit<Toast, 'id' | 'type' | 'message'>> = {}) {
  return push({ type: 'warn', message, ...opts });
}
function remove(id: number) {
  toasts.value = toasts.value.filter((t) => t.id !== id);
}
function clear() {
  toasts.value = [];
}

export function useToasts() {
  return { toasts, push, info, success, error, warn, remove, clear };
}
