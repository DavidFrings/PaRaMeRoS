<script setup lang="ts">
import Section from '@/components/SectionComponent.vue'

defineProps<{
  selector?: string
  heading: string
  content: string
  id: string
  media_type?: 'img' | 'vid'
  media?: string
}>()

const media_prefix = window.location.origin + '/uploads/'
</script>

<template>
  <Section
    :heading="heading"
    :content="content"
    :btn-link="`/post/${id}`"
    btn-text="Mehr Erfahren"
    :media_type="media_type"
    :media="media_prefix + media"
  />
</template>

<style scoped></style>
