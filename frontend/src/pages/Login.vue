<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'
import LoginForm from '@/components/admin/LoginForm.vue'
import { useLocale } from '@/composables/useLocale'
import { safeNext } from '@/lib/redirect'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const { localePath } = useLocale()

const done = () => router.replace(safeNext(route.query.next) ?? localePath('/'))
</script>

<template>
  <section class="mx-auto max-w-[400px] px-5 pt-10 pb-12 md:px-8 lg:pt-16">
    <h1 class="mb-8 text-[36px] leading-[1.06] font-medium tracking-[-0.02em] md:text-[48px]">
      {{ t('login.title') }}
    </h1>
    <LoginForm @success="done" />
  </section>
</template>
