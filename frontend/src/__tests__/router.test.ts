import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createMemoryHistory } from 'vue-router'
import { createAppRouter } from '@/router'

const make = () => createAppRouter(createMemoryHistory())
const lang = (v: string) => vi.spyOn(navigator, 'language', 'get').mockReturnValue(v)

beforeEach(() => {
  localStorage.clear()
  vi.restoreAllMocks()
})

describe('router', () => {
  it('scroll_resets_on_navigation', () => {
    const sb = make().options.scrollBehavior!
    const to = make().resolve('/team')
    const from = make().resolve('/')
    expect(sb(to, from, null)).toEqual({ top: 0 })
    expect(sb(to, from, { left: 0, top: 300 })).toEqual({ left: 0, top: 300 })
    expect(sb(make().resolve('/team#season-2024-25'), from, null)).toEqual({
      el: '#season-2024-25',
    })
  })

  it('en_twin_routes_exist', () => {
    const r = make().resolve('/en/team')
    expect(r.meta.locale).toBe('en')
    expect(r.matched.length).toBeGreaterThan(0)
    expect(make().resolve('/en/nope/x').meta.locale).toBe('en')
    const home = make().resolve('/').name
    for (const p of ['/en', '/en/']) {
      const m = make().resolve(p).matched.at(-1)
      expect(m?.meta.locale).toBe('en')
      expect(m?.name).toBe(`en-${String(home)}`)
    }
    expect(make().resolve('/en/nope').name).not.toBe(`en-${String(home)}`)
    expect(make().resolve('/en/nope').meta.locale).toBe('en')
  })

  it('nested_pages_resolve_in_both_locales', () => {
    for (const path of ['/team', '/logbuch', '/post/abc', '/prmrs/forest-project']) {
      const de = make().resolve(path)
      expect(de.matched.at(-1)?.name).not.toBe('/[...404]')
      expect(de.meta.locale).toBeUndefined()
      expect(make().resolve(`/en${path}`).meta.locale).toBe('en')
    }
  })

  it('progress_logs_redirects', async () => {
    const r = make()
    await r.push('/progress-logs')
    expect(r.currentRoute.value.path).toBe('/logbuch')
    await r.push('/en/progress-logs')
    expect(r.currentRoute.value.path).toBe('/en/logbuch')
  })

  it('first_visit_root_redirects_only_for_non_german', async () => {
    lang('en-US')
    const en = make()
    await en.push('/')
    expect(en.currentRoute.value.path).toBe('/en/')

    lang('de-DE')
    const de = make()
    await de.push('/')
    expect(de.currentRoute.value.path).toBe('/')

    lang('en-US')
    localStorage.setItem('pm-lang', 'de')
    const stored = make()
    await stored.push('/')
    expect(stored.currentRoute.value.path).toBe('/')
  })

  it('deep_link_never_redirects_by_language', async () => {
    lang('en-US')
    const r = make()
    await r.push('/team')
    expect(r.currentRoute.value.path).toBe('/team')
  })
})
