<template>
  <aside class="rounded-lg border border-neutral-600 p-3 bg-neutral-800/40 text-[12px] space-y-2">
    <div class="text-[11px] uppercase tracking-wide opacity-70 font-semibold">Environment</div>
    <div class="grid gap-2">
      <UiButton
        v-for="opt in items"
        :key="String(opt.value)"
        variant="ghost"
        size="sm"
        class="justify-start w-full"
        :class="{
          'bg-neutral-800/60 border border-neutral-600': modelValue === opt.value,
        }"
        :title="opt.label"
        @click="$emit('update:modelValue', opt.value)"
      >
        <div class="flex items-center gap-2">
          <Icon :name="iconFor(opt.value)" :width="16" class="opacity-80" />
          <span>{{ opt.label }}</span>
        </div>
      </UiButton>
    </div>
  </aside>
</template>
<script setup lang="ts">
import { Environment } from '@common/enums/Environment';
import UiButton from '@components/ui/buttons/UiButton.vue';

import Icon from '@/components/common/Icon.vue';

type EnvValue = 'all' | Environment;

const props = defineProps<{
  modelValue: EnvValue;
  items: Array<{ value: EnvValue; label: string }>;
}>();

defineEmits<{ (e: 'update:modelValue', v: EnvValue): void }>();

function iconFor(v: EnvValue) {
  if (v === 'all') return 'mingcute:components-fill';
  if (v === Environment.CLIENT) return 'mingcute:computer-fill';
  if (v === Environment.SERVER) return 'mingcute:server-fill';
  return 'mingcute:link-2-fill'; // BOTH
}
</script>
