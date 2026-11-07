<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { AuthExpiredError, deleteTournament, getTournament, putTournament } from '@/api/client'
import { berlinToRfc3339, rfc3339ToBerlinLocal } from '@/lib/time'
import AuthExpiredNotice from './AuthExpiredNotice.vue'
import ConfirmButton from './ConfirmButton.vue'

const { t } = useI18n()
const f = reactive({ name: '', place: '', start: '', end: '' })
const exists = ref(false)
const status = ref<string | null>(null)
const error = ref<string | null>(null)
const expired = ref(false)
const busy = ref(false)

onMounted(async () => {
  try {
    const x = await getTournament()
    if (!x) return
    exists.value = true
    Object.assign(f, {
      name: x.name,
      place: x.place ?? '',
      start: rfc3339ToBerlinLocal(x.starts_at),
      end: x.ends_at ? rfc3339ToBerlinLocal(x.ends_at) : '',
    })
  } catch (e) {
    console.error('tournament fetch failed', e)
  }
})

async function run(action: () => Promise<void>, done: string) {
  if (busy.value) return
  busy.value = true
  status.value = error.value = null
  expired.value = false
  try {
    await action()
    status.value = done
  } catch (e) {
    if (e instanceof AuthExpiredError) expired.value = true
    else {
      console.error('tournament save failed', e)
      error.value = 'admin.tournament.failed'
    }
  } finally {
    busy.value = false
  }
}

const save = () =>
  run(async () => {
    await putTournament({
      name: f.name.trim(),
      place: f.place.trim() || null,
      starts_at: berlinToRfc3339(f.start),
      ends_at: f.end ? berlinToRfc3339(f.end) : null,
    })
    exists.value = true
  }, 'admin.tournament.saved')

const remove = () =>
  run(async () => {
    await deleteTournament()
    exists.value = false
    Object.assign(f, { name: '', place: '', start: '', end: '' })
  }, 'admin.tournament.removed')

const label = 'text-[14px] font-medium'
</script>

<template>
  <section aria-labelledby="tf-title" class="grid gap-4">
    <h2 id="tf-title" class="text-[24px] leading-[1.15] font-medium">
      {{ t('admin.tournament.title') }}
    </h2>
    <form class="grid gap-4 sm:grid-cols-2" @submit.prevent="save">
      <div class="grid gap-1">
        <label for="tf-name" :class="label">{{ t('admin.tournament.name') }}</label>
        <input id="tf-name" v-model="f.name" class="field" required />
      </div>
      <div class="grid gap-1">
        <label for="tf-place" :class="label">{{ t('admin.tournament.place') }}</label>
        <input id="tf-place" v-model="f.place" class="field" />
      </div>
      <div class="grid gap-1">
        <label for="tf-start" :class="label">{{ t('admin.tournament.start') }}</label>
        <input
          id="tf-start"
          v-model="f.start"
          type="datetime-local"
          class="field"
          aria-describedby="tf-hint"
          required
        />
      </div>
      <div class="grid gap-1">
        <label for="tf-end" :class="label">{{ t('admin.tournament.end') }}</label>
        <input
          id="tf-end"
          v-model="f.end"
          type="datetime-local"
          class="field"
          :min="f.start || undefined"
          aria-describedby="tf-hint"
        />
      </div>
      <p id="tf-hint" class="text-[13px] text-(--muted) sm:col-span-2">
        {{ t('admin.tournament.hint') }}
      </p>
      <div class="flex flex-wrap items-center gap-2 sm:col-span-2">
        <button type="submit" class="btn-primary" :disabled="busy">
          {{ t('admin.tournament.save') }}
        </button>
        <ConfirmButton
          v-if="exists"
          :label="t('admin.tournament.remove')"
          :question="t('admin.tournament.confirmRemove')"
          @confirm="remove"
        />
      </div>
      <p v-if="error" role="alert" class="text-[15px] sm:col-span-2">{{ t(error) }}</p>
      <AuthExpiredNotice v-if="expired" class="sm:col-span-2" @relogged="expired = false" />
      <p role="status" class="text-[15px] text-(--text-2) empty:hidden sm:col-span-2">
        {{ status ? t(status) : '' }}
      </p>
    </form>
  </section>
</template>
