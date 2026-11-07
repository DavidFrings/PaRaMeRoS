<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useLogbuch } from '@/composables/useLogbuch'

const { t } = useI18n()
const { live } = useLogbuch()
</script>

<template>
  <span v-if="live" class="inline-flex items-center gap-1.5 font-mono text-[12px] text-(--green)">
    <span aria-hidden="true" class="h-2 w-2 rounded-full bg-(--green)" />{{ t('logbuch.live') }}
  </span>
</template>
