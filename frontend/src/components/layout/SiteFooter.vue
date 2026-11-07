<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useLocale } from '@/composables/useLocale'

const { t } = useI18n()
const { localePath } = useLocale()
const link = 'inline-flex min-h-11 items-center hover:text-(--text)'
const year = new Date().getFullYear()
</script>

<template>
  <footer
    class="mx-auto mt-16 flex w-full max-w-[1200px] flex-col gap-3 border-t border-(--line) px-5 py-8 text-[13px] text-(--muted) md:flex-row md:items-center md:justify-between md:px-8 lg:mt-24"
  >
    <p>© 2023–{{ year }} PaRaMeRoS · {{ t('nav.school') }}</p>
    <div class="flex flex-wrap gap-x-5">
      <RouterLink :to="localePath('/Imprint')" :class="link">{{ t('nav.imprint') }}</RouterLink>
      <RouterLink :to="localePath('/Privacy-Policy')" :class="link">{{
        t('nav.privacy')
      }}</RouterLink>
      <a href="/sitemap.xml" :class="link">{{ t('nav.sitemap') }}</a>
      <button type="button" data-cc="show-preferencesModal" :class="link">
        {{ t('nav.cookies') }}
      </button>
      <RouterLink :to="localePath('/prmrs/forest-project')" :class="link">{{
        t('nav.forest')
      }}</RouterLink>
    </div>
  </footer>
</template>
