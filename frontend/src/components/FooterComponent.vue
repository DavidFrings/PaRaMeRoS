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
  gap: var(--margin-m);
  margin-bottom: var(--margin-xxl);
  margin-top: var(--margin-xxl);
  padding: var(--margin-l);
  background: var(--surface-muted);
  border: 1px solid var(--border-muted);
  border-radius: var(--radius-l);
  box-shadow: var(--shadow-soft);
  user-select: none;
  text-align: center;
  backdrop-filter: blur(10px);
}

.copyright {
  align-self: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: var(--margin-xs) auto;
}

.url {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: var(--margin-xs);
  text-align: center;
}

.url a {
  padding: var(--margin-xxs) var(--margin-xs);
  border-radius: var(--radius-s);
}

.url a:hover {
  background: hsla(var(--primary-val), 0.12);
}
</style>
