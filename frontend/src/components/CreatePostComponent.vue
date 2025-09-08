<script setup lang="ts">
import { ref } from 'vue'
import VueCookies from 'vue-cookies'
const Cookies = VueCookies.VueCookies
import axios from 'axios'
import UploadIcon from '@/components/icons/UploadIcon.vue'

const props = defineProps<{
  name: string
}>()

const heading = ref('')
const content = ref('')
const media = ref<File | null>(null)

const api = import.meta.env.VITE_API

async function handleSubmit(e: Event) {
  e.preventDefault()

  const formData = new FormData()
  formData.append('name', props.name)
  formData.append('token', Cookies.get('auth_token'))
  formData.append('heading', heading.value)
  formData.append('content', content.value)

  if (media.value) {
    const filename = Date.now() + media.value.name
    formData.append('mediaName', filename)
    formData.append('media', media.value)
  }

  try {
    const res = await axios.post(`${api}newPost`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
    window.location.replace('/post/' + res.data.id)
  } catch (err) {
    console.error(err)
  }
}

function handleFileChange(e: Event) {
  const target = e.target as HTMLInputElement
  if (target.files && target.files[0]) {
    media.value = target.files[0]
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

function createObjectURL(file: File): string {
  return window.URL.createObjectURL(file)
}
</script>

<template>
  <div id="create-post">
    <form @submit="handleSubmit">
      <label for="file" class="file-label">
        <UploadIcon />
      </label>
      <input id="file" class="file-input" type="file" accept="image/*" @change="handleFileChange" />
      <input
        class="heading"
        type="text"
        placeholder="Heading"
        autofocus
        @change="handleHeadingChange"
      />
      <textarea class="content" placeholder="Content" autofocus @change="handleContentChange" />
      <input type="submit" value="Veröffentlichen" id="submit" />
    </form>
    <img v-if="media" :src="createObjectURL(media)" />
  </div>
</template>

<style scoped></style>
