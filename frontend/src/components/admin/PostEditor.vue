<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAuth } from '@/api/auth'
import {
  AuthExpiredError,
  ForbiddenError,
  createPost,
  deleteMedia,
  describeError,
  replaceMedia,
  updatePost,
  type Section,
} from '@/api/client'
import MarkdownContent from '@/components/MarkdownContent.vue'
import PostMedia from '@/components/PostMedia.vue'
import { refreshLogbuch } from '@/composables/useLogbuch'
import type { Post } from '@/lib/post'
import { seasonOf } from '@/lib/season'
import { berlinToRfc3339, parseApiTime, rfc3339ToBerlinLocal } from '@/lib/time'
import AuthExpiredNotice from './AuthExpiredNotice.vue'

// Props are read once: the parent keys the editor by post.
const props = defineProps<{ section: Section; post?: Post | null }>()
const emit = defineEmits<{ saved: []; cancel: [] }>()
const { t } = useI18n()
const { canSetCreatedAt } = useAuth()

// Same types and MiB limits as the API.
const ACCEPT = 'image/jpeg,image/png,image/webp,video/mp4,video/webm'
const MIB = 1024 * 1024

const { start } = seasonOf(new Date())
const initial = {
  heading: props.post?.heading ?? '',
  content: props.post?.content ?? '',
  heading_en: props.post?.heading_en ?? '',
  content_en: props.post?.content_en ?? '',
  // API season format is YYYY/YY.
  season: props.post ? (props.post.season ?? '') : `${start}/${String(start + 1).slice(2)}`,
}
type Field = keyof typeof initial
const saved = { ...initial }
const f = reactive({ ...initial })
// Berlin `datetime-local`; empty on create means "now".
let savedDate = props.post
  ? rfc3339ToBerlinLocal(parseApiTime(props.post.created_at).toISOString())
  : ''
const date = ref(savedDate)
// A post always has a date: clearing it on edit falls back to the saved one.
function restoreDate() {
  if (props.post && !date.value) date.value = savedDate
}
const file = ref<File | null>(null)
const creator = ref(props.post?.media_creator ?? '')
const removeMedia = ref(false)
const tab = ref<'de' | 'en'>('de')
const error = ref<string | null>(null)
const expired = ref(false)
const relogged = ref(false)
const busy = ref(false)
// The API changes the credit only together with new media.
const creatorLocked = computed(() => !!props.post && !file.value)

// Object URL of the chosen file for the preview; revoked on change and unmount.
const previewUrl = ref<string | null>(null)
watch(file, (x) => {
  if (previewUrl.value) URL.revokeObjectURL(previewUrl.value)
  previewUrl.value = x && !fileError(x) ? URL.createObjectURL(x) : null
})
onBeforeUnmount(() => {
  if (previewUrl.value) URL.revokeObjectURL(previewUrl.value)
})
const isVideo = computed(() => !!file.value?.type.startsWith('video/'))

const tabs = [
  { key: 'de', label: 'admin.editor.tabDe' },
  { key: 'en', label: 'admin.editor.tabEn' },
] as const

async function moveTab() {
  tab.value = tab.value === 'de' ? 'en' : 'de'
  await nextTick()
  document.getElementById(`pe-tab-${tab.value}`)?.focus()
}

const preview = computed(() =>
  tab.value === 'en'
    ? {
        heading: f.heading_en.trim() ? f.heading_en : f.heading,
        content: f.content_en.trim() ? f.content_en : f.content,
      }
    : { heading: f.heading, content: f.content },
)

function fileError(x: File) {
  if (!ACCEPT.split(',').includes(x.type)) return 'admin.editor.badType'
  if (x.type.startsWith('video/')) return x.size > 100 * MIB ? 'admin.editor.videoTooBig' : null
  return x.size > 15 * MIB ? 'admin.editor.imageTooBig' : null
}

function onFile(e: Event) {
  file.value = (e.target as HTMLInputElement).files?.[0] ?? null
  error.value = file.value && fileError(file.value)
  if (file.value) removeMedia.value = false
  // Without a new file the stored credit stays; drop any unsaved edit.
  else if (props.post) creator.value = props.post.media_creator ?? ''
}

function validate() {
  if (!f.heading.trim() || !f.content.trim()) {
    tab.value = 'de'
    return 'admin.editor.required'
  }
  if (props.section === 'team' && f.season && !/^\d{4}\/\d{2}$/.test(f.season))
    return 'admin.editor.seasonInvalid'
  if (file.value)
    return fileError(file.value) ?? (creator.value.trim() ? null : 'admin.editor.creatorRequired')
  return null
}

