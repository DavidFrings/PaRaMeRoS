<script setup lang="ts">
import { onMounted } from 'vue'
import SiteHeader from '@/components/layout/SiteHeader.vue'
import SiteFooter from '@/components/layout/SiteFooter.vue'
import SkipLink from '@/components/layout/SkipLink.vue'

onMounted(() => {
  console.info(
    '\n%cPaRaMeRoS-Website\n%cCopyright (C) 2025  David Frings\n\n%cThis program comes with ABSOLUTELY NO WARRANTY;\nThis is free software, and you are welcome to redistribute it under certain conditions;\n%chttps://github.com/DavidFrings/PaRaMeRoS\n',
    'font-size: 3rem; font-weight: bold;',
    'font-size: 2rem;',
    'font-size: 1.2rem;',
    'font-size: 1.2rem; text-decoration: none; color: #f17a2e;',
  )
})
</script>

<template>
  <SkipLink />
  <SiteHeader />
  <main id="main" tabindex="-1" class="outline-none">
    <RouterView />
  </main>
  <SiteFooter />
</template>
