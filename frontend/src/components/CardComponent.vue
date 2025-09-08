<script setup lang="ts">
defineProps<{
  selector?: string
  heading: string
  content: string
  id: string
  img?: string
  video?: string
}>()
</script>

<template>
  <section :class="`flex margin-xl ${selector}`">
    <div>
      <h2>{{ heading }}</h2>
      <p v-html="content"></p>
      <a class="btn" :href="`/posts/${id}`">Mehr Erfahren</a>
    </div>
    <img v-if="img && !video" :src="img" />
    <video
      v-if="video && !img"
      controls
      loop
      controlslist="play nodownload"
      :src="video"
      autoplay
    ></video>
  </section>
</template>

<style scoped></style>
