<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useLocale } from '@/composables/useLocale'
import { useLogbuch } from '@/composables/useLogbuch'
import { localizedPost } from '@/lib/post'
import { seasonFromPost } from '@/lib/season'
import PostTime from './PostTime.vue'
import SeasonTag from './SeasonTag.vue'

const props = defineProps<{ limit?: number }>()
const { t } = useI18n()
const { locale, localePath } = useLocale()
const { posts } = useLogbuch()
const items = computed(() => posts.value.slice(0, props.limit))
</script>

<template>
  <p v-if="!items.length" class="text-[15px] text-(--text-2)">{{ t('logbuch.empty') }}</p>
  <ul v-else class="grid gap-4">
    <li v-for="p in items" :key="p.uuid" class="grid gap-1">
      <PostTime :created="p.created_at" :updated="p.updated_at" />
      <h3 class="text-[17px] font-medium">
        <RouterLink
          :to="localePath('/post/' + p.uuid)"
          class="transition-colors hover:text-(--green)"
          >{{ localizedPost(p, locale).heading }}</RouterLink
        >
      </h3>
      <div><SeasonTag :label="seasonFromPost(p)" /></div>
    </li>
  </ul>
</template>
