<script setup lang="ts">
import Navbar from '@/components/NavbarComponent.vue'
import Footer from '@/components/FooterComponent.vue'
import { onMounted } from 'vue'

onMounted(() => {
  console.warn(
    '\nPaRaMeRoS-Website  Copyright (C) 2025  David Frings\n\n' +
      'This program comes with ABSOLUTELY NO WARRANTY;\n' +
      'This is free software, and you are welcome to redistribute it under certain conditions;\n\n' +
      'https://github.com/DavidFrings/PaRaMeRoS',
  )
})
</script>

<template>
  <Navbar />
  <router-view />
  <Footer />
</template>
