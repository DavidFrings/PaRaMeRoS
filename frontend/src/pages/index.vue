<script setup lang="ts">
import ArrowDown from '@/components/icons/ArrowDown.vue'
import Section from '@/components/Section.vue'
import AboutImg from '@/assets/imgs/intro.webp'
</script>

<template>
  <main id="index">
    <div class="landing">
      <h1>PaRaMeRoS</h1>
      <h3>Technik Teamgeist Innovation</h3>
      <div id="more">
        <p>Mehr</p>
        <ArrowDown />
      </div>
    </div>
    <Section
      heading="About Us"
      btn-text="Mehr erfahren"
      btn-link="/about"
      :img="AboutImg"
      content="Wir sind <b>PaRaMeRoS</b>, das First Lego League Team der <b>Pater Rupert Mayer Realschule</b>. Als engagierte Schüler begeistern wir uns für Robotik und Technologie – mit Kreativität, Teamgeist und Leidenschaft."
    />
    <Section
      selector="reverse"
      heading="Entwicklungsverlauf"
      btn-text="Mehr erfahren"
      btn-link="/progress-logs"
      :img="AboutImg"
      content="Erleben Sie den <b>Fortschritt unseres Teams</b> von den ersten Robotik-Ideen bis zu unseren größten Erfolgen. Hier zeigen wir <b>wichtige Meilensteine, aktuelle Projekte und unsere stetige Weiterentwicklung.</b>"
    />
  </main>
</template>

<style scoped>
.landing {
  padding-top: calc(50vh - 15rem);
  min-height: 50vh;
  width: 100%;
  display: flex;
  align-items: center;
  flex-direction: column;
  text-wrap-mode: nowrap;
  user-select: none;
}

.landing h1 {
  font-size: 7rem;
  color: transparent;
  background: linear-gradient(170deg, var(--primary) 45%, var(--text-muted) 110%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.landing h3 {
  color: var(--text-muted);
}

.landing #more {
  margin-top: 8rem;
  display: flex;
  align-items: center;
  flex-direction: column;
  user-select: none;
}

.landing #more p {
  font-weight: 500;
  color: var(--text-muted);
}

.landing #more svg {
  color: var(--text-muted);
  height: 1.4rem;
}

section {
  margin-top: 10rem;
}
</style>
