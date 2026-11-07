<script setup lang="ts">
import { useId, useTemplateRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { X } from 'lucide-vue-next'
import LoginForm from './LoginForm.vue'

const emit = defineEmits<{ relogged: [] }>()
const { t } = useI18n()
const id = useId()
const dialog = useTemplateRef<HTMLDialogElement>('dialog')
let opener: HTMLElement | null = null

function open() {
  opener = document.activeElement as HTMLElement | null
  const d = dialog.value
  // jsdom has no showModal; the native modal traps focus and closes on Esc.
  if (d?.showModal) d.showModal()
  else d?.setAttribute('open', '')
}

function close() {
  const d = dialog.value
  if (d?.close) d.close()
  else d?.removeAttribute('open')
}

function done() {
  close()
  emit('relogged')
}
</script>

<template>
  <div>
    <div
      role="alert"
      class="flex flex-wrap items-center gap-3 rounded-[10px] bg-(--raised) px-3 py-2 text-[15px]"
    >
      <span>{{ t('admin.sessionExpired') }}</span>
      <button type="button" class="btn" @click="open">{{ t('admin.relogin') }}</button>
    </div>
    <dialog
      ref="dialog"
      :aria-labelledby="`${id}-title`"
      class="m-auto w-[min(400px,calc(100vw-32px))] rounded-[16px] border border-(--line) bg-(--card) p-6 text-(--text) backdrop:bg-black/50"
      @close="opener?.focus()"
    >
      <div class="mb-4 flex items-center justify-between gap-3">
        <h2 :id="`${id}-title`" class="text-[22px] font-medium">{{ t('login.title') }}</h2>
        <button
          type="button"
          class="grid h-11 w-11 place-items-center rounded-[10px] transition-colors hover:bg-(--raised)"
          :aria-label="t('admin.close')"
          @click="close"
        >
          <X :size="18" :stroke-width="1.5" aria-hidden="true" />
        </button>
      </div>
      <LoginForm @success="done" />
    </dialog>
  </div>
</template>
