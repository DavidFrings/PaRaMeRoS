<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <a
    href="#main"
    class="fixed top-2 left-2 z-50 -translate-y-20 rounded-[10px] bg-(--orange) px-4 py-3 text-[15px] font-medium text-(--on-orange) focus:translate-y-0"
    >{{ t('nav.skip') }}</a
  >
</template>
