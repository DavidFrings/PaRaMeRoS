import '@/assets/css/main.css'

import { createApp } from 'vue'
import AppMain from './App.vue'
import { createRouter, createWebHistory } from 'vue-router'
import { routes } from 'vue-router/auto-routes'
import { createHead } from '@unhead/vue/client'
import { DataLoaderPlugin } from 'unplugin-vue-router/data-loaders'
import * as CookieConsent from 'vanilla-cookieconsent'
import 'vanilla-cookieconsent/dist/cookieconsent.css'
import '@/assets/css/cookieconsent.css'
import type { App } from 'vue'
import type { CookieConsentConfig } from 'vanilla-cookieconsent'

const name: string = import.meta.env.VITE_NAME
const desc: string = import.meta.env.VITE_DESC
const keywords: string = import.meta.env.VITE_KEYWORDS

const app = createApp(AppMain)
const CookieConsentVue = {
  install: (app: App, pluginConfig: CookieConsentConfig) => {
    app.config.globalProperties.$CookieConsent = CookieConsent
    app.config.globalProperties.$CookieConsent.run(pluginConfig)
  },
}

const consentOptions = {
  root: '#app',
  autoShow: true,
  disablePageInteraction: true,
  revision: 0,
  cookie: {
    name: 'cookie-consent',
  },
  guiOptions: {
    consentModal: {
      layout: 'box',
      position: 'middle center',
      equalWeightButtons: true,
      flipButtons: false,
    },
    preferencesModal: {
      layout: 'box',
      equalWeightButtons: true,
      flipButtons: false,
    },
  },

  categories: {
    necessary: {
      enabled: true,
      readOnly: true,
    },
  },

  language: {
    default: 'de',
    translations: {
      de: {
        consentModal: {
          title: 'Wir nutzen Cookies',
          description:
            'Wir setzen ausschließlich technisch notwendige Cookies ein. Sie sichern Login‑/Sitzungs\-Funktionen, schützen vor Missbrauch und ermöglichen das Verwalten unserer Posts. Keine Tracking- oder Marketing-Cookies.',
          //acceptAllBtn: 'Accept all',
          acceptNecessaryBtn: 'Notwendige akzeptieren',
          showPreferencesBtn: 'Mehr Informationen',
          footer: `
            <a href="/imprint" target="_blank">Impressum</a>
            <a href="/privacy-policy" target="_blank">Datenschutzerklärung</a>
          `,
        },
        preferencesModal: {
          title: 'Cookie Einstellungen verwalten',
          //acceptAllBtn: 'Accept all',
          acceptNecessaryBtn: 'Notwendige akzeptieren',
          //savePreferencesBtn: 'Accept current selection',
          closeIconLabel: 'SChließen',
          sections: [
            {
              title: 'Wir verwenden nur notwendige Cookies',
              description:
                'Diese Cookies sind erforderlich, damit Kernfunktionen (Session, Authentifizierung, CSRF-/Missbrauchsschutz) zuverlässig arbeiten. Ohne sie funktioniert diese Seite nicht korrekt.',
            },
            {
              title: 'Kontoverwaltung',
              description:
                'Sitzungs- und Verifizierungstoken halten Sie angemeldet und schützen unsere Inhalte.',
              linkedCategory: 'necessary',
            },
            {
              title: 'Datenschutzerklärung',
              description:
                'Wir speichern nur minimale, zweckgebundene Daten zur Sicherstellung des Betriebs. Keine Profilbildung, kein Tracking.',
              linkedCategory: 'necessary',
            },
            {
              title: 'Mehr Informationen',
              description:
                'Bei Fragen zu unseren Cookie-Richtlinien oder zu Ihren Einstellungen <a href="/impress">kontaktieren</a> Sie uns',
            },
          ],
        },
      },
    },
  },
}

const router = createRouter({
  history: createWebHistory(),
  routes,
})

const head = createHead({
  init: [
    {
      title: name,
      meta: [
        { name: 'description', content: desc },
        { name: 'keywords', content: keywords },
        { name: 'author', content: 'David Frings' },
        { property: 'og:site_name', content: name },
        { 'http-equiv': 'X-UA-Compatible', content: 'IE=edge' },
      ],
      link: [{ rel: 'manifest', href: '/manifest.webmanifest' }],
    },
  ],
})

app.use(head)
app.use(CookieConsentVue, consentOptions)
app.use(DataLoaderPlugin, { router })
app.use(router)
app.mount('#app')
