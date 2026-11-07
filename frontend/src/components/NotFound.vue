<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useLocale } from '@/composables/useLocale'

const { t } = useI18n()
const { localePath } = useLocale()
</script>

<template>
  <section class="mx-auto max-w-[1200px] px-5 py-16 md:px-8 lg:py-28">
    <p class="font-mono text-[12px] text-(--muted)">404</p>
    <h1 class="mt-2 text-[36px] leading-[1.06] font-medium tracking-[-0.02em] md:text-[56px]">
      {{ t('notFound.title') }}
    </h1>
    <p class="mt-4 text-[16px] text-(--text-2) md:text-[19px]">{{ t('notFound.text') }}</p>
    <RouterLink
      :to="localePath('/')"
      class="mt-6 inline-flex h-12 items-center rounded-[10px] bg-(--orange) px-5 text-[15px] font-medium whitespace-nowrap text-(--on-orange) transition-[filter] hover:brightness-95"
      >{{ t('notFound.home') }}</RouterLink
    >
  </section>
</template>
