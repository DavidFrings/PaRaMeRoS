import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createMemoryHistory } from 'vue-router'
import { createAppRouter } from '@/router'
import { i18n } from '@/i18n'
import type { Post } from '@/lib/post'

vi.mock('@/api/client', async (orig) => ({
  ...(await orig<typeof import('@/api/client')>()),
  getPosts: vi.fn(),
  getTournament: vi.fn(),
}))
import { getPosts, getTournament } from '@/api/client'
import { __resetLogbuchForTests } from '@/composables/useLogbuch'
import { __resetTournamentForTests } from '@/composables/useTournament'
import LiveBadge from '../LiveBadge.vue'
import PostMedia from '../PostMedia.vue'
import TournamentCountdown from '../TournamentCountdown.vue'

const NOW = new Date('2026-03-01T12:00:00Z')
const post = (o: Partial<Post> = {}): Post => ({
  uuid: 'u1',
  name: 'n',
  heading: 'H',
  content: 'C',
  heading_en: null,
  content_en: null,
  season: null,
  media_type: null,
  media_name: null,
  media_creator: null,
  created_at: '2026-03-01T11:00:00',
  updated_at: null,
  ...o,
})

async function mountAt(comp: object, path = '/team') {
  const router = createAppRouter(createMemoryHistory())
  await router.push(path)
  const w = mount(comp, { global: { plugins: [router, i18n] } })
  await flushPromises()
  return w
}

beforeEach(() => {
  vi.useFakeTimers()
  vi.setSystemTime(NOW)
  __resetLogbuchForTests()
  __resetTournamentForTests()
  vi.mocked(getPosts).mockReset()
  vi.mocked(getTournament).mockReset()
})
afterEach(() => vi.useRealTimers())

describe('LiveBadge', () => {
  it('LiveBadge_hidden_when_not_live', async () => {
    vi.mocked(getPosts).mockResolvedValue([post({ created_at: '2026-03-01T08:00:00' })])
    const w = await mountAt(LiveBadge)
    expect(w.text()).toBe('')
  })
  it('shows when live', async () => {
    vi.mocked(getPosts).mockResolvedValue([post()])
    const w = await mountAt(LiveBadge)
    expect(w.text()).toBe('Live')
  })
})

describe('PostMedia', () => {
  it('PostMedia_renders_picture_with_jxl_first', async () => {
    const w = await mountAt({
      components: { PostMedia },
      template: `<PostMedia :post="p" size="full" alt="Roboter" />`,
      data: () => ({ p: post({ media_type: 'img', media_name: 'a.jpg', media_creator: 'Max' }) }),
    })
    const s = w.findAll('picture source')
    expect(s.map((x) => x.attributes('type'))).toEqual(['image/jxl', 'image/webp'])
    expect(s[0].attributes('srcset')).toBe('/uploads/a.jxl')
    const img = w.get('img')
    expect(img.attributes('src')).toBe('/uploads/a.webp')
    expect(img.attributes('alt')).toBe('Roboter')
    expect(img.attributes('loading')).toBe('lazy')
    expect(w.get('figcaption').text()).toBe('Foto: Max')
  })
  it('PostMedia_renders_video_without_autoplay', async () => {
    const w = await mountAt({
      components: { PostMedia },
      template: `<PostMedia :post="p" size="full" alt="" />`,
      data: () => ({ p: post({ media_type: 'vid', media_name: 'a.mp4', media_creator: 'Max' }) }),
    })
    const v = w.get('video')
    expect(v.attributes('src')).toBe('/uploads/a.mp4')
    expect(v.attributes('controls')).toBeDefined()
    expect(v.attributes('preload')).toBe('metadata')
    for (const a of ['autoplay', 'loop', 'muted']) expect(v.attributes(a)).toBeUndefined()
    expect(w.get('figcaption').text()).toBe('Video: Max')
  })
  it('renders nothing without media', async () => {
    const w = await mountAt({
      components: { PostMedia },
      template: `<PostMedia :post="p" size="full" alt="" />`,
      data: () => ({ p: post() }),
    })
    expect(w.find('figure').exists()).toBe(false)
  })
})

describe('TournamentCountdown', () => {
  it('TournamentCountdown_shows_days_text', async () => {
    vi.mocked(getTournament).mockResolvedValue({
      name: 'Regionalwettbewerb Augsburg',
      place: 'Augsburg',
      starts_at: '2026-03-13T14:00:00+01:00',
      ends_at: null,
      updated_at: '2026-01-01T00:00:00',
    })
    const w = await mountAt(TournamentCountdown)
    // The unit is joined to the count with non-breaking spaces.
    expect(w.text()).toMatch(/Regionalwettbewerb Augsburg in\s12\sTagen/)
  })
})
