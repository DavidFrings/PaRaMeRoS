<script setup lang="ts">
import Navbar from '@/components/NavbarComponent.vue'
import Footer from '@/components/FooterComponent.vue'
import { onMounted } from 'vue'

onMounted(() => {
  console.info(
    '\n\n\n\n\n\n\n%cPaRaMeRoS-Website\n%cCopyright (C) 2025  David Frings\n\n%cThis program comes with ABSOLUTELY NO WARRANTY;\nThis is free software, and you are welcome to redistribute it under certain conditions;\n%chttps://github.com/DavidFrings/PaRaMeRoS\n\n\n\n\n\n\n',
    'font-size: 3rem; font-weight: bold;',
    'font-size: 2rem;',
    'font-size: 1.2rem;',
    'font-size: 1.2rem; text-decoration: none; color: var(--primary);',
  )
})
</script>

<template>
  <Navbar />
  <router-view />
  <Footer />
</template>
