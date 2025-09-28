<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  selector?: string
  heading: string
  content: string
  btnText?: string
  btnLink?: string
  media_type?: 'img' | 'vid'
  media?: string
  media_creator?: string
}>()

const validateMedia = computed(() => {
  const hasMedia = !!props.media
  const hasType = !!props.media_type
  const hasCreator = !!props.media_creator

  return {
    isValid: (!hasMedia && !hasType && !hasCreator) || (hasMedia && hasType && hasCreator),
    error:
      (hasMedia || hasType || hasCreator) && !(hasMedia && hasType && hasCreator)
        ? 'All media attributes must be given! (media, media_type und media_creator)'
        : null,
  }
})
</script>

<template>
  <section :class="`flex margin-xxl ${selector}`">
    <div>
      <h2>{{ heading }}</h2>
      <p v-html="content" :class="{ 'extended-content': !media }"></p>
      <a class="btn" v-if="btnText" :href="btnLink">{{ btnText }}</a>
    </div>
    <div v-if="validateMedia.isValid && media" id="media-div">
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
      <p v-if="media" id="creator">by: {{ media_creator }}</p>
    </div>
  </section>
</template>

<style scoped>
div {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  text-wrap: auto;
}

h2 {
  overflow-wrap: break-word;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

p {
  margin-top: var(--margin-xs);
  overflow-wrap: break-word;
  display: -webkit-box;
  -webkit-line-clamp: 6;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.extended-content {
  -webkit-line-clamp: 8;
}

a {
  margin-top: var(--margin-m);
}

.media {
  aspect-ratio: 16/9;
}

#creator {
  position: absolute;
  bottom: var(--margin-s);
  left: var(--margin-xs);
  margin: 0;
  background: hsla(var(--bg-dark-val), 0.45);
  padding: var(--margin-xxs) var(--margin-s);
  border-radius: var(--margin-s);
  font-size: 0.9rem;
}

#media-div {
  display: inline-block;
  position: relative;
}
</style>
