import { beforeEach, describe, expect, it, vi } from 'vitest'
import axios from 'axios'
import { tokenCookieExpiry } from '../auth'
import {
  AuthExpiredError,
  ForbiddenError,
  deletePost,
  getTournament,
  putTournament,
  updatePost,
} from '../client'

vi.mock('axios')
const request = vi.mocked(axios.request)

const b64url = (o: object) =>
  btoa(JSON.stringify(o)).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '')
const jwt = (payload: object) => `${b64url({ alg: 'HS256' })}.${b64url(payload)}.sig`

beforeEach(() => {
  request.mockReset()
  window.__ENV__ = { API: 'http://api.test' }
  document.cookie = 'auth_token=Bearer%20abc; path=/'
})

describe('tokenCookieExpiry', () => {
  it('cookie_expiry_matches_token_exp', () => {
    const exp = Math.floor(Date.now() / 1000) + 180
    for (const t of [jwt({ exp }), `Bearer ${jwt({ exp })}`]) {
      expect(Math.abs(tokenCookieExpiry(t).getTime() - exp * 1000)).toBeLessThanOrEqual(1000)
    }
  })

  it('throws without numeric exp', () => {
    expect(() => tokenCookieExpiry(jwt({ sub: 'x' }))).toThrow()
    expect(() => tokenCookieExpiry('garbage')).toThrow()
  })
})

describe('authed calls', () => {
  it('401_throws_AuthExpiredError', async () => {
    request.mockRejectedValue({ response: { status: 401 } })
    await expect(deletePost('u')).rejects.toBeInstanceOf(AuthExpiredError)
    await expect(updatePost('u', { name: 'team' })).rejects.toBeInstanceOf(AuthExpiredError)
  })

  it('missing_cookie_throws_AuthExpiredError_without_request', async () => {
    document.cookie = 'auth_token=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT'
    await expect(updatePost('u', { name: 'team' })).rejects.toBeInstanceOf(AuthExpiredError)
    expect(request).not.toHaveBeenCalled()
  })

  it('403_throws_ForbiddenError', async () => {
    request.mockRejectedValue({ response: { status: 403 } })
    await expect(deletePost('u')).rejects.toBeInstanceOf(ForbiddenError)
    await expect(
      putTournament({ name: 'x', place: null, starts_at: 'y', ends_at: null }),
    ).rejects.toBeInstanceOf(ForbiddenError)
  })

  it('other errors propagate unchanged', async () => {
    const err = { response: { status: 500 } }
    request.mockRejectedValue(err)
    await expect(deletePost('u')).rejects.toBe(err)
  })

  it('sends cookie value verbatim as Authorization', async () => {
    request.mockResolvedValue({ status: 202, data: '' })
    await deletePost('u')
    expect(request.mock.calls[0][0]).toMatchObject({
      baseURL: 'http://api.test',
      url: '/post/u',
      method: 'DELETE',
      headers: { Authorization: 'Bearer abc' },
    })
  })

  it('getTournament returns null on 204', async () => {
    request.mockResolvedValue({ status: 204, data: '' })
    expect(await getTournament()).toBeNull()
  })
})

describe('review lows', () => {
  it('describeError_has_no_bearer_token', async () => {
    const { describeError } = await import('../client')
    const e = Object.assign(new Error('Request failed'), {
      response: { status: 500 },
      config: { headers: { Authorization: 'Bearer secret.jwt.value' } },
    })
    expect(describeError(e)).not.toContain('Bearer')
    expect(describeError(e)).toContain('500')
  })

  it('check_removes_cookie_on_401', async () => {
    const { useAuth } = await import('../auth')
    request.mockRejectedValue({ response: { status: 401 } })
    await useAuth().check()
    expect(document.cookie).not.toContain('auth_token')
  })
})

describe('permissions', () => {
  it('can_follows_verified_permissions', async () => {
    const { useAuth } = await import('../auth')
    request.mockResolvedValue({ status: 200, data: { admin: false, permissions: ['logbuch'] } })
    const auth = useAuth()
    await auth.check()
    expect(auth.permissions.value).toEqual(['logbuch'])
    expect(auth.can('logbuch')).toBe(true)
    // Old logbuch posts carry the legacy name.
    expect(auth.can('progress-logs')).toBe(true)
    expect(auth.can('team')).toBe(false)
    expect(auth.can('tournament')).toBe(false)
  })

  it('can_is_false_when_logged_out', async () => {
    const { useAuth, logout } = await import('../auth')
    request.mockResolvedValue({ status: 200, data: { admin: true, permissions: ['logbuch'] } })
    const auth = useAuth()
    await auth.check()
    expect(auth.can('logbuch')).toBe(true)
    logout()
    expect(auth.permissions.value).toEqual([])
    expect(auth.can('logbuch')).toBe(false)
  })

  it('login_loads_permissions', async () => {
    const { useAuth, login } = await import('../auth')
    const token = `Bearer ${jwt({ exp: Math.floor(Date.now() / 1000) + 60 })}`
    request.mockImplementation(async (c) =>
      c.url === '/auth'
        ? { status: 200, data: '', headers: { authorization: token } }
        : { status: 200, data: { admin: false, permissions: ['team'] } },
    )
    await login('a', 'b')
    expect(useAuth().can('team')).toBe(true)
  })
})
