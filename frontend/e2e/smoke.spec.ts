import AxeBuilder from '@axe-core/playwright'
import type { Page } from '@playwright/test'
import { test, expect, IDS, API, fakeJwt } from './fixtures/api'

const L = {
  de: {
    locale: 'de-DE',
    prefix: '',
    notFound: 'Seite nicht gefunden',
    hero: 'Das Robotik-Team der PRMRS',
    stripTitle: 'Neu im Logbuch',
    logbuch: 'Logbuch',
    team: 'Team',
    about: 'Über uns',
    forest: 'Wald-Projekt',
    login: 'Anmelden',
    username: 'Benutzername',
    password: 'Passwort',
    admin: 'Verwaltung',
    newPost: 'Neuer Beitrag',
    heading: 'Überschrift',
    content: 'Text (Markdown)',
    publish: 'Veröffentlichen',
    footerForest: 'Wald-Projekt (9. Klassen PRMRS)',
    countdown: /Testturnier in\s12\sTagen/,
    imprint: 'Impressum',
  },
  en: {
    locale: 'en-US',
    prefix: '/en',
    notFound: 'Page not found',
    hero: 'The PRMRS robotics team',
    stripTitle: 'New in the logbook',
    logbuch: 'Logbook',
    team: 'Team',
    about: 'About us',
    forest: 'Forest project',
    login: 'Log in',
    username: 'Username',
    password: 'Password',
    admin: 'Admin',
    newPost: 'New post',
    heading: 'Heading',
    content: 'Text (Markdown)',
    publish: 'Publish',
    footerForest: 'Forest project (Year 9 PRMRS)',
    countdown: /Testturnier in\s12\sdays/,
    imprint: 'Imprint',
  },
} as const

// A page is real if its h1 shows; the catch-all 404 would also satisfy a route check.
async function expectPage(page: Page, h1: string | RegExp, notFound: string) {
  await expect(page.getByRole('heading', { level: 1, name: h1 })).toBeVisible()
  await expect(page.getByText(notFound)).toHaveCount(0)
}

async function expectNoSeriousViolations(page: Page, exclude?: string) {
  const b = new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag22aa'])
  if (exclude) b.exclude(exclude)
  const { violations } = await b.analyze()
  const bad = violations.filter((v) => v.impact === 'serious' || v.impact === 'critical')
  expect(
    bad.map((v) => `${v.id} (${v.impact}): ${v.nodes.map((n) => n.target.join(' ')).join(' | ')}`),
  ).toEqual([])
}

async function adminLogin(page: Page) {
  await page
    .context()
    .addCookies([
      { name: 'auth_token', value: encodeURIComponent(fakeJwt()), url: 'http://localhost:4173' },
    ])
}

