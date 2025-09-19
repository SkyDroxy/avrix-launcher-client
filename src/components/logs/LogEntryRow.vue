<template>
  <div
    class="font-mono text-[11px] leading-snug whitespace-pre-wrap break-all"
    :class="lineClass"
    :title="raw"
  >
    {{ raw }}
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue';

import type { LogLevel } from '@interfaces/logs';

const props = defineProps<{ raw: string }>();

function extractLevel(line: string): LogLevel {
  if (/\bERROR\b/.test(line)) return 'ERROR';
  if (/\bWARN\b/.test(line)) return 'WARN';
  if (/\bDEBUG\b/.test(line)) return 'DEBUG';
  return 'INFO';
}

const lineClass = computed(() => {
  switch (extractLevel(props.raw)) {
    case 'ERROR':
      return 'text-red-300';
    case 'WARN':
      return 'text-amber-200';
    case 'DEBUG':
      return 'text-fuchsia-300';
    default:
      return 'text-neutral-200';
  }
});
</script>
