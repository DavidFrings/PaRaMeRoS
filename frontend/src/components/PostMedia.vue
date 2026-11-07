<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { mediaSources } from '@/lib/media'
import type { Post } from '@/lib/post'

const props = defineProps<{ post: Post; size: 'full' | 'small'; alt: string }>()
const { t } = useI18n()

const src = computed(() => {
  const { media_type, media_name } = props.post
  if (!media_type || !media_name) return null
  if (media_type === 'vid') return { vid: `/uploads/${media_name}` }
  const s = mediaSources(media_name, props.size)
  return { jxl: `/${s.jxl}`, webp: `/${s.webp}` }
})
</script>

<template>
  <figure v-if="src">
    <video
      v-if="'vid' in src"
      controls
      preload="metadata"
      :src="src.vid"
      class="w-full rounded-[12px]"
    />
    <picture v-else>
      <source type="image/jxl" :srcset="src.jxl" />
      <source type="image/webp" :srcset="src.webp" />
      <img
        :src="src.webp"
        :alt="alt"
        loading="lazy"
        decoding="async"
        class="w-full rounded-[12px]"
      />
    </picture>
    <figcaption v-if="post.media_creator" class="mt-2 font-mono text-[12px] text-(--muted)">
      {{
        t(post.media_type === 'vid' ? 'logbuch.videoBy' : 'logbuch.photoBy', {
          creator: post.media_creator,
        })
      }}
    </figcaption>
  </figure>
</template>
