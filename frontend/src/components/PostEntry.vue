<script setup lang="ts">
import { computed } from 'vue'
import { useLocale } from '@/composables/useLocale'
import { localizedPost, type Post } from '@/lib/post'
import { seasonFromPost } from '@/lib/season'
import MarkdownContent from './MarkdownContent.vue'
import PostMedia from './PostMedia.vue'
import PostTime from './PostTime.vue'
import SeasonTag from './SeasonTag.vue'

const props = withDefaults(defineProps<{ post: Post; as?: 'h1' | 'h2'; link?: boolean }>(), {
  as: 'h2',
  link: true,
})
const { locale, localePath } = useLocale()
const text = computed(() => localizedPost(props.post, locale.value))
</script>

<template>
  <article class="grid gap-3">
    <PostTime :created="post.created_at" :updated="post.updated_at" />
    <component
      :is="as"
      class="text-[24px] leading-[1.15] font-medium tracking-[-0.01em] md:text-[32px]"
    >
      <RouterLink
        v-if="link"
        :to="localePath('/post/' + post.uuid)"
        class="transition-colors hover:text-(--green)"
        >{{ text.heading }}</RouterLink
      >
      <template v-else>{{ text.heading }}</template>
    </component>
    <MarkdownContent :source="text.content" />
    <PostMedia :post="post" size="full" :alt="text.heading" />
    <div><SeasonTag :label="seasonFromPost(post)" /></div>
  </article>
</template>
