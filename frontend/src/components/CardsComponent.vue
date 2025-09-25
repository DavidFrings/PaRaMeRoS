<script setup lang="ts">
import { onMounted, ref } from 'vue'
import axios from 'axios'
import Card from '@/components/CardComponent.vue'

const props = defineProps<{
  name: string
}>()

interface Post {
  uuid: string
  heading: string
  content: string
  media_type?: 'img' | 'vid'
  media_name?: string
  media_creator?: string
}

const posts = ref<Post[]>([])
const api = window.__ENV__.API

onMounted(async () => {
  try {
    const res = await axios.get(`${api}/posts/${props.name}`)
    posts.value = res.data
  } catch (err: unknown) {
    console.error(err)
  }
})
</script>

<template>
  <div id="cards">
    <Card
      v-for="p in posts"
      :key="p.uuid"
      :id="p.uuid"
      :heading="p.heading"
      :content="p.content"
      :media_type="p.media_type"
      :media="p.media_name"
      :media_creator="p.media_creator"
    />
  </div>
</template>

<style scoped></style>
