<script setup lang="ts">
import { computed } from 'vue'
import { useLocale } from '@/composables/useLocale'
import { useNow } from '@/composables/useNow'
import { formatEdited, formatTimestamp, parseApiTime } from '@/lib/time'

const props = defineProps<{ created: string; updated?: string | null }>()
const { locale } = useLocale()
const now = useNow(60_000)

const ts = computed(() => formatTimestamp(parseApiTime(props.created), now.value, locale.value))
const edited = computed(() =>
  props.updated && props.updated !== props.created
    ? formatEdited(parseApiTime(props.updated), locale.value)
    : null,
)
</script>

<template>
  <span class="font-mono text-[12px] text-(--muted)">
    <time :datetime="ts.iso" :title="ts.title">{{ ts.text }}</time
    ><template v-if="edited"> · {{ edited }}</template>
  </span>
</template>
