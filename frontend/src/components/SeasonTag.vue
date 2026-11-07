<script setup lang="ts">
import { useI18n } from 'vue-i18n'

withDefaults(defineProps<{ label: string; prefix?: boolean }>(), { prefix: false })
const { t } = useI18n()
</script>

<template>
  <span
    class="inline-block rounded-[6px] bg-(--green-tag) px-2 py-1 font-mono text-[12px] leading-none text-white"
    >{{ prefix ? `${t('logbuch.season')} ${label}` : label }}</span
  >
</template>
