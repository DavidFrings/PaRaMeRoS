import { createRouter, type RouteRecordRaw, type RouterHistory, START_LOCATION } from 'vue-router'
import { routes } from 'vue-router/auto-routes'
import { i18n } from '@/i18n'
import { LANG_KEY } from '@/composables/useLocale'

const isCatchAll = (r: RouteRecordRaw) => r.path.includes('(.*)')

// '(.*)' would also match '/en/' and beat the EN home; '(.+)' needs a segment.
// Child records need their own names too: a duplicate name replaces the DE record.
const twin = (r: RouteRecordRaw): RouteRecordRaw =>
  ({
    ...r,
    path: r.path === '/' ? '/en' : `/en${r.path.replace('(.*)', '(.+)')}`,
    name: r.name ? `en-${String(r.name)}` : undefined,
    meta: { ...r.meta, locale: 'en' },
    children: r.children?.map(twinChild),
  }) as RouteRecordRaw

const twinChild = (r: RouteRecordRaw): RouteRecordRaw =>
  ({
    ...r,
    name: r.name ? `en-${String(r.name)}` : undefined,
    meta: { ...r.meta, locale: 'en' },
    children: r.children?.map(twinChild),
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
  const router = createRouter({
    history,
    routes: buildRoutes(),
    // Team scrolls to its hash itself once its async sections load.
    scrollBehavior: (to, _from, saved) => saved ?? (to.hash ? { el: to.hash } : { top: 0 }),
  })

  router.beforeEach((to, from) => {
    if (from !== START_LOCATION || to.path !== '/') return
    try {
      if (localStorage.getItem(LANG_KEY) === null && !navigator.language.startsWith('de'))
        return { path: '/en/', query: to.query, hash: to.hash }
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
