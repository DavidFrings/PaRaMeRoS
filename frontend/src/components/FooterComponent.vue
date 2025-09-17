<script setup lang="ts"></script>

<template>
  <footer id="footer">
    <div class="copyright">
      <p>Copyright &#169; 2024-2025 PaRaMeRoS - David Frings</p>
      <p>Alle Rechte vorbehalten</p>
    </div>
    <div class="url">
      <a href="/">Home</a>
      <a href="/about">Über Uns</a>
      <a href="/progress-logs">Entwicklung</a>
      <a href="/prmrs/forest-project">Wald Projekt</a>
      <span></span>
      <a href="/imprint">Impressum</a>
      <a href="/privacy-policy">Datenschutz</a>
      <a href="/cookies">Cookies</a>
      <a href="/sitemap">Sitemap</a>
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
  gap: 0.4rem;
  text-align: center;
}
</style>
