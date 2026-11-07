<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import LegalPage from '@/components/LegalPage.vue'
import PrivacyDe from '@/components/legal/PrivacyDe.vue'
import PrivacyEn from '@/components/legal/PrivacyEn.vue'
import { useLocale } from '@/composables/useLocale'

const { t } = useI18n()
const { locale } = useLocale()
</script>

<template>
  <LegalPage :title="t('legal.privacy')">
    <PrivacyEn v-if="locale === 'en'" />
    <PrivacyDe v-else />
  </LegalPage>
</template>
