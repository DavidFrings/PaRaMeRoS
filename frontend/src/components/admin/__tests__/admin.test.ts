import { beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { ref } from 'vue'
import { createMemoryHistory } from 'vue-router'
import { createAppRouter } from '@/router'
import { i18n } from '@/i18n'
import type { Post } from '@/lib/post'

vi.mock('@/api/client', async (orig) => ({
  ...(await orig<typeof import('@/api/client')>()),
  getPosts: vi.fn(),
  getPost: vi.fn(),
  createPost: vi.fn(),
  updatePost: vi.fn(),
  replaceMedia: vi.fn(),
  deleteMedia: vi.fn(),
  deletePost: vi.fn(),
  getTournament: vi.fn(),
  putTournament: vi.fn(),
  deleteTournament: vi.fn(),
}))
vi.mock('@/api/auth', () => ({
  login: vi.fn(),
  logout: vi.fn(),
  useAuth: () => ({ loggedIn: ref(true), check: vi.fn().mockResolvedValue(undefined) }),
}))

import {
  AuthExpiredError,
  createPost,
  deletePost,
  getPost,
  getPosts,
  getTournament,
  putTournament,
} from '@/api/client'
import PostEditor from '../PostEditor.vue'
import TournamentForm from '../TournamentForm.vue'
import PostAdminActions from '../PostAdminActions.vue'
import AdminPage from '@/pages/admin/index.vue'

const post = (o: Partial<Post> = {}): Post => ({
  uuid: 'u1',
  name: 'coaches',
  heading: 'Unsere Coaches',
  content: 'Text',
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

async function mountAt(comp: object, props: Record<string, unknown> = {}, path = '/admin') {
  const router = createAppRouter(createMemoryHistory())
  await router.push(path)
  const w = mount(comp, { props, global: { plugins: [router, i18n] }, attachTo: document.body })
  await flushPromises()
  return { w, router }
}

function chooseFile(input: HTMLInputElement, file: File) {
  Object.defineProperty(input, 'files', { value: [file], configurable: true })
  input.dispatchEvent(new Event('change'))
}

beforeEach(() => {
  vi.clearAllMocks()
  vi.mocked(getPosts).mockResolvedValue([])
  vi.mocked(getTournament).mockResolvedValue(null)
  document.body.innerHTML = ''
})

describe('PostEditor', () => {
  it('editor_keeps_text_on_401', async () => {
    vi.mocked(createPost).mockRejectedValue(new AuthExpiredError())
    const { w } = await mountAt(PostEditor, { section: 'logbuch' })
    await w.get('#pe-heading-de').setValue('Neue Überschrift')
    await w.get('#pe-content-de').setValue('Langer **Text**')
    await w.get('form').trigger('submit')
    await flushPromises()

    expect(createPost).toHaveBeenCalledOnce()
    expect((w.get('#pe-heading-de').element as HTMLInputElement).value).toBe('Neue Überschrift')
    expect((w.get('#pe-content-de').element as HTMLTextAreaElement).value).toBe('Langer **Text**')
    expect(w.get('[role=alert]').text()).toContain('Sitzung abgelaufen. Bitte erneut anmelden')
  })

  it('editor_rejects_oversized_image_before_upload', async () => {
    const { w } = await mountAt(PostEditor, { section: 'logbuch' })
    await w.get('#pe-heading-de').setValue('H')
    await w.get('#pe-content-de').setValue('C')
    const big = new File(['x'], 'big.jpg', { type: 'image/jpeg' })
    Object.defineProperty(big, 'size', { value: 15 * 1024 * 1024 + 1 })
    chooseFile(w.get('input[type=file]').element as HTMLInputElement, big)
    await flushPromises()
    await w.get('#pe-creator').setValue('Anna')
    await w.get('form').trigger('submit')
    await flushPromises()

    expect(createPost).not.toHaveBeenCalled()
    expect(w.get('[role=alert]').text()).toContain('15 MB')
  })

  it('editor_credit_readonly_without_new_file', async () => {
    const p = post({ media_type: 'img', media_name: 'a.webp', media_creator: 'Anna' })
    const { w } = await mountAt(PostEditor, { section: 'coaches', post: p })
    const credit = () => w.get('#pe-creator').element as HTMLInputElement
    expect(credit().value).toBe('Anna')
    expect(credit().readOnly).toBe(true)
    chooseFile(
      w.get('input[type=file]').element as HTMLInputElement,
      new File(['x'], 'b.jpg', { type: 'image/jpeg' }),
    )
    await flushPromises()
    expect(credit().readOnly).toBe(false)
  })

  it('editor_credit_resets_when_file_cleared', async () => {
    const p = post({ media_type: 'img', media_name: 'a.webp', media_creator: 'Anna' })
    const { w } = await mountAt(PostEditor, { section: 'coaches', post: p })
    const input = w.get('input[type=file]').element as HTMLInputElement
    chooseFile(input, new File(['x'], 'b.jpg', { type: 'image/jpeg' }))
    await flushPromises()
    await w.get('#pe-creator').setValue('Bernd')
    Object.defineProperty(input, 'files', { value: [], configurable: true })
    input.dispatchEvent(new Event('change'))
    await flushPromises()
    const credit = w.get('#pe-creator').element as HTMLInputElement
    expect(credit.readOnly).toBe(true)
    expect(credit.value).toBe('Anna')
  })
})

describe('TournamentForm', () => {
  it('tournament_form_sends_berlin_offset', async () => {
    vi.mocked(putTournament).mockImplementation(async (t) => ({ ...t, updated_at: 'x' }))
    const { w } = await mountAt(TournamentForm)
    await w.get('#tf-name').setValue('Regionalwettbewerb')
    await w.get('#tf-start').setValue('2027-03-28T10:00')
    await w.get('form').trigger('submit')
    await flushPromises()

    expect(putTournament).toHaveBeenCalledWith({
      name: 'Regionalwettbewerb',
      place: null,
      starts_at: '2027-03-28T10:00:00+02:00',
      ends_at: null,
    })
  })
})

describe('shared data refresh', () => {
  it('editor_save_refreshes_logbuch', async () => {
    vi.mocked(createPost).mockResolvedValue('n1')
    const { w } = await mountAt(PostEditor, { section: 'logbuch' })
    await w.get('#pe-heading-de').setValue('H')
    await w.get('#pe-content-de').setValue('C')
    await w.get('form').trigger('submit')
    await flushPromises()
    expect(getPosts).toHaveBeenCalledWith('logbuch')
  })

  it('post_delete_refreshes_logbuch', async () => {
    vi.mocked(deletePost).mockResolvedValue()
    const { w } = await mountAt(PostAdminActions, { uuid: 'u1', title: 'T' })
    await w.get('button').trigger('click')
    await w.get('[role=group] .btn-primary').trigger('click')
    await flushPromises()
    expect(deletePost).toHaveBeenCalledWith('u1')
    expect(getPosts).toHaveBeenCalledWith('logbuch')
  })

  it('tournament_save_refreshes_tournament', async () => {
    vi.mocked(putTournament).mockImplementation(async (t) => ({ ...t, updated_at: 'x' }))
    const { w } = await mountAt(TournamentForm)
    const before = vi.mocked(getTournament).mock.calls.length
    await w.get('#tf-name').setValue('R')
    await w.get('#tf-start').setValue('2027-03-28T10:00')
    await w.get('form').trigger('submit')
    await flushPromises()
    expect(getTournament).toHaveBeenCalledTimes(before + 1)
  })
})

describe('admin page', () => {
  it('coaches_new_opens_existing', async () => {
    const coach = post()
    vi.mocked(getPosts).mockImplementation(async (s) => (s === 'coaches' ? [coach] : []))
    vi.mocked(getPost).mockResolvedValue(coach)
    const { w, router } = await mountAt(AdminPage)
    const btn = w
      .get('[data-section=coaches]')
      .findAll('button')
      .find((b) => b.text() === 'Neuer Beitrag')
    await btn!.trigger('click')
    await flushPromises()

    expect(router.currentRoute.value.query.edit).toBe('u1')
    expect((w.get('#pe-heading-de').element as HTMLInputElement).value).toBe('Unsere Coaches')
  })
})
