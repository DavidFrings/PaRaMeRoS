<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
</script>

<template>
  <main id="404">
    <h1>Not Found</h1>
    <p>{{ route.fullPath }} does not exist.</p>
  </main>
</template>

<style scoped>
main {
  justify-items: center;
  height: 90vh;
  padding-top: 15rem;
}
</style>
