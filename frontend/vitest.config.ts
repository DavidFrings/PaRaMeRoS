import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vitest/config'
import VueRouter from 'unplugin-vue-router/vite'
import Vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [VueRouter({ extensions: ['.vue'], importMode: 'async', logs: false }), Vue()],
  resolve: {
    alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) },
  },
  test: {
    environment: 'jsdom',
    include: ['src/**/__tests__/*.test.ts'],
    passWithNoTests: true,
  },
})
