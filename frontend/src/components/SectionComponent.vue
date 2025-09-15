<script setup lang="ts">
defineProps<{
  selector?: string
  heading: string
  content: string
  btnText?: string
  btnLink?: string
  media_type?: 'img' | 'vid'
  media?: string
}>()
</script>

<template>
  <section :class="`flex margin-xl ${selector}`">
    <div>
      <h2>{{ heading }}</h2>
      <p v-html="content"></p>
      <a class="btn" v-if="btnText" :href="btnLink">{{ btnText }}</a>
    </div>
    <img v-if="media_type == 'img'" :src="media" alt="" class="media" />
    <video
      v-if="media_type == 'vid'"
      controls
      loop
      controlslist="nodownload"
      autoplay
      class="media"
    >
      <source :src="media" type="video/mp4" />
    </video>
  </section>
</template>

<style scoped>
div {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  text-wrap: auto;
}

p {
  margin-top: var(--margin-xs);
}

a {
  margin-top: var(--margin-m);
}

.media {
  aspect-ratio: 16/9;
}
</style>
