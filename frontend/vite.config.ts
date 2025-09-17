import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'

import Vue from '@vitejs/plugin-vue'
import VueDevTools from 'vite-plugin-vue-devtools'
import VueRouter from 'unplugin-vue-router/vite'
import { VitePWA } from 'vite-plugin-pwa'

// https://vite.dev/config/
export default defineConfig({
  clearScreen: false,
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
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
    VueDevTools(),
    VitePWA({
      registerType: 'autoUpdate',
      manifest: {
        name: process.env.VITE_NAME,
        short_name: process.env.VITE_NAME,
        start_url: "/",
        theme_color: 'transparent',
        background_color: 'transparent',
        prefer_related_applications: true
      }
    })
  ]
})
