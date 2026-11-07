<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { ArrowRight } from 'lucide-vue-next'
import { getPosts } from '@/api/client'
import LiveBadge from '@/components/LiveBadge.vue'
import LogbuchList from '@/components/LogbuchList.vue'
import SeasonTag from '@/components/SeasonTag.vue'
import TournamentCountdown from '@/components/TournamentCountdown.vue'
import { useLocale } from '@/composables/useLocale'
import { useLogbuch } from '@/composables/useLogbuch'
import { mediaSources } from '@/lib/media'
import type { Post } from '@/lib/post'
import { seasonAnchor, seasonFromPost, seasonOf } from '@/lib/season'
import ErfolgeImg from '@/assets/imgs/erfolge.webp'

const { t } = useI18n()
const { localePath } = useLocale()
const { posts: logbuch } = useLogbuch()

const currentSeason = seasonOf(new Date()).label
const teamPosts = ref<Post[]>([])

onMounted(async () => {
  // A failed fetch renders like "no team posts".
  teamPosts.value = await getPosts('team').catch(() => [])
})

// Labels like "24/25" sort correctly as strings within one century.
const teams = computed(() =>
  teamPosts.value
    .map((post) => ({ post, season: seasonFromPost(post) }))
    .sort((a, b) => b.season.localeCompare(a.season)),
)
// PostMedia always captions with the photo credit; home shows the season instead.
const img = (p: Post, size: 'full' | 'small') =>
  p.media_type === 'img' && p.media_name ? mediaSources(p.media_name, size) : null

const hero = computed(() => {
  const x = teams.value.find((x) => img(x.post, 'full'))
  return x && { season: x.season, src: img(x.post, 'full')! }
})

const logbuchImg = computed(() => {
  const s = logbuch.value[0] && img(logbuch.value[0], 'small')
  return s ? `/${s.webp}` : ErfolgeImg
})

const link = 'inline-flex min-h-11 items-center gap-1 text-[15px] font-medium text-(--green)'
</script>

