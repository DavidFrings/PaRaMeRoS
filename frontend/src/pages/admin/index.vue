<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { computed, nextTick, onMounted, ref, useTemplateRef, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'
import { describeError, getPost, getPosts, type Section } from '@/api/client'
import { logout, useAuth } from '@/api/auth'
import PostEditor from '@/components/admin/PostEditor.vue'
import PostList from '@/components/admin/PostList.vue'
import TournamentForm from '@/components/admin/TournamentForm.vue'
import { useLocale } from '@/composables/useLocale'
import type { Post } from '@/lib/post'
import { parseApiTime } from '@/lib/time'

const SECTIONS: Section[] = ['logbuch', 'team', 'coaches', 'forest-project']

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const { localePath } = useLocale()
const { loggedIn, check, can } = useAuth()
const title = useTemplateRef<HTMLElement>('title')

const ready = ref(false)
const lists = ref<Partial<Record<Section, Post[]>>>({})
const loadFailed = ref(false)
const notFound = ref(false)
const forbidden = ref(false)
const editing = ref<{ section: Section; post: Post | null } | null>(null)

// Only the areas this account may write; the API enforces the same rule.
const sections = computed(() => SECTIONS.filter((s) => can(s)))
const groups = computed(() =>
  sections.value.map((section) => ({ section, posts: lists.value[section] ?? [] })),
)

// Old logbuch posts carry the legacy name "progress-logs".
const sectionOf = (name: string): Section => SECTIONS.find((s) => s === name) ?? 'logbuch'

const newest = (a: Post, b: Post) =>
  parseApiTime(b.created_at).getTime() - parseApiTime(a.created_at).getTime()

async function load() {
  loadFailed.value = false
  const shown = sections.value
  const all = await Promise.all(
    shown.map((s) =>
      getPosts(s).catch((e) => {
        console.error('admin posts fetch failed', describeError(e))
        loadFailed.value = true
        return [] as Post[]
      }),
    ),
  )
  lists.value = Object.fromEntries(shown.map((s, i) => [s, [...all[i]].sort(newest)]))
}

async function openFromQuery(id: unknown) {
  notFound.value = forbidden.value = false
  if (typeof id !== 'string' || !id) {
    if (editing.value?.post) editing.value = null
    return
  }
  let post = Object.values(lists.value)
    .flat()
    .find((p) => p.uuid === id)
  if (!post) {
    try {
      post = await getPost(id)
    } catch {
      notFound.value = true
      return
    }
  }
  // A deep link may point at a section this account cannot write; the API would 403 the save.
  if (!can(post.name)) {
    forbidden.value = true
    if (editing.value?.post) editing.value = null
    return
  }
  editing.value = { section: sectionOf(post.name), post }
}

function newPost(section: Section) {
  // The coaches section holds a single post.
  const coaches = section === 'coaches' && lists.value.coaches?.[0]
  if (coaches) void router.push({ query: { edit: coaches.uuid } })
  else editing.value = { section, post: null }
}

async function close() {
  editing.value = null
  if (route.query.edit) await router.replace({ query: {} })
  await nextTick()
  title.value?.focus()
}

async function saved() {
  await close()
  await load()
}

function signOut() {
  logout()
  void router.push(localePath('/'))
}

onMounted(async () => {
  await check()
  if (!loggedIn.value) {
    await router.replace(`${localePath('/Login')}?next=${encodeURIComponent(route.fullPath)}`)
    return
  }
  await load()
  ready.value = true
  await openFromQuery(route.query.edit)
})
watch(
  () => route.query.edit,
  (id) => ready.value && openFromQuery(id),
)
</script>

<template>
  <div v-if="ready" class="mx-auto max-w-[1200px] px-5 pt-6 pb-8 md:px-8 lg:pt-12">
    <PostEditor
      v-if="editing"
      :key="editing.post?.uuid ?? `new-${editing.section}`"
      :section="editing.section"
      :post="editing.post"
      @saved="saved"
      @cancel="close"
    />
    <template v-else>
      <div class="flex flex-wrap items-center justify-between gap-4">
        <h1
          ref="title"
          tabindex="-1"
          class="text-[36px] leading-[1.06] font-medium tracking-[-0.02em] outline-none md:text-[56px]"
        >
          {{ t('admin.title') }}
        </h1>
        <button type="button" class="btn" @click="signOut">{{ t('admin.logout') }}</button>
      </div>
      <p v-if="notFound" role="alert" class="mt-4 text-[15px]">{{ t('admin.editor.notFound') }}</p>
      <p v-if="forbidden" role="alert" class="mt-4 text-[15px]">{{ t('admin.forbidden') }}</p>
      <p v-if="loadFailed" role="alert" class="mt-4 text-[15px]">{{ t('admin.loadFailed') }}</p>
      <p v-if="!groups.length && !can('tournament')" class="mt-4 text-[15px]">
        {{ t('admin.noPermission') }}
      </p>
      <div v-else class="mt-10 grid gap-12">
        <TournamentForm v-if="can('tournament')" />
        <PostList v-if="groups.length" :groups="groups" @new="newPost" @deleted="load" />
      </div>
    </template>
  </div>
</template>