for (const [lang, c] of Object.entries(L)) {
  test.describe(`${lang}`, () => {
    test.use({ locale: c.locale })

    test('home_shows_hero_logbuch_and_strip', async ({ page }) => {
      await page.goto(`${c.prefix}/`)
      await expectPage(page, c.hero, c.notFound)
      await expect(page.getByRole('heading', { name: c.stripTitle })).toBeVisible()
      await expect(page.getByRole('link', { name: 'Erster Eintrag' })).toBeVisible()
      await expect(page.getByRole('img', { name: /2025\/26|25\/26/ }).first()).toBeVisible()
      await expect(page.getByText(/^Live$/)).toHaveCount(0)
    })

    test.describe('live', () => {
      test.use({ live: true })
      test('home_shows_live_badge_when_recent_entry', async ({ page }) => {
        await page.goto(`${c.prefix}/`)
        await expectPage(page, c.hero, c.notFound)
        await expect(
          page.getByRole('heading', { name: c.stripTitle }).getByText('Live'),
        ).toBeVisible()
      })
    })

    test('countdown_renders_days', async ({ page }) => {
      await page.goto(`${c.prefix}/`)
      await expectPage(page, c.hero, c.notFound)
      await expect(page.getByText(c.countdown).first()).toBeVisible()
    })

    test('logbuch_lists_entries_newest_first', async ({ page }) => {
      await page.goto(`${c.prefix}/logbuch`)
      await expectPage(page, c.logbuch, c.notFound)
      const titles = page.getByRole('main').getByRole('heading', { level: 2 })
      await expect(titles).toHaveText(
        ['Erster Eintrag', 'Zweiter Eintrag', 'Dritter Eintrag'].map((h) =>
          lang === 'en' ? `${h} EN` : h,
        ),
      )
    })

    test('team_has_season_sections', async ({ page }) => {
      await page.goto(`${c.prefix}/team`)
      await expectPage(page, c.team, c.notFound)
      await expect(page.locator('#season-2025-26')).toBeVisible()
      await expect(page.locator('#season-2024-25')).toBeVisible()
      await expect(page.getByRole('heading', { level: 2 })).toHaveText(
        ['Team Beta', 'Team Alpha'].map((h) => (lang === 'en' ? `${h} EN` : h)),
      )
    })

    test('about_shows_coaches_post', async ({ page }) => {
      await page.goto(`${c.prefix}/About`)
      await expectPage(page, c.about, c.notFound)
      await expect(
        page.getByText(lang === 'en' ? 'Content of Die Coaches' : 'Inhalt von Die Coaches'),
      ).toBeVisible()
    })

    test('post_page_shows_entry', async ({ page }) => {
      await page.goto(`${c.prefix}/post/${IDS.newest}`)
      await expectPage(page, lang === 'en' ? 'Erster Eintrag EN' : 'Erster Eintrag', c.notFound)
    })

    test('forest_project_only_in_footer', async ({ page }) => {
      await page.goto(`${c.prefix}/`)
      await expectPage(page, c.hero, c.notFound)
      await expect(page.getByRole('banner').locator('a[href*="forest-project"]')).toHaveCount(0)
      const link = page.getByRole('contentinfo').getByRole('link', { name: c.footerForest })
      await expect(link).toBeVisible()
      await link.click()
      await expectPage(page, c.forest, c.notFound)
      await expect(page).toHaveURL(`${c.prefix}/prmrs/forest-project`)
    })

    test('progress_logs_redirects_to_logbuch', async ({ page }) => {
      await page.goto(`${c.prefix}/progress-logs`)
      await expect(page).toHaveURL(`${c.prefix}/logbuch`)
      await expectPage(page, c.logbuch, c.notFound)
    })

    test('admin_requires_login_and_returns_after_login', async ({ page }) => {
      await page.goto(`${c.prefix}/admin`)
      await expect(page).toHaveURL(`${c.prefix}/Login?next=${c.prefix}/admin`)
      await expectPage(page, c.login, c.notFound)
      await page.getByLabel(c.username).fill('admin')
      await page.getByLabel(c.password, { exact: true }).fill('secret')
      await page.getByRole('button', { name: c.login, exact: true }).click()
      await expect(page).toHaveURL(`${c.prefix}/admin`)
      await expectPage(page, c.admin, c.notFound)
    })

    test('admin_create_logbuch_post_flow', async ({ page, created }) => {
      // The guard sends logged-out visitors to Login?next=/admin and back.
      await page.goto(`${c.prefix}/admin`)
      await expectPage(page, c.login, c.notFound)
      await page.getByLabel(c.username).fill('admin')
      await page.getByLabel(c.password, { exact: true }).fill('secret')
      await page.getByRole('button', { name: c.login, exact: true }).click()
      await expectPage(page, c.admin, c.notFound)
      await page
        .locator('[data-section="logbuch"]')
        .getByRole('button', { name: c.newPost })
        .click()
      await page.getByLabel(c.heading).first().fill('E2E Eintrag')
      await page.getByLabel(c.content).first().fill('Text vom Test')
      await page.getByRole('button', { name: c.publish }).click()
      await expect.poll(() => created.length).toBe(1)
      expect(created[0]).toMatch(/name="name"\r?\n\r?\nlogbuch/)
      expect(created[0]).toMatch(/name="heading"\r?\n\r?\nE2E Eintrag/)
    })

    test('no_third_party_requests', async ({ page, baseURL }) => {
      const origins = new Set<string>()
      page.on('request', (r) => {
        if (/^https?:/.test(r.url())) origins.add(new URL(r.url()).origin)
      })
      for (const p of [
        '/',
        '/logbuch',
        '/team',
        '/About',
        '/prmrs/forest-project',
        '/Imprint',
        '/Privacy-Policy',
        `/post/${IDS.newest}`,
      ]) {
        await page.goto(`${c.prefix}${p}`)
        await expect(page.getByRole('heading', { level: 1 })).toBeVisible()
        await page.waitForLoadState('networkidle')
      }
      expect([...origins].sort()).toEqual([new URL(baseURL!).origin, API].sort())
    })

    test.describe('axe_has_no_serious_violations', () => {
      for (const scheme of ['light', 'dark'] as const) {
        test.describe(scheme, () => {
          test.use({ colorScheme: scheme })
          for (const [name, path, h1] of [
            ['home', '/', c.hero],
            ['logbuch', '/logbuch', c.logbuch],
            ['team', '/team', c.team],
            ['about', '/About', c.about],
            ['login', '/Login', c.login],
          ] as const) {
            test(name, async ({ page }) => {
              await page.goto(`${c.prefix}${path}`)
              await expectPage(page, h1, c.notFound)
              await page.waitForLoadState('networkidle')
              await expectNoSeriousViolations(page)
            })
          }
          test('admin', async ({ page }) => {
            await adminLogin(page)
            await page.goto(`${c.prefix}/admin`)
            await expectPage(page, c.admin, c.notFound)
            await expectNoSeriousViolations(page)
            await page
              .locator('[data-section="logbuch"]')
              .getByRole('button', { name: c.newPost })
              .click()
            await expect(page.getByLabel(c.heading).first()).toBeVisible()
            await expectNoSeriousViolations(page)
          })
        })
      }

      test.describe('with cookie banner', () => {
        test.use({ consent: false })
        test('home', async ({ page }) => {
          await page.goto(`${c.prefix}/`)
          await expectPage(page, c.hero, c.notFound)
          await expect(page.locator('#cc-main .cm')).toBeVisible()
          await expectNoSeriousViolations(page)
        })
        test('banner buttons are 44 px tall on phones', async ({ page }, info) => {
          test.skip(info.project.name !== 'phone')
          await page.goto(`${c.prefix}/`)
          const btns = page.locator('#cc-main .cm__btn')
          await expect(btns.first()).toBeVisible()
          for (const b of await btns.all()) {
            expect((await b.boundingBox())!.height).toBeGreaterThanOrEqual(44)
          }
        })
      })
    })
  })
}

test.describe('phone', () => {
  test('no_horizontal_scroll_on_phone', async ({ page }, info) => {
    test.skip(info.project.name !== 'phone')
    await adminLogin(page)
    for (const lang of Object.values(L)) {
      for (const p of ['/', '/logbuch', '/team', '/About', '/Login', '/admin']) {
        await page.goto(`${lang.prefix}${p}`)
        await expect(page.getByRole('heading', { level: 1 })).toBeVisible()
        await page.waitForLoadState('networkidle')
        const [sw, iw] = await page.evaluate(() => [
          document.documentElement.scrollWidth,
          window.innerWidth,
        ])
        expect(sw, `${lang.prefix}${p}`).toBeLessThanOrEqual(iw)
      }
    }
  })
})
