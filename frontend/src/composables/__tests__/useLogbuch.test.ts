import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { defineComponent } from 'vue'
import { mount, flushPromises } from '@vue/test-utils'
import type { Post } from '@/lib/post'

vi.mock('@/api/client', () => ({ getPosts: vi.fn() }))
import { getPosts } from '@/api/client'
import { useLogbuch, __resetLogbuchForTests } from '../useLogbuch'

const post = (created_at: string): Post => ({
  uuid: 'u',
  name: 'n',
  heading: 'H',
  content: 'C',
  heading_en: null,
  content_en: null,
  season: null,
  media_type: null,
  media_name: null,
  media_creator: null,
  created_at,
  updated_at: null,
})

const Harness = defineComponent({
  setup: () => useLogbuch(),
  template: '<i />',
})

beforeEach(() => {
  vi.useFakeTimers()
  vi.setSystemTime(new Date('2026-03-01T12:00:00Z'))
  __resetLogbuchForTests()
  vi.mocked(getPosts).mockReset()
})
afterEach(() => vi.useRealTimers())

describe('useLogbuch', () => {
  it('useLogbuch_does_not_poll_when_not_live', async () => {
    vi.mocked(getPosts).mockResolvedValue([post('2026-03-01T08:00:00')])
    mount(Harness)
    await flushPromises()
    await vi.advanceTimersByTimeAsync(5 * 60_000)
    expect(getPosts).toHaveBeenCalledTimes(1)
  })
  it('useLogbuch_polls_every_60s_when_live', async () => {
    vi.mocked(getPosts).mockResolvedValue([post('2026-03-01T11:30:00')])
    mount(Harness)
    await flushPromises()
    await vi.advanceTimersByTimeAsync(3 * 60_000)
    expect(getPosts).toHaveBeenCalledTimes(4)
  })
  it('sorts newest first and keeps old posts on failed refresh', async () => {
    vi.mocked(getPosts).mockResolvedValueOnce([
      post('2026-03-01T08:00:00'),
      post('2026-03-01T09:00:00'),
    ])
    const w = mount(Harness)
    await flushPromises()
    const vm = w.vm as unknown as ReturnType<typeof useLogbuch>
    expect(vm.posts[0].created_at).toBe('2026-03-01T09:00:00')
    vi.spyOn(console, 'error').mockImplementation(() => {})
    vi.mocked(getPosts).mockRejectedValueOnce(new Error('x'))
    await vm.refresh()
    expect(vm.posts).toHaveLength(2)
  })
})
