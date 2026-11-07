<script setup lang="ts">
import { ref, useId } from 'vue'
import { useI18n } from 'vue-i18n'
import { Eye, EyeOff } from 'lucide-vue-next'
import { login } from '@/api/auth'

const emit = defineEmits<{ success: [] }>()
const { t } = useI18n()
const id = useId()
const username = ref('')
const password = ref('')
const show = ref(false)
const error = ref('')
const busy = ref(false)

function message(e: unknown) {
  const r = (e as { response?: { status?: number; headers?: Record<string, unknown> } }).response
  if (r?.status === 401) return t('login.wrong')
  if (r?.status === 429) {
    const s = Number(r.headers?.['retry-after'])
    return t('login.tooMany', { n: Math.max(1, Math.ceil((s > 0 ? s : 60) / 60)) })
  }
  return t('login.failed')
}

async function submit() {
  if (busy.value) return
  busy.value = true
  error.value = ''
  try {
    await login(username.value, password.value)
    password.value = ''
    emit('success')
  } catch (e) {
    error.value = message(e)
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <form class="grid gap-4" @submit.prevent="submit">
    <p v-if="error" role="alert" class="rounded-[10px] bg-(--raised) px-3 py-2 text-[15px]">
      {{ error }}
    </p>
    <div class="grid gap-1">
      <label :for="`${id}-user`" class="text-[14px] font-medium">{{ t('login.username') }}</label>
      <input
        :id="`${id}-user`"
        v-model="username"
        class="field"
        autocomplete="username"
        autocapitalize="none"
        required
        autofocus
      />
    </div>
    <div class="grid gap-1">
      <label :for="`${id}-pass`" class="text-[14px] font-medium">{{ t('login.password') }}</label>
      <div class="relative">
        <input
          :id="`${id}-pass`"
          v-model="password"
          :type="show ? 'text' : 'password'"
          class="field pr-12"
          autocomplete="current-password"
          required
        />
        <button
          type="button"
          class="absolute inset-y-0 right-0 grid w-11 place-items-center rounded-[10px] text-(--text-2) transition-colors hover:text-(--text)"
          :aria-label="show ? t('login.hidePassword') : t('login.showPassword')"
          :aria-pressed="show"
          @click="show = !show"
        >
          <EyeOff v-if="show" :size="18" :stroke-width="1.5" aria-hidden="true" />
          <Eye v-else :size="18" :stroke-width="1.5" aria-hidden="true" />
        </button>
      </div>
    </div>
    <button type="submit" class="btn-primary mt-2" :disabled="busy">{{ t('login.submit') }}</button>
  </form>
</template>
