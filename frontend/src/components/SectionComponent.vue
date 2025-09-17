<script setup lang="ts">
defineProps<{
  selector?: string
  heading: string
  content: string
  btnText?: string
  btnLink?: string
  media_type?: 'img' | 'vid'
  media?: string
  creator?: string
}>()
</script>

<template>
  <section :class="`flex margin-xl ${selector}`">
    <div>
      <h2>{{ heading }}</h2>
      <p v-html="content"></p>
      <a class="btn" v-if="btnText" :href="btnLink">{{ btnText }}</a>
    </div>
    <div id="media-div">
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
      <p v-if="media" id="creator">by: {{ creator }}</p>
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

p {
  margin-top: var(--margin-xs);
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
