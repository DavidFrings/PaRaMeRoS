<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { Menu, X } from 'lucide-vue-next'
import Logo from '@/assets/imgs/logo.webp'

const route = useRoute()
const open = ref(false)
watch(
  () => route.fullPath,
  () => (open.value = false),
)

const links = [
  { to: '/', label: 'Start' },
  { to: '/About', label: 'Über uns' },
  { to: '/team', label: 'Team' },
  { to: '/logbuch', label: 'Logbuch' },
]

const isActive = (to: string) =>
  to === '/' ? route.path === '/' : route.path.toLowerCase().startsWith(to.toLowerCase())
</script>

<template>
  <header class="mx-auto max-w-[1200px] px-4 md:px-8">
    <div class="flex h-16 items-center justify-between">
      <RouterLink to="/" class="flex min-h-11 items-center gap-3 rounded-[10px]">
        <img :src="Logo" alt="" class="h-9 w-9 rounded-[8px] object-contain" />
        <span class="text-[17px] font-semibold">PaRaMeRoS</span>
      </RouterLink>
      <nav aria-label="Hauptnavigation" class="hidden items-center gap-8 text-[15px] lg:flex">
        <RouterLink
          v-for="l in links"
          :key="l.to"
          :to="l.to"
          :aria-current="isActive(l.to) ? 'page' : undefined"
          class="transition-colors hover:text-(--text)"
          :class="isActive(l.to) ? 'text-(--text)' : 'text-(--text-2)'"
          >{{ l.label }}</RouterLink
        >
      </nav>
      <div class="flex items-center gap-1">
        <button
          type="button"
          :aria-label="open ? 'Menü schließen' : 'Menü'"
          :aria-expanded="open"
          aria-controls="mobile-nav"
          class="grid h-11 w-11 place-items-center rounded-[10px] text-(--text) transition-colors hover:bg-(--raised) lg:hidden"
          @click="open = !open"
        >
          <X v-if="open" :size="18" :stroke-width="1.5" aria-hidden="true" />
          <Menu v-else :size="18" :stroke-width="1.5" aria-hidden="true" />
        </button>
      </div>
    </div>
    <nav
      v-show="open"
      id="mobile-nav"
      aria-label="Hauptnavigation"
      class="mb-4 grid rounded-[14px] border border-(--line) bg-(--card) py-1 lg:hidden"
    >
      <RouterLink
        v-for="l in links"
        :key="l.to"
        :to="l.to"
        :aria-current="isActive(l.to) ? 'page' : undefined"
        class="flex h-12 items-center px-4 text-[15px]"
        :class="isActive(l.to) ? 'text-(--text)' : 'text-(--text-2)'"
        >{{ l.label }}</RouterLink
      >
    </nav>
  </header>
</template>
