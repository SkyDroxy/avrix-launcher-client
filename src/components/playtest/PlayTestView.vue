<template>
  <div class="space-y-6 text-[12px]">
    <h1 class="text-lg font-semibold flex items-center gap-2">
      <Icon name="mingcute:test-tube-fill" :width="20" />
      <span>PlayTest (DEV uniquement)</span>
      <span
        class="ml-2 px-2 py-0.5 rounded bg-accent-600/30 border border-accent-500/40 text-accent-200 text-[10px] tracking-wide"
      >DEV</span>
    </h1>

    <section class="space-y-2">
      <h2 class="font-semibold text-sm">UI Buttons</h2>
      <div class="flex flex-wrap gap-2">
        <UiButton variant="primary" size="sm" icon="mingcute:check-fill">Primary</UiButton>
        <UiButton variant="secondary" size="sm" icon="mingcute:information-fill">
          Secondary
        </UiButton>
        <UiButton variant="ghost" size="sm" icon="mingcute:information-fill">Ghost</UiButton>
        <UiButton variant="info" size="sm" icon="mingcute:information-fill">Info</UiButton>
        <UiButton variant="success" size="sm" icon="mingcute:check-circle-fill">Success</UiButton>
        <UiButton variant="warning" size="sm" icon="mingcute:warning-fill">Warning</UiButton>
        <UiButton variant="danger" size="sm" icon="mingcute:close-circle-fill">Danger</UiButton>
        <UiButton variant="accent" size="sm" icon="mingcute:close-circle-fill">Accent</UiButton>
      </div>
    </section>

    <section class="space-y-2">
      <h2 class="font-semibold text-sm">Toasts</h2>
      <div class="flex flex-wrap gap-2">
        <UiButton
          v-for="b in toastButtons"
          :key="b.label"
          :variant="
            b.variant === 'error'
              ? 'danger'
              : b.variant === 'success'
                ? 'success'
                : b.variant === 'info'
                  ? 'primary'
                  : 'warning'
          "
          size="xs"
          @click="b.run"
        >
          {{ b.label }}
        </UiButton>
      </div>
    </section>

    <section class="space-y-2">
      <h2 class="font-semibold text-sm">Composants principales</h2>
      <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
        <div class="p-3 rounded-md border border-neutral-700/70 bg-neutral-800/60">
          <h3 class="font-medium mb-2 flex items-center gap-2 text-neutral-300">
            <Icon name="mingcute:plugin-2-fill" :width="16" /> Plugin Card (exemple)
          </h3>
          <div class="text-xs opacity-60 mb-2">Prévisualisation statique d'une carte plugin.</div>
          <div
            class="rounded-md border border-neutral-700 bg-neutral-900/50 p-2 space-y-1 text-[11px]"
          >
            <div class="font-semibold">Sample Plugin</div>
            <div class="flex items-center justify-between opacity-70 tabular-nums">
              <span class="px-1 py-0.5 rounded bg-neutral-700/50">v1.2.3</span>
              <span title="01/01/2025 12:34:56">01/01/2025</span>
              <span class="opacity-60">128 KB</span>
            </div>
            <div class="flex items-center justify-between opacity-60 text-[10px]">
              <span class="truncate" title="AuteurX">AuteurX</span>
              <span
                class="px-1 py-0.5 rounded bg-neutral-700/60 text-[10px] uppercase tracking-wide"
              >both</span>
            </div>
            <div class="flex items-center justify-between text-[10px] opacity-50">
              <span title="GPL">GPL</span>
              <span title="sample-plugin" class="truncate">sample-plugin</span>
            </div>
            <div class="text-[10px] mt-1 text-neutral-300 leading-snug">
              Une description courte de démonstration pour le plugin.
            </div>
          </div>
        </div>
        <div class="p-3 rounded-md border border-neutral-700/70 bg-neutral-800/60">
          <h3 class="font-medium mb-2 flex items-center gap-2 text-neutral-300">
            <Icon name="mingcute:terminal-box-fill" :width="16" /> Logs (extrait live)
            <UiButton
              :border="false"
              variant="ghost"
              size="xs"
              square
              class="ml-auto text-neutral-400 hover:text-neutral-200"
              title="Vider extrait"
              icon="mingcute:delete-2-fill"
              @click="clearLogs"
            />
          </h3>
          <ul class="text-[11px] leading-tight max-h-32 overflow-auto custom-scrollbar pr-1">
            <li v-for="l in liveExcerpt" :key="l.id" class="flex items-start gap-1">
              <span
                class="uppercase tracking-wide text-[9px] font-semibold"
                :class="logCls(l.level)"
              >{{ l.level }}</span>
              <span class="opacity-60">[{{ l.source }}]</span>
              <span class="flex-1 break-words" :class="logCls(l.level)">{{ l.message }}</span>
            </li>
            <li v-if="!liveExcerpt.length" class="opacity-50 italic">(aucun log)</li>
          </ul>
        </div>
        <div class="p-3 rounded-md border border-neutral-700/70 bg-neutral-800/60">
          <h3 class="font-medium mb-2 flex items-center gap-2 text-neutral-300">
            <Icon name="mingcute:settings-5-fill" :width="16" /> Couleurs UI
          </h3>
          <div class="grid grid-cols-2 gap-2 text-[10px]">
            <div v-for="c in colors" :key="c.name" class="flex items-center gap-2">
              <div :class="'w-5 h-5 rounded border border-neutral-700 ' + c.bg"></div>
              <div>{{ c.name }}</div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Inputs & Form controls -->
    <section class="space-y-2">
      <h2 class="font-semibold text-sm">Champs & Contrôles</h2>
      <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
        <div class="p-3 rounded-md border border-neutral-700/70 bg-neutral-800/60 space-y-3">
          <h3 class="font-medium flex items-center gap-2 text-neutral-300 text-[12px]">
            <Icon name="mingcute:textbox-fill" :width="16" /> UiInput
          </h3>
          <UiInput v-model="demoText" placeholder="Texte..." size="sm" />
          <UiInput v-model="demoText" placeholder="Dense" dense size="sm" />
          <UiInput
            v-model="demoText"
            placeholder="Avec icône"
            icon-left="mingcute:search-2-fill"
            icon-right="mingcute:close-circle-fill"
            size="sm"
          />
        </div>
        <div class="p-3 rounded-md border border-neutral-700/70 bg-neutral-800/60 space-y-3">
          <h3 class="font-medium flex items-center gap-2 text-neutral-300 text-[12px]">
            <Icon name="mingcute:search-2-fill" :width="16" /> UiSearchInput
          </h3>
          <UiSearchInput v-model="demoSearch" placeholder="Rechercher..." />
          <UiSearchInput v-model="demoSearch" placeholder="Dense" dense />
        </div>
        <div class="p-3 rounded-md border border-neutral-700/70 bg-neutral-800/60 space-y-3">
          <h3 class="font-medium flex items-center gap-2 text-neutral-300 text-[12px]">
            <Icon name="mingcute:check-circle-fill" :width="16" /> UiCheckbox & Switch
          </h3>
          <div class="flex flex-col gap-2">
            <UiCheckbox v-model="cb1" size="xs">XS Checkbox</UiCheckbox>
            <UiCheckbox v-model="cb2" size="sm">SM Checkbox</UiCheckbox>
            <UiCheckbox v-model="cb3" size="md" indeterminate>Indeterminate</UiCheckbox>
            <UiCheckbox v-model="sw1" switch size="sm">Switch sm</UiCheckbox>
            <UiCheckbox v-model="sw2" switch size="md">Switch md</UiCheckbox>
            <UiCheckbox v-model="sw3" switch size="lg" color="emerald">Switch lg</UiCheckbox>
          </div>
        </div>
        <div class="p-3 rounded-md border border-neutral-700/70 bg-neutral-800/60 space-y-3">
          <h3 class="font-medium flex items-center gap-2 text-neutral-300 text-[12px]">
            <Icon name="mingcute:bookmark-fill" :width="16" /> UiBadge
          </h3>
          <div class="flex flex-wrap gap-2 text-[10px]">
            <UiBadge variant="neutral">Neutral</UiBadge>
            <UiBadge variant="indigo">Indigo</UiBadge>
            <UiBadge variant="green">Green</UiBadge>
            <UiBadge variant="amber">Amber</UiBadge>
            <UiBadge variant="red">Red</UiBadge>
            <UiBadge variant="core">Core</UiBadge>
            <UiBadge variant="info">Info</UiBadge>
            <UiBadge variant="success">Success</UiBadge>
            <UiBadge variant="warning">Warning</UiBadge>
            <UiBadge variant="danger">Danger</UiBadge>
            <UiBadge variant="accent">Accent</UiBadge>
          </div>
        </div>
      </div>
    </section>

    <section class="space-y-2">
      <h2 class="font-semibold text-sm">Actions Mock</h2>
      <div class="flex flex-wrap gap-2">
        <UiButton variant="primary" size="xs" @click="emitLog('info')">Log Info</UiButton>
        <UiButton variant="success" size="xs" @click="emitLog('success')">Log Success</UiButton>
        <UiButton variant="warning" size="xs" @click="emitLog('warn')">Log Warn</UiButton>
        <UiButton variant="danger" size="xs" @click="emitLog('error')">Log Error</UiButton>
      </div>
    </section>
  </div>
