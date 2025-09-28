<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import Section from '@/components/SectionComponent.vue'
import CoachesImg from '@/assets/imgs/coaches.webp'
import SiebenImg from '@/assets/imgs/2024-25.webp'
import SechsImg from '@/assets/imgs/2023-24.webp'
import FuenfImg from '@/assets/imgs/2022-23.webp'
import VierImg from '@/assets/imgs/2021-22.webp'
import DreiImg from '@/assets/imgs/2020-21.webp'
import ZweiImg from '@/assets/imgs/2019-20.webp'
import EinsImg from '@/assets/imgs/2017-18.webp'
</script>

<template>
  <main id="about">
    <Section
      heading="About Us"
      content="
        Unser Team besteht aus engagierten Schülerinnen und Schülern, die sich für <b>Robotik</b> und <b>Technologie</b> begeistern.
        Wir arbeiten intensiv daran, unser Wissen und unsere Fähigkeiten zu erweitern und uns bestmöglich auf die Wettbewerbe vorzubereiten.
        In den vergangenen Jahren hat die <b>Pater-Rupert-Mayer-Realschule</b> bereits mit vielen Teams erfolgreich an der <b>First Lego League</b> teilgenommen.
        Unsere Vorgängerinnen und Vorgänger konnten zahlreiche Preise gewinnen und wertvolle Erfahrungen sammeln, von denen auch wir profitieren.
        Unser Ziel ist es, uns stetig zu verbessern und immer anspruchsvollere Projekte umzusetzen. Dabei hilft uns besonders der Austausch mit früheren Teammitgliedern, die ihre Erfahrungen mit uns teilen.
        Neben technischen Fähigkeiten lernen wir auch wichtige Soft Skills wie <b>Teamarbeit</b>, <b>Kommunikation</b> und <b>Zeitmanagement</b>.
        <b>Wir sind stolz darauf, Teil dieses First Lego League Teams PaRaMeRoS zu sein und zu zeigen, was wir gemeinsam erreichen können!</b>
      "
      btn-text="Zur Schulhomepage"
      btn-link="https://www.prmrs.de"
    />
    <Section
      heading="Unsere Coaches"
      media_type="img"
      :media="CoachesImg"
      media_creator="Privat"
      content="
        Unser Coach, <b>Herr v. Essen</b>, ist die treibende Kraft hinter dem Erfolg von <b>PaRaMeRoS</b> und seit Beginn dabei.
        Seit der Gründung 2017 begleitet er das Team als erfahrener Mentor.
        Mit seiner Expertise vermittelt er <b>technisches Wissen</b> und stärkt zugleich <b>Teamgeist</b> und <b>Leidenschaft</b>.
        Ebenso wichtig ist die Unterstützung unserer Co-Coachin, <b>Frau Knauke</b>, die eng mit ihm zusammenarbeitet.
      "
    />

    <h1>Unsere Jahrgänge</h1>

    <Section
      selector="reverse"
      heading="2024 / 25"
      content="
        Auch dieses Jahr nahmen wir wieder an der First-Lego-League teil. Unser Team „PaRaMeRoS“ bestand
        aus Paula Affolter, Alex Brandhof, Raphael Cischek, Leo Graner, Benedikt Huber und Christian Stecher.
        Am Samstag, den 25.01.2025, traf sich das Team und bereitete den Tisch vor. Nach der Eröffnung starteten wir
        Übungsrunden mit unserem Roboter, bevor die Testrunde begann. Schnell wurde klar, dass Optimierungen nötig waren.
      "
      media_type="img"
      :media="SiebenImg"
      media_creator="Privat"
    />
    <Section
      selector="reverse"
      heading="2023 / 24"
      content="
        In der Saison 2023/24 entwickelten Maximilian Gründinger, David Frings, Carl, Alex, Anna, Maria, Livia, Nathan, Noah & Sebastian
        nicht nur diese Website, sondern auch innovative Lösungen für die Herausforderungen der FIRST Lego League 2023/24. Maximilian
        programmierte dabei eigenständig eine KI, die einen Pathfinder für den Roboter implementiert und den Dijkstra-Algorithmus verwendet.
      "
      btn-text="Zum Algorithmus"
      btn-link="https://github.com/AIIrondev/PaRaMeRoS-Algorithmus/wiki"
      media_type="img"
      :media="SechsImg"
      media_creator="Privat"
    />
    <Section
      selector="reverse"
      heading="2022 / 23"
      content="
        In der Saison 2022/23 zeigten sich Maximilian, David, Carl, Alex, Anna,
        Maria, Livia, Noah & Sebastian
        als das PaRaMeRoS Team. Gemeinsam setzen sie die Tradition der Exzellenz fort
        und nahmen die Herausforderungen
        der FIRST Lego League 2022/23 an. Unsere Hingabe und Leidenschaft für Technologie &
        Teamarbeit die das Team in dieser Zeit entdeckte ebneten den Weg für neue Erfolge.
      "
      media_type="img"
      :media="FuenfImg"
      media_creator="Privat"
    />
    <Section
      selector="reverse"
      heading="2021 / 22"
      content="
        In der Saison 2021/22 erzielten Lena, Romina, Simeon & Tobias herausragende Erfolge.
        Sie gewannen den <b>Hauptpreis in Roboter-Design</b> und den <b>3. Platz in Robot-Game & Grundwerte</b>.
        Diese Auszeichnungen sind Meilensteine und Anerkennung für jeden Teilnehmer.
        Nach dem regionalen Wettbewerb und dem Feiern unserer Erfolge erlebten wir jedoch auch Fehler... und lernten daraus.
      "
      media_type="img"
      :media="VierImg"
      media_creator="Privat"
    />
    <Section
      selector="reverse"
      heading="2020 / 21"
      content="
        Trotz globaler Herausforderungen im Jahr 2020/21 ließen sich das PaRaMeRoS Team nicht entmutigen.
        Lena, Romina, Simeon & Tobias meisterten Herausforderungen und sammelten wertvolle Erfahrungen,
        auch wenn keine spezifischen Preise gewonnen wurden.
      "
      media_type="img"
      :media="DreiImg"
      media_creator="Privat"
    />
    <Section
      selector="reverse"
      heading="2019 / 20"
      content="
        Nachdem wir 2018/19 wegen eines Schneesturms nicht teilnehmen konnten,
        konzentrierten sich Marcel, Kyan Amin, Konstantin, Markus, Moritz, Leonardo, Constantin, Phillip & Raphael
        in der Saison 2019/20 auf kontinuierliche Verbesserung und den Austausch von Erfahrungen.
        Ihr Teamgeist wurde gestärkt, und sie erkannten den Wert von Fähigkeiten wie Teamarbeit und Kommunikation.
        "
      media_type="img"
      :media="ZweiImg"
      media_creator="Privat"
    />
    <Section
      selector="reverse"
      heading="2017 / 18"
      content="
        In der Saison 2017/18 wagten Benedikt, Jonas, Robert, Jakob, Clemens, Kilian, Dominic, Carl
        & Sebastian als Teilnehmer vom
        PaRaMeRoS Team den ersten Schritt in die Welt der FIRST Lego League. Ihre Begeisterung für
        Robotik entfaltete sich,
        und gemeinsam legten sie den Grundstein für kommende Jahre.
      "
      media_type="img"
      :media="EinsImg"
      media_creator="Privat"
    />
  </main>
</template>

<style scoped>
h1 {
  margin: 6rem 0;
  justify-self: center;
}
</style>
