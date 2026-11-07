<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import { getPosts } from '@/api/client'
import PostAdminActions from '@/components/admin/PostAdminActions.vue'
import MarkdownContent from '@/components/MarkdownContent.vue'
import PostMedia from '@/components/PostMedia.vue'
import SeasonTag from '@/components/SeasonTag.vue'
import { useLocale } from '@/composables/useLocale'
import { localizedPost, type Post } from '@/lib/post'
import { seasonAnchor, seasonFromPost } from '@/lib/season'

const { t } = useI18n()
const { locale } = useLocale()
const route = useRoute()
const loaded = ref<Post[]>([])

// Labels like "24/25" sort correctly as strings within one century.
const teams = computed(() =>
  loaded.value
    .map((post) => ({ post, season: seasonFromPost(post), ...localizedPost(post, locale.value) }))
    .sort((a, b) => b.season.localeCompare(a.season)),
)

// The sections only exist after the posts load, so the hash is applied by hand.
async function scrollToHash() {
  await nextTick()
  if (route.hash) document.getElementById(decodeURIComponent(route.hash.slice(1)))?.scrollIntoView()
}

onMounted(async () => {
  // A failed fetch renders like "no teams".
  loaded.value = await getPosts('team').catch(() => [])
  await scrollToHash()
})
watch(() => route.hash, scrollToHash)
</script>

<template>
  <div class="mx-auto max-w-[1200px] px-5 pt-6 pb-8 md:px-8 lg:pt-12">
    <h1 class="text-[36px] leading-[1.06] font-medium tracking-[-0.02em] md:text-[56px]">
      {{ t('team.title') }}
    </h1>
    <section
      v-for="x in teams"
      :id="seasonAnchor(x.season)"
      :key="x.post.uuid"
      class="mt-10 scroll-mt-20 grid gap-4 border-t border-(--line) pt-6 lg:mt-16 lg:grid-cols-[5fr_7fr] lg:gap-16"
    >
      <div>
        <SeasonTag :label="x.season" prefix />
        <h2 class="mt-3 text-[24px] leading-[1.15] font-medium md:text-[32px]">
          {{ x.heading }}
        </h2>
        <MarkdownContent :source="x.content" class="mt-2 text-(--text-2)" />
        <PostAdminActions
          :uuid="x.post.uuid"
          :title="x.heading"
          class="mt-4"
          @deleted="loaded = loaded.filter((p) => p.uuid !== x.post.uuid)"
        />
      </div>
      <PostMedia :post="x.post" size="full" :alt="t('team.alt', { season: x.season })" />
    </section>
  </div>
</template>