</template>
<script setup lang="ts">
import UiBadge from '@components/ui/buttons/UiBadge.vue';
import UiButton from '@components/ui/buttons/UiButton.vue';
import UiCheckbox from '@components/ui/input/UiCheckbox.vue';
import UiInput from '@components/ui/input/UiInput.vue';
import UiSearchInput from '@components/ui/input/UiSearchInput.vue';
import { useLogs } from '@composables/useLogs';
import { useToasts } from '@composables/useToasts';
import { computed, ref } from 'vue';

import Icon from '@/components/common/Icon.vue';

const { success, info, error, warn } = useToasts();
const { addLog, entries: logEntries, clearLogs: clearAllLogs } = useLogs();

const toastButtons = [
  { label: 'Success', variant: 'success', run: () => success('Toast succès test') },
  { label: 'Info', variant: 'info', run: () => info('Toast info test') },
  { label: 'Erreur', variant: 'error', run: () => error('Toast erreur test') },
  { label: 'Warn', variant: 'warn', run: () => warn('Toast warn test') },
];

// Live excerpt (dernier N logs)
const EXCERPT_MAX = 25;
const liveExcerpt = computed(() => logEntries.value.slice(-EXCERPT_MAX));
function logCls(level: string) {
  switch (level) {
    case 'success':
      return 'text-emerald-400';
    case 'warn':
      return 'text-amber-400';
    case 'error':
      return 'text-rose-400';
    default:
      return 'text-neutral-300';
  }
}

const colors = [
  { name: 'Primary', bg: 'bg-primary-600' },
  { name: 'Secondary', bg: 'bg-secondary-600' },
  { name: 'Ghost', bg: 'bg-neutral-700/70' },
  { name: 'Info', bg: 'bg-info-600' },
  { name: 'Accent', bg: 'bg-accent-600' },
  { name: 'Warn', bg: 'bg-warn-500' },
  { name: 'Success', bg: 'bg-success-600' },
  { name: 'Error', bg: 'bg-error-600' },
];

function emitLog(level: 'info' | 'success' | 'warn' | 'error') {
  addLog({ level, source: 'playtest', message: `Message ${level} test` });
}
function clearLogs() {
  clearAllLogs();
}

// Demo state
const demoText = ref('');
const demoSearch = ref('');
const cb1 = ref(false);
const cb2 = ref(true);
const cb3 = ref(false);
const sw1 = ref(true);
const sw2 = ref(false);
const sw3 = ref(true);
</script>
<style scoped>
/* Additional scoped styles if needed */
</style>
