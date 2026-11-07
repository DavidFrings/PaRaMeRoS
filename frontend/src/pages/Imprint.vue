<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import ImprintDe from '@/components/legal/ImprintDe.vue'
import ImprintEn from '@/components/legal/ImprintEn.vue'
import LegalPage from '@/components/LegalPage.vue'
import { useLocale } from '@/composables/useLocale'

const { t } = useI18n()
const { locale } = useLocale()
</script>

<template>
  <LegalPage :title="t('nav.imprint')">
    <ImprintEn v-if="locale === 'en'" />
    <ImprintDe v-else />
  </LegalPage>
</template>
