<template>
  <nav class="flex items-center gap-1 text-[11px] truncate" aria-label="Breadcrumb">
    <template v-for="(seg, idx) in segments" :key="seg.fullPath">
      <span
        v-if="idx < segments.length - 1"
        class="flex items-center gap-1 opacity-70 hover:opacity-90 cursor-pointer transition"
        @click="go(seg.fullPath)"
      >
        <span class="truncate max-w-[110px]">{{ seg.label }}</span>
        <span class="opacity-40">/</span>
      </span>
      <span v-else class="font-medium opacity-90 truncate max-w-[140px]" :title="seg.label">{{
        seg.label
      }}</span>
    </template>
  </nav>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';

interface Segment {
  label: string;
  fullPath: string;
}
const route = useRoute();
const router = useRouter();

function labelOf(r: any): string {
  if (r.meta && r.meta.label) return String(r.meta.label);
  if (r.name) return String(r.name);
  return r.path === '/' ? 'Accueil' : r.path.split('/').filter(Boolean).pop() || '—';
}

const segments = computed<Segment[]>(() => {
  const matched = route.matched || [];
  return matched.map((m) => ({ label: labelOf(m), fullPath: m.path || '/' }));
});

function go(p: string) {
  router.push(p).catch(() => {});
}
</script>
<style scoped>
nav {
  user-select: none;
}
</style>
