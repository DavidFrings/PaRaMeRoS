<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import ArrowDown from '@/components/icons/ArrowDownIcon.vue'
import Section from '@/components/SectionComponent.vue'
import AboutImg from '@/assets/imgs/room.webp'
import ErfolgeImg from '@/assets/imgs/erfolge.webp'
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'

const skipAnimations = ref(false)

onMounted(() => {
  const route = useRoute()
  skipAnimations.value = route.query.skipAnimation === 'true'

  if (!skipAnimations.value) {
    window.scrollTo(0, 0)
    document.body.classList.add('no-scroll')

    const timer = setTimeout(() => {
      document.body.classList.remove('no-scroll')
    }, 4010)

    onBeforeUnmount(() => {
      clearTimeout(timer)
      document.body.classList.remove('no-scroll')
    })
  }
})
</script>

<template>
  <main id="index" :class="{ 'skip-animations': skipAnimations }">
    <div class="nav-block"></div>
    <div class="landing">
      <h1>PaRaMeRoS</h1>
      <h3>
        <span>Technik </span>
        <span>Teamgeist </span>
        <span>Innovation</span>
      </h3>
      <div id="more">
        <p>Mehr</p>
        <ArrowDown />
      </div>
    </div>
    <div class="content">
      <Section
        heading="About Us"
        btn-text="Mehr erfahren"
        btn-link="/about"
        media_type="img"
        :media="AboutImg"
        creator="Privat"
        content="Wir sind <b>PaRaMeRoS</b>, das First Lego League Team der <b>Pater Rupert Mayer Realschule</b>. Als engagierte Schüler begeistern wir uns für Robotik und Technologie – mit Kreativität, Teamgeist und Leidenschaft."
      />
      <Section
        selector="reverse"
        heading="Entwicklungsverlauf"
        btn-text="Mehr erfahren"
        btn-link="/progress-logs"
        media_type="img"
        :media="ErfolgeImg"
        creator="Privat"
        content="Erleben Sie den <b>Fortschritt unseres Teams</b> von den ersten Robotik-Ideen bis zu unseren größten Erfolgen. Hier zeigen wir <b>wichtige Meilensteine, aktuelle Projekte und unsere stetige Weiterentwicklung.</b>"
      />
      <Section
        heading="PRMRS - Projekt der 9. Klassen"
        btn-text="Mehr erfahren"
        btn-link="/prmrs/forest-project"
        content="text"
      />
      <!-- media_type="img"
        :media=""
        creator="Privat" -->
    </div>
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
  background: linear-gradient(
    160deg,
    var(--primary) 40%,
    hsl(79deg 69.87% 56.03%) 75%,
    var(--secondary) 95%
  );
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  opacity: 0;
  transform: translateY(var(--margin-m));
  animation: fadeInUp 0.6s forwards;
  animation-delay: 0.2s;
}

.landing h3 {
  color: var(--text-muted);
}

.landing h3 span {
  opacity: 0;
  transform: translateY(var(--margin-m));
  animation: fadeInUp 0.6s forwards;
}

.landing h3 span:nth-child(1) {
  animation-delay: 1s;
}
.landing h3 span:nth-child(2) {
  animation-delay: 1.5s;
}
.landing h3 span:nth-child(3) {
  animation-delay: 2s;
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
  opacity: 0;
  transform: translateY(var(--margin-m));
  animation: fadeInUp 0.6s forwards;
  animation-delay: 2.7s;
}

.landing #more svg {
  color: var(--text-muted);
  height: 1.4rem;
  opacity: 0;
  transform: translateY(var(--margin-m));
  animation: fadeInUp 0.6s forwards;
  animation-delay: 3s;
}

section {
  margin-top: 10rem;
}

.content {
  opacity: 0;
  transform: translateY(var(--margin-m));
  animation: fadeInUp 0.6s forwards;
  animation-delay: 4s;
}

.nav-block {
  position: fixed;
  top: 0;
  height: 6rem;
  width: 100%;
  background: var(--bg-dark);
  left: 0;
  z-index: 3;
  opacity: 1;
  animation: fadeOut 0.6s forwards;
  animation-delay: 4s;
}

@keyframes fadeInUp {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes fadeOut {
  to {
    opacity: 0;
    z-index: 0;
  }
}

.skip-animations .landing h1,
.skip-animations .landing h3 span,
.skip-animations .landing #more p,
.skip-animations .landing #more svg,
.skip-animations .content {
  opacity: 1;
  transform: translateY(0);
  animation: none;
}

.skip-animations .nav-block {
  opacity: 0;
  z-index: 0;
  animation: none;
}
</style>
