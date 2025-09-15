<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useRoute } from 'vue-router'
import { onMounted, ref } from 'vue'
import axios from 'axios'

const api = import.meta.env.VITE_API

const heading = ref('')
const content = ref('')
const name = ref('')
const media = ref('')
const media_type = ref('')
const created_at = ref('')
const updated_at = ref('')
const mode = ref(false) // eslint-disable-line @typescript-eslint/no-unused-vars

const route = useRoute()
const id = route.params.id

onMounted(async () => {
  try {
    const { data: res } = await axios.get(`${api}/post/${id}`)
    name.value = res.name
    heading.value = res.heading
    content.value = res.content
    media.value = res.media_name
    media_type.value = res.media_type
    created_at.value = res.created_at
    updated_at.value = res.updated_at
  } catch (err) {
    console.error(err)
  }
})

const media_prefix = window.location.origin + '/uploads/'
</script>

<template>
  <main id="post">
    <h1>{{ heading }}</h1>
    <p>{{ content }}</p>
    <div>
      <img v-if="media_type == 'img'" :src="media_prefix + media" alt="" />
      <video
        v-if="media_type == 'vid'"
        controls
        loop
        controlslist="nodownload disablepictureinpicture"
        autoplay
      >
        <source :src="media_prefix + media" type="video/mp4" />
      </video>
    </div>
    <small>Autor: {{ name }}</small>
    <small>Erstellt am: {{ created_at }}</small>
    <small>Aktualisiert am: {{ updated_at }}</small>
  </main>
</template>

<style scoped></style>