async function save(post: Post) {
  // Only changed fields: "" clears heading_en/content_en/season on the API.
  const changed = Object.fromEntries(
    (Object.keys(f) as Field[]).filter((k) => f[k] !== saved[k]).map((k) => [k, f[k]]),
  )
  const dateChanged = canSetCreatedAt.value && !!date.value && date.value !== savedDate
  if (Object.keys(changed).length || dateChanged) {
    await updatePost(
      post.uuid,
      dateChanged ? { ...changed, created_at: berlinToRfc3339(date.value) } : changed,
    )
    Object.assign(saved, changed)
    savedDate = date.value
  }
  if (file.value) await replaceMedia(post.uuid, file.value, creator.value.trim())
  else if (removeMedia.value && post.media_name) await deleteMedia(post.uuid)
}

async function submit() {
  if (busy.value) return
  expired.value = false
  relogged.value = false
  restoreDate()
  error.value = validate()
  if (error.value) return
  busy.value = true
  try {
    if (props.post) await save(props.post)
    else
      await createPost({
        name: props.section,
        heading: f.heading,
        content: f.content,
        heading_en: f.heading_en || undefined,
        content_en: f.content_en || undefined,
        season: props.section === 'team' ? f.season || undefined : undefined,
        created_at: canSetCreatedAt.value && date.value ? berlinToRfc3339(date.value) : undefined,
        file: file.value ?? undefined,
        media_creator: file.value ? creator.value.trim() : undefined,
      })
    void refreshLogbuch()
    emit('saved')
  } catch (e) {
    if (e instanceof AuthExpiredError) expired.value = true
    else if (e instanceof ForbiddenError) error.value = 'admin.forbidden'
    else {
      console.error('saving post failed', describeError(e))
      error.value = 'admin.editor.failed'
    }
  } finally {
    busy.value = false
  }
}

function onRelogged() {
  expired.value = false
  relogged.value = true
}

const label = 'text-[14px] font-medium'
const hint = 'text-[13px] text-(--muted)'
</script>

