<script setup lang="ts">
const currentYear = new Date().getFullYear()
</script>

<template>
  <footer id="footer">
    <div class="copyright">
      <p>Copyright &#169; 2023-{{ currentYear }} PaRaMeRoS - David Frings</p>
      <p><b>Alle Rechte vorbehalten</b></p>
    </div>
    <div class="url">
      <a href="/?skipAnimation=true">Home</a>
      <a href="/about">Über Uns</a>
      <a href="/progress-logs">Entwicklung</a>
      <a href="/prmrs/forest-project">Wald Projekt</a>
      <a href="/imprint">Impressum</a>
      <a href="/privacy-policy">Datenschutz</a>
      <a data-cc="show-preferencesModal">Cookies</a>
      <a href="/sitemap.xml">Sitemap</a>
    </div>
  </footer>
</template>

<style scoped>
footer {
  position: relative;
  bottom: 0;
  display: flex;
  flex-direction: column;
  margin-bottom: var(--margin-xxl);
  user-select: none;
  text-align: center;
}

.copyright {
  align-self: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: var(--margin-xl) auto;
}

.url {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 0.8rem;
  text-align: center;
}
</style>
