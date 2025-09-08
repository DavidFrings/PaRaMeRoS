<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useRoute } from 'vue-router'
import { ref } from 'vue'

const post = ref({})    // eslint-disable-line @typescript-eslint/no-unused-vars
const heading = ref('') // eslint-disable-line @typescript-eslint/no-unused-vars
const content = ref('') // eslint-disable-line @typescript-eslint/no-unused-vars
const mode = ref(false) // eslint-disable-line @typescript-eslint/no-unused-vars

const route = useRoute()
const id = route.params.id
console.log(id)
</script>

<template>
  <main id="post">
    {{ id }}
  </main>
</template>

<style scoped></style>
