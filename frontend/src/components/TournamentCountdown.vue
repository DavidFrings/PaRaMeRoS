<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useLocale } from '@/composables/useLocale'
import { useTournament } from '@/composables/useTournament'

const { t } = useI18n()
const { localePath } = useLocale()
const { tournament, state } = useTournament()
</script>

<template>
  <p v-if="tournament && state.kind !== 'hidden'" class="text-[15px] text-(--text)">
    <RouterLink v-if="state.kind === 'running'" :to="localePath('/logbuch')" class="underline">{{
      t('countdown.running')
    }}</RouterLink>
    <template v-else-if="state.kind === 'days'">{{
      t('countdown.days', { name: tournament.name, n: state.days }, state.days)
    }}</template>
    <template v-else>{{
      t('countdown.clock', { name: tournament.name, hms: state.hms })
    }}</template>
    <span v-if="tournament.place" class="mt-0.5 block font-mono text-[12px] text-(--muted)">{{
      tournament.place
    }}</span>
  </p>
</template>