<template>
  <section
    class="mx-auto grid max-w-[1200px] items-center gap-6 px-4 pt-2 md:px-8 lg:gap-16 lg:pt-12"
    :class="{ 'lg:grid-cols-[5fr_7fr]': hero }"
  >
    <div class="order-2 lg:order-1">
      <SeasonTag :label="currentSeason" prefix />
      <TournamentCountdown class="mt-3" />
      <h1
        class="mt-4 text-[36px] leading-[1.06] font-medium tracking-[-0.02em] md:text-[56px] md:tracking-[-0.03em]"
      >
        {{ t('home.heroTitle') }}
      </h1>
      <p
        class="mt-4 max-w-[36rem] text-[16px] leading-[24px] text-(--text-2) md:text-[19px] md:leading-[29px]"
      >
        {{ t('home.heroText') }}
      </p>
      <div class="mt-6 flex flex-wrap items-center gap-x-6 gap-y-3 md:mt-8">
        <RouterLink
          :to="localePath('/team')"
          class="inline-flex h-12 items-center rounded-[10px] bg-(--orange) px-5 text-[15px] font-medium whitespace-nowrap text-(--on-orange) transition-[filter] hover:brightness-95"
          >{{ t('home.meetTeam') }}</RouterLink
        >
        <RouterLink
          :to="localePath('/logbuch')"
          class="inline-flex min-h-11 items-center text-[15px] font-medium whitespace-nowrap text-(--green) underline decoration-current/30 underline-offset-4"
          >{{ t('home.toLogbuch') }}</RouterLink
        >
      </div>
    </div>
    <figure v-if="hero" class="order-1 lg:order-2">
      <picture>
        <source type="image/jxl" :srcset="'/' + hero.src.jxl" />
        <img
          :src="'/' + hero.src.webp"
          :alt="t('home.teamAlt', { season: hero.season })"
          fetchpriority="high"
          class="aspect-[4/3] w-full rounded-[16px] object-cover lg:rounded-[20px]"
        />
      </picture>
      <figcaption class="mt-2 font-mono text-[12px] text-(--muted)">
        {{ t('home.teamCaption', { season: hero.season }) }}
      </figcaption>
    </figure>
  </section>

  <section class="mx-auto max-w-[1200px] px-4 pt-10 md:px-8 lg:pt-20">
    <div class="flex items-center justify-between gap-4 px-1">
      <h2 class="flex items-center gap-3 text-[13px] text-(--muted)">
        {{ t('home.newInLogbuch') }} <LiveBadge />
      </h2>
      <RouterLink :to="localePath('/logbuch')" :class="link"
        >{{ t('home.allEntries') }} <ArrowRight :size="16" :stroke-width="1.5" aria-hidden="true"
      /></RouterLink>
    </div>
    <div class="mt-2 rounded-[14px] border border-(--line) bg-(--card) p-5 md:p-6">
      <LogbuchList :limit="3" class="lg:grid-cols-3 lg:gap-8" />
    </div>
  </section>

  <section
    class="mx-auto grid max-w-[1200px] gap-3 px-5 pt-14 md:px-8 lg:grid-cols-[5fr_7fr] lg:gap-16 lg:pt-24"
  >
    <h2 class="text-[24px] leading-[1.15] font-medium md:text-[32px]">
      {{ t('home.whoTitle') }}
    </h2>
    <div>
      <p class="text-[16px] leading-[25px] text-(--text-2) md:text-[18px] md:leading-[28px]">
        {{ t('home.whoText') }}
      </p>
      <RouterLink :to="localePath('/About')" :class="[link, 'mt-2']"
        >{{ t('home.moreAbout') }} <ArrowRight :size="16" :stroke-width="1.5" aria-hidden="true"
      /></RouterLink>
    </div>
  </section>

  <section v-if="teams.length" class="pt-14 lg:pt-24">
    <div
      class="mx-auto flex max-w-[1200px] flex-col gap-1 px-5 md:flex-row md:items-baseline md:justify-between md:px-8"
    >
      <h2 class="text-[24px] leading-[1.15] font-medium md:text-[32px]">
        {{ t('home.seasonsTitle') }}
      </h2>
      <span class="font-mono text-[12px] text-(--muted)">{{
        t(
          'home.seasonsCount',
          { n: teams.length, oldest: teams[teams.length - 1]!.season },
          teams.length,
        )
      }}</span>
    </div>
    <ul
      class="no-bar mx-auto mt-5 flex max-w-[1200px] snap-x snap-mandatory scroll-px-5 gap-3 overflow-x-auto px-5 pb-1 md:scroll-px-8 md:gap-4 md:px-8"
    >
      <li v-for="x in teams" :key="x.post.uuid" class="w-[240px] shrink-0 snap-start md:w-[300px]">
        <RouterLink
          :to="localePath('/team') + '#' + seasonAnchor(x.season)"
          class="block rounded-[12px]"
        >
          <picture v-if="img(x.post, 'small')">
            <source type="image/jxl" :srcset="'/' + img(x.post, 'small')!.jxl" />
            <img
              :src="'/' + img(x.post, 'small')!.webp"
              :alt="t('home.teamAlt', { season: x.season })"
              loading="lazy"
              decoding="async"
              class="aspect-[4/3] w-full rounded-[12px] object-cover"
            />
          </picture>
          <div v-else class="aspect-[4/3] rounded-[12px] bg-(--raised)" />
          <span class="mt-2 block font-mono text-[12px] text-(--text-2)">{{ x.season }}</span>
        </RouterLink>
      </li>
    </ul>
  </section>

  <section class="mx-auto max-w-[1200px] px-4 pt-14 md:px-8 lg:pt-24">
    <article
      class="overflow-hidden rounded-[16px] border border-(--line) bg-(--card) md:grid md:grid-cols-2"
    >
      <img
        :src="logbuchImg"
        alt=""
        loading="lazy"
        decoding="async"
        class="aspect-[16/10] w-full object-cover md:aspect-auto md:h-full"
      />
      <div class="p-5 md:flex md:flex-col md:justify-center md:p-8">
        <h2 class="text-[20px] leading-[1.2] font-medium md:text-[24px]">
          {{ t('home.logbuchTitle') }}
        </h2>
        <p class="mt-2 text-[15px] leading-[22px] text-(--text-2)">{{ t('home.logbuchText') }}</p>
        <RouterLink :to="localePath('/logbuch')" :class="[link, 'mt-2']"
          >{{ t('home.toLogbuch') }} <ArrowRight :size="16" :stroke-width="1.5" aria-hidden="true"
        /></RouterLink>
      </div>
    </article>
  </section>
</template>
