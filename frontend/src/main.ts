import '@/assets/css/app.css'
import 'vanilla-cookieconsent/dist/cookieconsent.css'
import '@/assets/css/cookieconsent.css'

import { createApp } from 'vue'
import AppMain from './App.vue'
import { createWebHistory } from 'vue-router'
import { createAppRouter } from './router'
import { i18n } from '@/i18n'
import { createHead } from '@unhead/vue/client'
import * as CookieConsent from 'vanilla-cookieconsent'
import { consentOptions } from '@/cookieconsent'

const name = window.__ENV__.NAME
const desc = window.__ENV__.DESC
const keywords = window.__ENV__.KEYWORDS

const app = createApp(AppMain)

const router = createAppRouter(createWebHistory())

const head = createHead({
  init: [
    {
      title: name,
      meta: [
        { name: 'description', content: desc },
        { name: 'keywords', content: keywords },
        { name: 'author', content: 'David Frings' },
        { property: 'og:site_name', content: name },
      ],
      link: [{ rel: 'manifest', href: '/manifest.webmanifest' }],
    },
  ],
})

app.use(head)
app.use(i18n)
app.use(router)
app.mount('#app')
CookieConsent.run(consentOptions)
