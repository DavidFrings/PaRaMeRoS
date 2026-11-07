<script setup lang="ts">
import { nextTick, ref, useTemplateRef } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps<{ label: string; question: string; name?: string }>()
const emit = defineEmits<{ confirm: [] }>()
const { t } = useI18n()
const asking = ref(false)
const trigger = useTemplateRef<HTMLButtonElement>('trigger')
const cancelButton = useTemplateRef<HTMLButtonElement>('cancelButton')

async function ask() {
  asking.value = true
  await nextTick()
  cancelButton.value?.focus()
}

async function answer(yes: boolean) {
  asking.value = false
  if (yes) emit('confirm')
  await nextTick()
  trigger.value?.focus()
}
</script>

<template>
  <span
    v-if="asking"
    role="group"
    :aria-label="props.question"
    class="inline-flex flex-wrap items-center gap-2"
  >
    <span class="text-[14px] text-(--text-2)">{{ props.question }}</span>
    <button type="button" class="btn-primary" @click="answer(true)">{{ props.label }}</button>
    <button ref="cancelButton" type="button" class="btn" @click="answer(false)">
      {{ t('admin.cancel') }}
    </button>
  </span>
  <button
    v-else
    ref="trigger"
    type="button"
    class="btn"
    :aria-label="props.name ? `${props.label}: ${props.name}` : undefined"
    @click="ask"
  >
    {{ props.label }}
  </button>
</template>
