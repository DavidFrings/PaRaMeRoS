<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useLocale } from '@/composables/useLocale'

const { t } = useI18n()
const { localePath } = useLocale()
</script>

<template>
  <section class="mx-auto max-w-[1200px] px-5 py-24 md:px-8">
    <h1 class="text-[28px] font-semibold">{{ t('notFound.title') }}</h1>
    <p class="mt-3 text-(--text-2)">{{ t('notFound.text') }}</p>
    <RouterLink
      :to="localePath('/')"
      class="mt-6 inline-flex min-h-11 items-center text-(--orange)"
      >{{ t('notFound.home') }}</RouterLink
    >
  </section>
</template>
