<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { getPosts } from '@/api/client'
import PostEntry from '@/components/PostEntry.vue'
import type { Post } from '@/lib/post'
import { parseApiTime } from '@/lib/time'

const { t } = useI18n()
const loaded = ref<Post[]>([])

onMounted(async () => {
  // A failed fetch renders like "no entries".
  loaded.value = await getPosts('forest-project').catch(() => [])
})

const posts = computed(() =>
  [...loaded.value].sort(
    (a, b) => parseApiTime(b.created_at).getTime() - parseApiTime(a.created_at).getTime(),
  ),
)
</script>

<template>
  <section class="mx-auto max-w-[760px] px-5 pt-6 pb-8 md:px-8 lg:pt-12">
    <h1 class="text-[36px] leading-[1.06] font-medium tracking-[-0.02em] md:text-[56px]">
      {{ t('forest.title') }}
    </h1>
    <p class="mt-4 text-[16px] leading-[25px] text-(--text-2) md:text-[18px] md:leading-[28px]">
      {{ t('forest.note') }}
    </p>
    <p v-if="!posts.length" class="mt-8 text-[15px] text-(--text-2)">{{ t('logbuch.empty') }}</p>
    <ul v-else class="mt-8 grid gap-12 md:gap-16">
      <li v-for="p in posts" :key="p.uuid" class="border-t border-(--line) pt-6">
        <PostEntry :post="p" />
      </li>
    </ul>
  </section>
</template>
