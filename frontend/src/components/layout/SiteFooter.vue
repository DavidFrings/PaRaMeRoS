<script setup lang="ts">
const year = new Date().getFullYear()
</script>

<template>
  <footer
    class="mx-auto mt-16 flex max-w-[1200px] flex-col gap-3 border-t border-(--line) px-5 py-8 text-[13px] text-(--muted) md:flex-row md:items-center md:justify-between md:px-8 lg:mt-24"
  >
    <p>© 2023–{{ year }} PaRaMeRoS · Pater-Rupert-Mayer-Realschule</p>
    <div class="flex flex-wrap gap-x-5">
      <RouterLink to="/Imprint" class="inline-flex min-h-11 items-center hover:text-(--text)"
        >Impressum</RouterLink
      >
      <RouterLink to="/Privacy-Policy" class="inline-flex min-h-11 items-center hover:text-(--text)"
        >Datenschutz</RouterLink
      >
      <a href="/sitemap.xml" class="inline-flex min-h-11 items-center hover:text-(--text)"
        >Sitemap</a
      >
      <RouterLink
        to="/prmrs/forest-project"
        class="inline-flex min-h-11 items-center hover:text-(--text)"
        >Wald-Projekt (9. Klassen PRMRS)</RouterLink
      >
    </div>
  </footer>
</template>
