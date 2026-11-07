<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <h1 class="px-5 text-[28px] font-semibold md:px-8">{{ t('logbuch.title') }}</h1>
</template>
