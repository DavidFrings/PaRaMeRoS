import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import type { Locale } from '@/lib/time'

export const LANG_KEY = 'pm-lang'

export function useLocale() {
  const route = useRoute()
  const router = useRouter()
  const locale = computed<Locale>(() => (route.meta.locale === 'en' ? 'en' : 'de'))

  const localePath = (path: string) =>
    locale.value === 'en' ? `/en${path === '/' ? '/' : path}` : path

  function switchTo(l: Locale) {
    try {
      localStorage.setItem(LANG_KEY, l)
    } catch {
      /* storage blocked: choice just isn't remembered */
    }
    const base = route.fullPath.replace(/^\/en(?=\/|\?|#|$)/, '') || '/'
    router.push(l === 'en' ? `/en${base === '/' ? '/' : base}` : base)
  }

  return { locale, localePath, switchTo }
}
