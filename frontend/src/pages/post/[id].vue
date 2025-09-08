<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useRoute } from 'vue-router'
import { ref } from 'vue'

const post = ref({})
const heading = ref('')
const content = ref('')
const mode = ref(false)

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
