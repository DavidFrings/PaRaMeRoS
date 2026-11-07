<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import LiveBadge from '@/components/LiveBadge.vue'
import PostEntry from '@/components/PostEntry.vue'
import { useLogbuch } from '@/composables/useLogbuch'

const { t } = useI18n()
const { posts } = useLogbuch()
</script>

<template>
  <section class="mx-auto max-w-[760px] px-5 pt-6 pb-8 md:px-8 lg:pt-12">
    <div class="flex items-center gap-4">
      <h1 class="text-[36px] leading-[1.06] font-medium tracking-[-0.02em] md:text-[56px]">
        {{ t('logbuch.title') }}
      </h1>
      <LiveBadge />
    </div>
    <p v-if="!posts.length" class="mt-6 text-[15px] text-(--text-2)">{{ t('logbuch.empty') }}</p>
    <ul v-else class="mt-8 grid gap-12 md:gap-16">
      <li v-for="p in posts" :key="p.uuid" class="border-t border-(--line) pt-6">
        <PostEntry :post="p" />
      </li>
    </ul>
  </section>
</template>
