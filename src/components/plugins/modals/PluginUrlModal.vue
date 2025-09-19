<template>
  <BaseModal
    :model-value="modelValue"
    title="Installer via URL"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <form class="space-y-4 pt-1" @submit.prevent="submit">
      <div class="space-y-1">
        <label class="text-[11px] uppercase tracking-wide font-medium text-neutral-300"
          >URL du plugin</label
        >
        <input
          ref="inputEl"
          v-model="localUrl"
          type="text"
          placeholder="https://exemple.com/mon-plugin.jar"
          class="w-full text-[12px] px-2.5 py-2 rounded-md bg-neutral-700/60 border border-neutral-600/70 focus:outline-none focus:ring-2 focus:ring-indigo-400/70 focus:border-indigo-300 placeholder-neutral-500"
        />
        <p v-if="localUrl && !isLikelyJar" class="text-[10px] text-amber-400/80">
          L'URL ne se termine pas par .jar (vérifiez).
        </p>
      </div>
      <div class="flex justify-between items-center gap-3 pt-1">
        <div class="text-[10px] text-neutral-400/80" v-if="hint">{{ hint }}</div>
        <div class="flex ml-auto gap-2">
          <UiButton type="button" variant="ghost" @click="close">Annuler</UiButton>
          <UiButton
            type="submit"
            variant="primary"
            :disabled="loading || !isValid"
            :loading="loading"
          >
            Valider
          </UiButton>
        </div>
      </div>
    </form>
  </BaseModal>
</template>
<script setup lang="ts">
import UiButton from '@components/ui/buttons/UiButton.vue';
import BaseModal from '@components/ui/overlays/BaseModal.vue';
import { ref, watch, computed, onMounted, nextTick } from 'vue';

const props = defineProps<{ modelValue: boolean; url: string; loading?: boolean }>();
const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
  (e: 'update:url', v: string): void;
  (e: 'validate'): void;
}>();

const localUrl = ref(props.url.trim());
const inputEl = ref<HTMLInputElement | null>(null);
onMounted(() => {
  if (props.modelValue) focusInputSoon();
});
watch(
  () => props.modelValue,
  (v) => {
    if (v) focusInputSoon();
  }
);
function focusInputSoon() {
  nextTick(() => inputEl.value?.focus());
}
watch(
  () => props.url,
  (v) => {
    if (v !== localUrl.value) localUrl.value = v;
  }
);
watch(localUrl, (v) => emit('update:url', v.trim()));

const isLikelyJar = computed(() => /\.jar($|\?)/i.test(localUrl.value));
const isValid = computed(() => !!localUrl.value.trim() && /^https?:\/\//i.test(localUrl.value));
const hint = computed(() =>
  !localUrl.value
    ? ''
    : !/^https?:/i.test(localUrl.value)
      ? 'Doit commencer par http:// ou https://'
      : ''
);

function close() {
  emit('update:modelValue', false);
}
function validate() {
  if (!isValid.value) return;
  emit('validate');
}
function submit() {
  validate();
}
</script>
