import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'

import Vue from '@vitejs/plugin-vue'
import VueDevTools from 'vite-plugin-vue-devtools' // eslint-disable-line @typescript-eslint/no-unused-vars
import VueRouter from 'unplugin-vue-router/vite'
import { VitePWA } from 'vite-plugin-pwa'
import Tailwind from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  clearScreen: false,
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  build: {
    sourcemap: true,
  },
  plugins: [
    VueRouter({
      extensions: ['.vue'],
      importMode: 'async',
      logs: false,
    }),
    Vue(),
    Tailwind(),
    //VueDevTools(),
    VitePWA({
      registerType: 'autoUpdate',
      workbox: { globIgnores: ['**/config.js'] }, // config.js is runtime config, never precache it
      manifest: {
        name: 'PaRaMeRoS',
        short_name: 'PaRaMeRoS',
        start_url: '/',
        theme_color: '#161513',
        background_color: '#161513',
        prefer_related_applications: true,
      },
    }),
  ],
})
