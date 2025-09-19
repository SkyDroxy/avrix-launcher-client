<template>
  <div :class="wrapperClasses">
    <Icon
      name="mingcute:search-2-fill"
      :width="iconSize"
      class="text-neutral-400 group-focus-within:text-indigo-300"
    />
    <input
      ref="inp"
      v-model="inner"
      type="text"
      :placeholder="placeholder"
      :class="inputClasses"
      :spellcheck="false"
      @keydown.enter="emit('enter', inner)"
    />
    <UiButton
      v-if="clearable && inner"
      size="xs"
      variant="ghost"
      class="!p-0 text-neutral-400 hover:text-neutral-200"
      :focus-ring="false"
      :press="false"
      :border="false"
      :title="clearTitle"
      @click="clear"
    >
      <Icon name="mingcute:close-circle-fill" :width="iconSize" />
    </UiButton>
  </div>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import { ref, watch, computed, onMounted } from 'vue';

import Icon from '@/components/common/Icon.vue';

interface Props {
  modelValue?: string;
  placeholder?: string;
  dense?: boolean;
  clearable?: boolean;
  clearTitle?: string;
  autoFocus?: boolean;
  size?: 'xs' | 'sm' | 'md' | 'lg';
  widthClass?: string; // override width if needed
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  placeholder: 'recherche',
  dense: true,
  clearable: true,
  clearTitle: 'Effacer',
  autoFocus: false,
  size: 'md',
  widthClass: 'w-48',
});

const emit = defineEmits<{
  (e: 'update:modelValue', v: string): void;
  (e: 'enter', v: string): void;
  (e: 'clear'): void;
}>();

const inp = ref<HTMLInputElement | null>(null);
const inner = ref(props.modelValue);

watch(
  () => props.modelValue,
  (v) => {
    if (v !== inner.value) inner.value = v;
  }
);
watch(inner, (v) => emit('update:modelValue', v));

function clear() {
  inner.value = '';
  emit('clear');
  requestAnimationFrame(() => inp.value?.focus());
}

onMounted(() => {
  if (props.autoFocus) inp.value?.focus();
});

const baseHeight = computed(() => (props.size === 'xs' ? 'h-6 text-[11px]' : 'h-7 text-[12px]'));
const iconSize = computed(() => (props.size === 'xs' ? 14 : 16));

const wrapperClasses = computed(() =>
  [
    'group flex items-center gap-1 pl-1 pr-1.5 rounded border bg-neutral-700/40 border-neutral-600/70',
    'focus-within:border-indigo-400/70 focus-within:bg-neutral-700/70 transition',
    baseHeight.value,
    props.widthClass,
  ].join(' ')
);
const inputClasses = computed(() =>
  [
    'bg-transparent outline-none placeholder-neutral-500 w-full',
    props.size === 'xs' ? 'text-[11px]' : 'text-[12px]',
  ].join(' ')
);
</script>

<style scoped></style>