<template>
  <form class="grid gap-8 lg:grid-cols-2 lg:gap-10" novalidate @submit.prevent="submit">
    <div class="grid content-start gap-5">
      <h2 class="text-[24px] leading-[1.15] font-medium md:text-[32px]">
        {{
          t(post ? 'admin.editor.editTitle' : 'admin.editor.newTitle', {
            section: t(`admin.sections.${section}`),
          })
        }}
      </h2>

      <div
        role="tablist"
        :aria-label="t('admin.editor.languages')"
        class="flex gap-1 border-b border-(--line)"
      >
        <button
          v-for="x in tabs"
          :id="`pe-tab-${x.key}`"
          :key="x.key"
          type="button"
          role="tab"
          :aria-selected="tab === x.key"
          :aria-controls="`pe-panel-${x.key}`"
          :tabindex="tab === x.key ? 0 : -1"
          class="-mb-px min-h-11 border-b-2 px-3 text-[15px] transition-colors"
          :class="
            tab === x.key
              ? 'border-(--orange) font-medium text-(--text)'
              : 'border-transparent text-(--text-2) hover:text-(--text)'
          "
          @click="tab = x.key"
          @keydown.left.prevent="moveTab"
          @keydown.right.prevent="moveTab"
        >
          {{ t(x.label) }}
        </button>
      </div>

      <div
        v-for="l in ['de', 'en'] as const"
        v-show="tab === l"
        :id="`pe-panel-${l}`"
        :key="l"
        role="tabpanel"
        :aria-labelledby="`pe-tab-${l}`"
        class="grid gap-4"
      >
        <div class="grid gap-1">
          <label :for="`pe-heading-${l}`" :class="label">{{ t('admin.editor.heading') }}</label>
          <input
            v-if="l === 'de'"
            id="pe-heading-de"
            v-model="f.heading"
            class="field"
            aria-required="true"
          />
          <input v-else id="pe-heading-en" v-model="f.heading_en" lang="en" class="field" />
        </div>
        <div class="grid gap-1">
          <label :for="`pe-content-${l}`" :class="label">{{ t('admin.editor.content') }}</label>
          <textarea
            v-if="l === 'de'"
            id="pe-content-de"
            v-model="f.content"
            rows="16"
            class="field min-h-[320px] font-mono text-[15px] leading-relaxed"
            aria-required="true"
          />
          <textarea
            v-else
            id="pe-content-en"
            v-model="f.content_en"
            lang="en"
            rows="16"
            class="field min-h-[320px] font-mono text-[15px] leading-relaxed"
          />
        </div>
      </div>

      <div v-if="section === 'team'" class="grid gap-1">
        <label for="pe-season" :class="label">{{ t('admin.editor.season') }}</label>
        <input
          id="pe-season"
          v-model="f.season"
          class="field max-w-[160px] font-mono"
          placeholder="2025/26"
          inputmode="numeric"
          aria-describedby="pe-season-hint"
        />
        <p id="pe-season-hint" :class="hint">{{ t('admin.editor.seasonHint') }}</p>
      </div>

      <div v-if="canSetCreatedAt" class="grid gap-1">
        <label for="pe-date" :class="label">{{ t('admin.editor.date') }}</label>
        <input
          id="pe-date"
          v-model="date"
          type="datetime-local"
          class="field max-w-[260px]"
          aria-describedby="pe-date-hint"
          @blur="restoreDate"
        />
        <p id="pe-date-hint" :class="hint">
          {{ t(post ? 'admin.editor.dateHint' : 'admin.editor.dateHintNew') }}
        </p>
      </div>

      <div class="grid gap-3">
        <div class="grid gap-1">
          <label for="pe-file" :class="label">{{ t('admin.editor.media') }}</label>
          <input
            id="pe-file"
            type="file"
            accept="image/jpeg,image/png,image/webp,video/mp4,video/webm"
            class="text-[15px] file:mr-3 file:min-h-11 file:rounded-[10px] file:border file:border-(--field-border) file:bg-(--card) file:px-4 file:text-(--text)"
            aria-describedby="pe-file-hint"
            @change="onFile"
          />
          <p id="pe-file-hint" :class="hint">{{ t('admin.editor.mediaHint') }}</p>
        </div>
        <div class="grid gap-1">
          <label for="pe-creator" :class="label">{{ t('admin.editor.creator') }}</label>
          <input
            id="pe-creator"
            v-model="creator"
            class="field"
            autocomplete="name"
            :readonly="creatorLocked"
            :class="creatorLocked && 'text-(--text-2)'"
            :aria-required="file ? 'true' : undefined"
            :aria-describedby="creatorLocked ? 'pe-creator-hint' : undefined"
          />
          <p v-if="creatorLocked" id="pe-creator-hint" :class="hint">
            {{ t('admin.editor.creatorLocked') }}
          </p>
        </div>
        <label
          v-if="post?.media_name && !file"
          class="flex min-h-11 items-center gap-3 text-[15px]"
        >
          <input v-model="removeMedia" type="checkbox" class="h-5 w-5 accent-(--orange)" />
          {{ t('admin.editor.removeMedia') }}
        </label>
      </div>

      <p v-if="error" role="alert" class="rounded-[10px] bg-(--raised) px-3 py-2 text-[15px]">
        {{ t(error) }}
      </p>
      <AuthExpiredNotice v-if="expired" @relogged="onRelogged" />
      <p role="status" class="text-[15px] empty:hidden">
        {{ relogged ? t('admin.reloggedIn') : '' }}
      </p>
    </div>

    <section aria-labelledby="pe-preview" class="grid content-start gap-3">
      <h2 id="pe-preview" class="font-mono text-[12px] text-(--muted)">
        {{ t('admin.editor.preview') }}
      </h2>
      <article class="grid gap-3 rounded-[16px] border border-(--line) bg-(--card) p-5" :lang="tab">
        <p class="text-[24px] leading-[1.15] font-medium md:text-[32px]">{{ preview.heading }}</p>
        <figure v-if="file && previewUrl">
          <video
            v-if="isVideo"
            controls
            preload="metadata"
            :src="previewUrl"
            class="w-full rounded-[12px]"
          />
          <img v-else :src="previewUrl" alt="" class="w-full rounded-[12px]" />
          <figcaption v-if="creator.trim()" class="mt-2 font-mono text-[12px] text-(--muted)">
            {{ t(isVideo ? 'logbuch.videoBy' : 'logbuch.photoBy', { creator: creator.trim() }) }}
          </figcaption>
        </figure>
        <PostMedia
          v-else-if="post?.media_name && !removeMedia"
          :post="post"
          size="small"
          :alt="preview.heading"
        />
        <MarkdownContent :source="preview.content" />
      </article>
    </section>

    <div
      class="sticky bottom-0 -mx-5 flex justify-end gap-2 border-t border-(--line) bg-(--bg) px-5 py-3 md:-mx-8 md:px-8 lg:static lg:col-span-2 lg:mx-0 lg:border-0 lg:px-0"
    >
      <button type="button" class="btn" @click="emit('cancel')">{{ t('admin.cancel') }}</button>
      <button type="submit" class="btn-primary" :disabled="busy" :aria-busy="busy">
        {{ t(post ? 'admin.editor.save' : 'admin.editor.publish') }}
      </button>
    </div>
  </form>
</template>
