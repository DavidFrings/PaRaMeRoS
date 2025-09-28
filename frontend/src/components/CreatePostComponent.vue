<script setup lang="ts">
import { ref } from 'vue'
import axios from 'axios'
import UploadIcon from '@/components/icons/UploadIcon.vue'
import { useCookies } from 'vue3-cookies'
const { cookies } = useCookies()

const props = defineProps<{
  name: string
}>()

const heading = ref('')
const content = ref('')
const media = ref<File | null>(null)
const media_type = ref<'img' | 'vid' | null>(null)
const media_creator = ref('Privat')

const api = window.__ENV__.API

async function handleSubmit(e: Event) {
  e.preventDefault()
  ;(document.getElementById('submit') as HTMLInputElement).disabled = true

  const trans_content = content.value.replace(/\n/g, '<br>').replace(/\*([^*]+)\*/g, '<b>$1</b>')

  const formData = new FormData()
  formData.append('name', props.name)
  formData.append('heading', heading.value)
  formData.append('content', trans_content)

  if (media.value && media_type.value && media_creator.value) {
    formData.append('media', media.value)
    formData.append('media_type', media_type.value)
    formData.append('media_creator', media_creator.value)
  }

  try {
    const res = await axios.post(`${api}/post`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
        Authorization: cookies.get('auth_token'),
      },
    })
    window.location.replace('/post/' + res.data.id)
  } catch (err: unknown) {
    ;(document.getElementById('submit') as HTMLInputElement).disabled = false
    console.error(err)
  }
}

function handleFileChange(e: Event) {
  const target = e.target as HTMLInputElement
  if (target.files && target.files[0]) {
    const file = target.files[0]
    media.value = file

    if (file.type.startsWith('image/')) {
      media_type.value = 'img'
    } else if (file.type.startsWith('video/')) {
      media_type.value = 'vid'
    } else {
      media_type.value = null
      media.value = null
      media_creator.value = ''
    }
  }
}

function handleHeadingChange(e: Event) {
  const target = e.target as HTMLInputElement
  heading.value = target.value
}

function handleContentChange(e: Event) {
  const target = e.target as HTMLTextAreaElement
  content.value = target.value
}

function handleCreatorChange(e: Event) {
  const target = e.target as HTMLTextAreaElement
  media_creator.value = target.value
}

function createObjectURL(file: File): string {
  return window.URL.createObjectURL(file)
}
</script>

<template>
  <section id="create-post" :class="`flex margin-xxl column-reverse`">
    <div>
      <form @submit="handleSubmit">
        <div id="file-div">
          <label for="file" class="file-label">
            <UploadIcon />
          </label>
          <input
            id="file"
            class="file-input"
            type="file"
            accept="image/*, video/*"
            @change="handleFileChange"
          />
          <input
            class="media-creator"
            type="text"
            placeholder="Bild-/Video Urheber (ansonsten: Privat)"
            @change="handleCreatorChange"
          />
        </div>
        <input
          class="heading"
          type="text"
          placeholder="Überschrift"
          autofocus
          @change="handleHeadingChange"
        />
        <textarea
          class="content"
          placeholder="Dein Beitrag"
          autofocus
          @change="handleContentChange"
        />
        <input type="submit" value="Veröffentlichen" id="submit" />
      </form>
    </div>
    <div v-if="media" id="media-div">
      <img v-if="media_type === 'img'" :src="createObjectURL(media)" class="media" />
      <video v-if="media_type === 'vid'" :src="createObjectURL(media)" class="media" controls />
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
  width: 100%;
}

p {
  overflow-wrap: break-word;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  overflow: hidden;
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

.file-label {
  position: relative;
  display: flex;
  width: 3rem;
  cursor: pointer;
}

.file-input {
  display: none;
}

#file-div {
  display: flex;
  flex-direction: row;
  align-items: center;
  width: 100%;
  gap: var(--margin-xs);
}

input[type='text'],
textarea {
  background: var(--bg);
  border: none;
  border-radius: var(--margin-s);
  outline: none;
  color: var(--text);
  resize: none;
  min-height: var(--margin-l);
  width: 100%;
  padding: var(--margin-xxs) var(--margin-s);
}

.media-creator,
textarea {
  font: var(--p-big);
}

textarea {
  min-height: calc(12 * var(--margin-xxl));
  overflow: auto;
  scrollbar-width: thin;
  scrollbar-color: var(--text-muted) transparent;
  scroll-behavior: smooth;
}

.heading,
#file-div,
textarea {
  margin-bottom: var(--margin-m);
}

.heading {
  overflow-wrap: break-word;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  font: var(--h2);
}

form {
  width: 100%;
}

#submit {
  background: linear-gradient(
    150deg,
    var(--primary) 15%,
    hsl(79deg 69.87% 56.03%) 70%,
    var(--secondary) 95%
  );
  border: none;
  border-radius: var(--margin-xs);
  outline: none;
  color: #f2f2f2ff;
  font: var(--p-big);
  font-weight: bold;
  padding: var(--margin-xxs) var(--margin-xl);
  position: relative;
  justify-self: flex-end;
  cursor: pointer;
  display: flex;
}
</style>
