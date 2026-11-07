import { createRouter, type RouteRecordRaw, type RouterHistory, START_LOCATION } from 'vue-router'
import { routes } from 'vue-router/auto-routes'
import { i18n } from '@/i18n'
import { LANG_KEY } from '@/composables/useLocale'

const isCatchAll = (r: RouteRecordRaw) => r.path.includes('(.*)')

const twin = (r: RouteRecordRaw): RouteRecordRaw =>
  ({
    ...r,
    path: r.path === '/' ? '/en' : `/en${r.path}`,
    name: r.name ? `en-${String(r.name)}` : undefined,
    meta: { ...r.meta, locale: 'en' },
  }) as RouteRecordRaw

function buildRoutes(): RouteRecordRaw[] {
  const pages = routes.filter((r) => !isCatchAll(r))
  const catchAll = routes.filter(isCatchAll)
  return [
    { path: '/progress-logs', redirect: '/logbuch' },
    { path: '/en/progress-logs', redirect: '/en/logbuch' },
    ...pages,
    ...pages.map(twin),
    ...catchAll,
    ...catchAll.map(twin),
  ]
}

export function createAppRouter(history: RouterHistory) {
  const router = createRouter({ history, routes: buildRoutes() })

  router.beforeEach((to, from) => {
    if (from !== START_LOCATION || to.path !== '/') return
    try {
      if (localStorage.getItem(LANG_KEY) === null && !navigator.language.startsWith('de'))
        return '/en/'
    } catch {
      /* storage blocked: stay on German */
    }
  })

  router.afterEach((to) => {
    const l = to.meta.locale === 'en' ? 'en' : 'de'
    i18n.global.locale.value = l
    document.documentElement.lang = l
  })

  return router
}
