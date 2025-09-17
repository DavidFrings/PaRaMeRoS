<script setup lang="ts">
import { onMounted, ref } from 'vue'
import axios from 'axios'
import Card from '@/components/CardComponent.vue'

defineProps<{
  name: string
}>()

interface Post {
  id: string
  heading: string
  content: string
  media?: 'img' | 'video' | false
  img?: string
  video?: string
}

const posts = ref<Post[]>([])
const api = import.meta.env.VITE_API

onMounted(async () => {
  try {
    const res = await axios.get(`${api}${name}`)
    posts.value = res.data
  } catch (err) {
    console.error(err)
  }
})
</script>

<template>
  <div id="cards">
    <Card
      v-for="p in posts"
      :key="p.id"
      :id="p.id"
      :heading="p.heading"
      :content="p.content"
      v-bind="p.media === 'img' ? { img: p.img } : p.media === 'video' ? { video: p.video } : {}"
    />
  </div>
</template>

<style scoped></style>
