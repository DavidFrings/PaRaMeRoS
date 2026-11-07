<script setup lang="ts">
import { ref, watch, useTemplateRef } from 'vue'
import { useRoute } from 'vue-router'
import { Menu, X } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'
import { useLocale } from '@/composables/useLocale'
import LiveBadge from '@/components/LiveBadge.vue'
import Logo from '@/assets/imgs/logo.webp'

const route = useRoute()
const open = ref(false)
watch(
  () => route.fullPath,
  () => (open.value = false),
)

const { t } = useI18n()
const { locale, localePath, switchTo } = useLocale()
const menuButton = useTemplateRef('menuButton')

function closeOnEscape() {
  if (!open.value) return
  open.value = false
  menuButton.value?.focus()
}

const links = [
  { to: '/', label: 'nav.start' },
  { to: '/About', label: 'nav.about' },
  { to: '/team', label: 'nav.team' },
  { to: '/logbuch', label: 'nav.logbuch' },
]

const isActive = (to: string) => {
  const trim = (x: string) => x.toLowerCase().replace(/(?<=.)\/$/, '')
  const p = trim(localePath(to))
  const cur = trim(route.path)
  return to === '/' ? cur === p : cur.startsWith(p)
}
</script>

<template>
  <header class="mx-auto w-full max-w-[1200px] px-4 md:px-8" @keydown.esc="closeOnEscape">
    <div class="flex h-16 items-center justify-between">
      <RouterLink :to="localePath('/')" class="flex min-h-11 items-center gap-3 rounded-[10px]">
        <img :src="Logo" alt="" class="h-9 w-9 rounded-[8px] object-contain" />
        <span class="text-[17px] font-semibold">PaRaMeRoS</span>
      </RouterLink>
      <nav :aria-label="t('nav.main')" class="hidden items-center gap-8 text-[15px] lg:flex">
        <RouterLink
          v-for="l in links"
          :key="l.to"
          :to="localePath(l.to)"
          :aria-current="isActive(l.to) ? 'page' : undefined"
          class="transition-colors hover:text-(--text)"
          :class="isActive(l.to) ? 'text-(--text)' : 'text-(--text-2)'"
          ><span class="inline-flex items-center gap-2"
            >{{ t(l.label) }}<LiveBadge v-if="l.to === '/logbuch'" /></span
        ></RouterLink>
      </nav>
      <div class="flex items-center gap-1">
        <div role="group" :aria-label="t('nav.languageGroup')" class="flex items-center">
          <template v-for="(l, i) in ['de', 'en'] as const" :key="l">
            <span v-if="i" aria-hidden="true" class="font-mono text-[12px] text-(--muted)">·</span>
            <button
              type="button"
              :lang="l"
              :aria-current="locale === l ? 'true' : undefined"
              class="h-11 min-w-11 rounded-[8px] px-2 font-mono text-[12px] transition-colors lg:h-9 lg:min-w-0"
              :class="
                locale === l
                  ? 'font-medium text-(--text-2) underline underline-offset-4'
                  : 'text-(--muted) hover:bg-(--raised)'
              "
              @click="locale !== l && switchTo(l)"
            >
              {{ l.toUpperCase() }}
            </button>
          </template>
        </div>
        <button
          ref="menuButton"
          type="button"
          :aria-label="open ? t('nav.menuClose') : t('nav.menu')"
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
      :aria-label="t('nav.main')"
      class="mb-4 grid rounded-[14px] border border-(--line) bg-(--card) py-1 lg:hidden"
    >
      <RouterLink
        v-for="l in links"
        :key="l.to"
        :to="localePath(l.to)"
        :aria-current="isActive(l.to) ? 'page' : undefined"
        class="flex h-12 items-center px-4 text-[15px]"
        :class="isActive(l.to) ? 'text-(--text)' : 'text-(--text-2)'"
        ><span class="inline-flex items-center gap-2"
          >{{ t(l.label) }}<LiveBadge v-if="l.to === '/logbuch'" /></span
      ></RouterLink>
    </nav>
  </header>
</template>
