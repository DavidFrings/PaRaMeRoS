<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { ArrowRight } from 'lucide-vue-next'
import { getPosts } from '@/api/client'
import PostAdminActions from '@/components/admin/PostAdminActions.vue'
import MarkdownContent from '@/components/MarkdownContent.vue'
import PostMedia from '@/components/PostMedia.vue'
import { useLocale } from '@/composables/useLocale'
import { localizedPost, type Post } from '@/lib/post'

const { t } = useI18n()
const { locale, localePath } = useLocale()
const coaches = ref<Post | null>(null)

onMounted(async () => {
  // A failed fetch hides the coaches block.
  coaches.value = (await getPosts('coaches').catch(() => []))[0] ?? null
})

const coachesText = computed(() => coaches.value && localizedPost(coaches.value, locale.value))
const link = 'inline-flex min-h-11 items-center gap-1 text-[15px] font-medium text-(--green)'
</script>

<template>
  <section class="mx-auto max-w-[760px] px-5 pt-6 pb-8 md:px-8 lg:pt-12">
    <h1 class="text-[36px] leading-[1.06] font-medium tracking-[-0.02em] md:text-[56px]">
      {{ t('about.title') }}
    </h1>
    <div
      class="mt-6 grid gap-4 text-[16px] leading-[25px] text-(--text-2) md:text-[18px] md:leading-[28px]"
    >
      <p>{{ t('about.p1') }}</p>
      <p>{{ t('about.p2') }}</p>
      <p>{{ t('about.p3') }}</p>
      <p class="font-medium text-(--text)">{{ t('about.p4') }}</p>
    </div>
    <a
      href="https://www.prmrs.de"
      rel="noopener noreferrer"
      target="_blank"
      :class="[link, 'mt-2']"
      >{{ t('about.school') }}</a
    >

    <div v-if="coaches && coachesText" class="mt-12 grid gap-3 border-t border-(--line) pt-8">
      <h2 class="text-[24px] leading-[1.15] font-medium md:text-[32px]">
        {{ coachesText.heading }}
      </h2>
      <MarkdownContent :source="coachesText.content" class="text-(--text-2)" />
      <PostMedia :post="coaches" size="full" :alt="t('about.coachesAlt')" />
      <PostAdminActions
        :uuid="coaches.uuid"
        :title="coachesText.heading"
        :section="coaches.name"
        @deleted="coaches = null"
      />
    </div>

    <RouterLink :to="localePath('/team')" :class="[link, 'mt-8']"
      >{{ t('about.allTeams') }} <ArrowRight :size="16" :stroke-width="1.5" aria-hidden="true"
    /></RouterLink>
  </section>
</template>
