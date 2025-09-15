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
const mediaType = ref<'img' | 'vid' | null>(null)

const api = import.meta.env.VITE_API

async function handleSubmit(e: Event) {
  e.preventDefault()

  const formData = new FormData()
  formData.append('name', props.name)
  formData.append('heading', heading.value)
  formData.append('content', content.value)

  if (media.value && mediaType.value) {
    formData.append('media', media.value)
    formData.append('media_type', mediaType.value)
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
    console.error(err)
  }
}

function handleFileChange(e: Event) {
  const target = e.target as HTMLInputElement
  if (target.files && target.files[0]) {
    const file = target.files[0]
    media.value = file

    if (file.type.startsWith('image/')) {
      mediaType.value = 'img'
    } else if (file.type.startsWith('video/')) {
      mediaType.value = 'vid'
    } else {
      mediaType.value = null
      media.value = null
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
      <input
        id="file"
        class="file-input"
        type="file"
        accept="image/*, video/*"
        @change="handleFileChange"
      />
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
    <img v-if="media && mediaType === 'img'" :src="createObjectURL(media)" />
    <video v-if="media && mediaType === 'vid'" :src="createObjectURL(media)" controls />
  </div>
</template>

<style scoped></style>
