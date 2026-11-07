import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
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
const ALL = ['logbuch', 'team', 'coaches', 'forest-project', 'tournament']
const perms = vi.hoisted(() => ({ value: [] as string[] }))
vi.mock('@/api/auth', () => ({
  login: vi.fn(),
  logout: vi.fn(),
  useAuth: () => ({
    loggedIn: ref(true),
    permissions: perms,
    check: vi.fn().mockResolvedValue(undefined),
    can: (p: string) => perms.value.includes(p === 'progress-logs' ? 'logbuch' : p),
  }),
}))

import {
  AuthExpiredError,
  ForbiddenError,
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

const urlOrig = { c: URL.createObjectURL, r: URL.revokeObjectURL }
afterEach(() => {
  Object.assign(URL, { createObjectURL: urlOrig.c, revokeObjectURL: urlOrig.r })
})

beforeEach(() => {
  vi.clearAllMocks()
  Object.assign(URL, { createObjectURL: () => 'blob:default', revokeObjectURL: () => {} })
  perms.value = [...ALL]
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

  it('editor_keeps_text_on_403', async () => {
    vi.mocked(createPost).mockRejectedValue(new ForbiddenError())
    const { w } = await mountAt(PostEditor, { section: 'logbuch' })
    await w.get('#pe-heading-de').setValue('Neue Überschrift')
    await w.get('#pe-content-de').setValue('Langer **Text**')
    await w.get('form').trigger('submit')
    await flushPromises()

    expect((w.get('#pe-heading-de').element as HTMLInputElement).value).toBe('Neue Überschrift')
    expect((w.get('#pe-content-de').element as HTMLTextAreaElement).value).toBe('Langer **Text**')
    expect(w.get('[role=alert]').text()).toBe('Dafür fehlt die Berechtigung')
    expect(w.text()).not.toContain('Sitzung abgelaufen')
  })

  it('editor_preview_shows_chosen_media', async () => {
    const create = vi.fn(() => 'blob:fake')
    const revoke = vi.fn()
    Object.assign(URL, { createObjectURL: create, revokeObjectURL: revoke })
    const { w } = await mountAt(PostEditor, { section: 'logbuch' })
    const input = w.get('input[type=file]').element as HTMLInputElement
    chooseFile(input, new File(['x'], 'a.jpg', { type: 'image/jpeg' }))
    await flushPromises()
    expect(w.get('#pe-preview + article img').attributes('src')).toMatch(/^blob:/)
    chooseFile(input, new File(['x'], 'v.mp4', { type: 'video/mp4' }))
    await flushPromises()
    expect(revoke).toHaveBeenCalledWith('blob:fake')
    expect(w.get('#pe-preview + article video').attributes('src')).toMatch(/^blob:/)
    Object.defineProperty(input, 'files', { value: [], configurable: true })
    input.dispatchEvent(new Event('change'))
    await flushPromises()
    expect(w.find('#pe-preview + article img, #pe-preview + article video').exists()).toBe(false)
    expect(revoke).toHaveBeenCalledTimes(2)
  })

  it('editor_preview_shows_existing_media_unless_removed', async () => {
    const p = post({ media_type: 'img', media_name: 'a.webp', media_creator: 'Anna' })
    const { w } = await mountAt(PostEditor, { section: 'coaches', post: p })
    expect(w.find('#pe-preview + article img').exists()).toBe(true)
    await w.get('input[type=checkbox]').setValue(true)
    expect(w.find('#pe-preview + article img').exists()).toBe(false)
  })

  it('editor_preview_skips_rejected_file', async () => {
    const create = vi.fn(() => 'blob:fake')
    Object.assign(URL, { createObjectURL: create, revokeObjectURL: vi.fn() })
    const { w } = await mountAt(PostEditor, { section: 'logbuch' })
    chooseFile(
      w.get('input[type=file]').element as HTMLInputElement,
      new File(['x'], 'a.gif', { type: 'image/gif' }),
    )
    await flushPromises()
    expect(create).not.toHaveBeenCalled()
    expect(w.find('#pe-preview + article img').exists()).toBe(false)
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

describe('permissions', () => {
  it('tournament_form_shows_403_message', async () => {
    vi.mocked(putTournament).mockRejectedValue(new ForbiddenError())
    const { w } = await mountAt(TournamentForm)
    await w.get('#tf-name').setValue('R')
    await w.get('#tf-start').setValue('2027-03-28T10:00')
    await w.get('form').trigger('submit')
    await flushPromises()
    expect(w.get('[role=alert]').text()).toBe('Dafür fehlt die Berechtigung')
    expect((w.get('#tf-name').element as HTMLInputElement).value).toBe('R')
  })

  it('post_actions_follow_section_permission', async () => {
    perms.value = ['logbuch']
    const team = await mountAt(PostAdminActions, { uuid: 'u1', title: 'T', section: 'team' })
    expect(team.w.find('a, button').exists()).toBe(false)
    const legacy = await mountAt(PostAdminActions, {
      uuid: 'u2',
      title: 'T',
      section: 'progress-logs',
    })
    expect(legacy.w.text()).toContain('Bearbeiten')
    expect(legacy.w.text()).toContain('Löschen')
  })

  it('delete_403_shows_message', async () => {
    vi.mocked(deletePost).mockRejectedValue(new ForbiddenError())
    const { w } = await mountAt(PostAdminActions, { uuid: 'u1', title: 'T', section: 'logbuch' })
    await w.get('button').trigger('click')
    await w.get('[role=group] .btn-primary').trigger('click')
    await flushPromises()
    expect(w.get('[role=alert]').text()).toBe('Dafür fehlt die Berechtigung')
  })

  it('admin_shows_only_permitted_areas', async () => {
    perms.value = ['logbuch', 'tournament']
    const { w } = await mountAt(AdminPage)
    expect(w.find('#tf-title').exists()).toBe(true)
    expect(w.get('[data-section=logbuch]').text()).toContain('Neuer Beitrag')
    for (const s of ['team', 'coaches', 'forest-project'])
      expect(w.find(`[data-section="${s}"]`).exists()).toBe(false)
    expect(getPosts).toHaveBeenCalledWith('logbuch')
    expect(getPosts).not.toHaveBeenCalledWith('forest-project')
    expect(w.text()).not.toContain('Keine Berechtigung')
  })

  it('admin_without_tournament_hides_form', async () => {
    perms.value = ['team']
    const { w } = await mountAt(AdminPage)
    expect(w.find('#tf-title').exists()).toBe(false)
    expect(w.find('[data-section=team]').exists()).toBe(true)
  })

  it('admin_blocks_editing_forbidden_post', async () => {
    perms.value = ['logbuch']
    vi.mocked(getPost).mockResolvedValue(post({ uuid: 'f1', name: 'forest-project' }))
    const { w } = await mountAt(AdminPage, {}, '/admin?edit=f1')
    expect(w.get('[role=alert]').text()).toBe('Dafür fehlt die Berechtigung')
    expect(w.find('#pe-heading-de').exists()).toBe(false)
  })

  it('admin_without_permissions_shows_note', async () => {
    perms.value = []
    const { w } = await mountAt(AdminPage)
    expect(w.text()).toContain('Keine Berechtigung')
    expect(w.find('#tf-title').exists()).toBe(false)
    expect(w.find('[data-section]').exists()).toBe(false)
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
    const { w } = await mountAt(PostAdminActions, { uuid: 'u1', title: 'T', section: 'logbuch' })
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
