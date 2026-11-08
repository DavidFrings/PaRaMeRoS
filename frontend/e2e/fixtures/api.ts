import { readFileSync } from 'node:fs'
import { test as base, expect, type Route } from '@playwright/test'

export const API = 'http://api.test'
const webp = readFileSync(new URL('../../src/assets/imgs/erfolge.webp', import.meta.url))

const HOUR = 3_600_000
const DAY = 24 * HOUR
// API times are naive UTC strings.
const ago = (ms: number) => new Date(Date.now() - ms).toISOString().slice(0, 19)

const b64 = (o: object) => Buffer.from(JSON.stringify(o)).toString('base64url')
export const fakeJwt = () =>
  `Bearer ${b64({ alg: 'none' })}.${b64({ sub: 'u', exp: Math.floor(Date.now() / 1000) + 3600 })}.sig`

const post = (uuid: string, name: string, heading: string, extra: object = {}) => ({
  uuid,
  name,
  heading,
  content: `Inhalt von ${heading}`,
  heading_en: `${heading} EN`,
  content_en: `Content of ${heading}`,
  season: null,
  media_type: null,
  media_name: null,
  media_creator: null,
  created_at: ago(3 * DAY),
  updated_at: null,
  ...extra,
})

export const IDS = {
  newest: '11111111-1111-4111-8111-111111111111',
  middle: '22222222-2222-4222-8222-222222222222',
  oldest: '33333333-3333-4333-8333-333333333333',
}

export const fixtures = (live: boolean) => ({
  logbuch: [
    // Deliberately unsorted: the page must sort newest first.
    post(IDS.oldest, 'logbuch', 'Dritter Eintrag', { created_at: ago(40 * DAY) }),
    post(IDS.newest, 'logbuch', 'Erster Eintrag', { created_at: ago(live ? HOUR : 2 * DAY) }),
    post(IDS.middle, 'logbuch', 'Zweiter Eintrag', { created_at: ago(10 * DAY) }),
  ],
  team: [
    post('44444444-4444-4444-8444-444444444444', 'team', 'Team Alpha', {
      season: '2024/25',
      media_type: 'img',
      media_name: 'alpha.webp',
      media_creator: 'Frau Muster',
    }),
    post('55555555-5555-4555-8555-555555555555', 'team', 'Team Beta', {
      season: '2025/26',
      media_type: 'img',
      media_name: 'beta.webp',
      media_creator: 'Frau Muster',
    }),
  ],
  coaches: [post('66666666-6666-4666-8666-666666666666', 'coaches', 'Die Coaches')],
  'forest-project': [
    post('77777777-7777-4777-8777-777777777777', 'forest-project', 'Wald Eintrag'),
  ],
})

export type Role = 'normal' | 'forest' | 'super'
// Same derivation as the API's `role_allows`.
const ROLE_PERMISSIONS: Record<Role, string[]> = {
  normal: ['logbuch', 'team', 'coaches', 'tournament'],
  forest: ['logbuch', 'team', 'coaches', 'forest-project', 'tournament'],
  super: ['logbuch', 'team', 'coaches', 'forest-project', 'tournament'],
}

type Opts = { live: boolean; tournament: boolean; consent: boolean; role: Role }

export const test = base.extend<Opts & { created: string[] }>({
  live: [false, { option: true }],
  tournament: [true, { option: true }],
  consent: [true, { option: true }],
  role: ['super', { option: true }],
  created: async ({}, use) => use([]),
  page: async ({ page, context, live, tournament, consent, role, created }, use) => {
    if (consent)
      await context.addCookies([
        {
          name: 'cookie-consent',
          value: encodeURIComponent(
            JSON.stringify({
              categories: ['necessary'],
              revision: 0,
              data: null,
              consentTimestamp: new Date().toISOString(),
              consentId: 'e2e',
              services: { necessary: [] },
              lastConsentTimestamp: new Date().toISOString(),
            }),
          ),
          url: 'http://localhost:4173',
        },
      ])
    else
      // vanilla-cookieconsent hides the banner from automated browsers (navigator.webdriver).
      await page.addInitScript(() =>
        Object.defineProperty(navigator, 'webdriver', { get: () => false }),
      )
    const data = fixtures(live)
    const all = Object.values(data).flat()
    const cors = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Headers': 'authorization, content-type',
      'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
      'Access-Control-Expose-Headers': 'Authorization',
    }
    const json = (route: Route, body: unknown, headers: Record<string, string> = {}) =>
      route.fulfill({ status: 200, json: body, headers: { ...cors, ...headers } })

    await page.route(`${API}/**`, async (route) => {
      const req = route.request()
      const { pathname } = new URL(req.url())
      const method = req.method()
      if (method === 'OPTIONS') return route.fulfill({ status: 204, headers: cors })
      let m: RegExpMatchArray | null
      if (method === 'GET' && (m = pathname.match(/^\/posts\/([\w-]+)$/)))
        return json(route, data[m[1] as keyof typeof data] ?? [])
      if (method === 'GET' && (m = pathname.match(/^\/post\/([\w-]+)$/))) {
        const p = all.find((x) => x.uuid === m![1])
        return p ? json(route, p) : route.fulfill({ status: 404, headers: cors })
      }
      if (method === 'GET' && pathname === '/tournament')
        return tournament
          ? json(route, {
              name: 'Testturnier',
              place: 'Aula',
              // 12 days and 3 h ahead, so the day count is 12 whatever the time of day.
              starts_at: new Date(Date.now() + 12 * DAY + 3 * HOUR).toISOString(),
              ends_at: null,
              updated_at: new Date().toISOString(),
            })
          : route.fulfill({ status: 204, headers: cors })
      if (method === 'POST' && pathname === '/auth')
        return json(route, {}, { Authorization: fakeJwt() })
      // Like the API's verify(): no header is a 400, not a 401.
      if (method !== 'GET' && !(method === 'POST' && pathname === '/auth'))
        if (!req.headers()['authorization'])
          return route.fulfill({
            status: 400,
            body: 'Missing Authorization header!',
            headers: cors,
          })
      if (method === 'GET' && pathname === '/auth/verify')
        return req.headers()['authorization']
          ? json(route, {
              role,
              permissions: ROLE_PERMISSIONS[role],
              can_set_created_at: role === 'super',
            })
          : route.fulfill({ status: 400, body: 'Missing Authorization header!', headers: cors })
      if (method === 'POST' && pathname === '/post') {
        created.push(req.postData() ?? '')
        return json(route, { id: IDS.newest })
      }
      return route.fulfill({ status: 404, headers: cors })
    })
    // Media is served from the preview origin; the browser falls back from the missing .jxl.
    await page.route('**/uploads/**', (route) =>
      route.request().url().endsWith('.jxl')
        ? route.fulfill({ status: 404 })
        : route.fulfill({ status: 200, body: webp, contentType: 'image/webp' }),
    )
    await use(page)
  },
})

export { expect }
