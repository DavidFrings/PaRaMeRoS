<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { getPost } from '@/api/client'
import NotFound from '@/components/NotFound.vue'
import PostEntry from '@/components/PostEntry.vue'
import type { Post } from '@/lib/post'

const route = useRoute()
const post = ref<Post | null>(null)
const failed = ref(false)

watch(
  () => route.params.id,
  async (id) => {
    post.value = null
    failed.value = false
    try {
      post.value = await getPost(String(id))
    } catch {
      // Unknown id and network error both show the 404 content.
      failed.value = true
    }
  },
  { immediate: true },
)
</script>

<template>
  <NotFound v-if="failed" />
  <section v-else-if="post" class="mx-auto max-w-[760px] px-5 pt-6 pb-8 md:px-8 lg:pt-12">
    <PostEntry :post="post" as="h1" :link="false" />
  </section>
</template>
