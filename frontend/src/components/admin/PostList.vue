<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Section } from '@/api/client'
import PostTime from '@/components/PostTime.vue'
import type { Post } from '@/lib/post'
import PostAdminActions from './PostAdminActions.vue'

defineProps<{ groups: { section: Section; posts: Post[] }[] }>()
const emit = defineEmits<{ new: [section: Section]; deleted: [] }>()
const { t } = useI18n()
</script>

<template>
  <div class="grid gap-10">
    <section
      v-for="g in groups"
      :key="g.section"
      :data-section="g.section"
      :aria-labelledby="`pl-${g.section}`"
      class="grid gap-4 border-t border-(--line) pt-6"
    >
      <div class="flex flex-wrap items-center justify-between gap-3">
        <h2 :id="`pl-${g.section}`" class="text-[24px] leading-[1.15] font-medium">
          {{ t(`admin.sections.${g.section}`) }}
        </h2>
        <button
          type="button"
          class="btn-primary"
          :aria-describedby="`pl-${g.section}`"
          @click="emit('new', g.section)"
        >
          {{ t('admin.newPost') }}
        </button>
      </div>
      <p v-if="!g.posts.length" class="text-[15px] text-(--text-2)">{{ t('admin.empty') }}</p>
      <ul v-else class="grid gap-3">
        <li
          v-for="p in g.posts"
          :key="p.uuid"
          class="grid gap-3 rounded-[14px] border border-(--line) bg-(--card) p-4 md:grid-cols-[1fr_auto] md:items-center"
        >
          <div class="grid gap-1">
            <p class="text-[16px] font-medium">{{ p.heading }}</p>
            <PostTime :created="p.created_at" :updated="p.updated_at" />
          </div>
          <PostAdminActions
            :uuid="p.uuid"
            :title="p.heading"
            :section="p.name"
            @deleted="emit('deleted')"
          />
        </li>
      </ul>
    </section>
  </div>
</template>
