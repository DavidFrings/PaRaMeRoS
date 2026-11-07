import { ref } from 'vue'
import { useCookies } from 'vue3-cookies'
import { AuthExpiredError, TOKEN_COOKIE, readToken, request } from './client'

/** Expiry of the JWT's `exp` claim, so the cookie dies with the token. */
export function tokenCookieExpiry(token: string): Date {
  const payload = token.replace(/^Bearer\s+/i, '').split('.')[1]
  try {
    const b64 = payload.replace(/-/g, '+').replace(/_/g, '/')
    const { exp } = JSON.parse(atob(b64.padEnd(Math.ceil(b64.length / 4) * 4, '=')))
    if (typeof exp === 'number' && Number.isFinite(exp)) return new Date(exp * 1000)
  } catch {
    // fall through
  }
  throw new Error('Token has no valid exp claim')
}

const loggedIn = ref(!!readToken())
// From /auth/verify; the API lists every permission for admins.
const permissions = ref<string[]>([])

async function check() {
  if (!readToken()) {
    loggedIn.value = false
    permissions.value = []
    return
  }
  try {
    const res = await request<{ permissions?: string[] }>(
      { url: '/auth/verify', method: 'GET' },
      true,
    )
    permissions.value = res.data.permissions ?? []
    loggedIn.value = true
  } catch (e) {
    if (e instanceof AuthExpiredError) useCookies().cookies.remove(TOKEN_COOKIE, '/')
    loggedIn.value = false
    permissions.value = []
  }
}

// Old logbuch posts carry the legacy name "progress-logs".
const can = (perm: string) =>
  loggedIn.value && permissions.value.includes(perm === 'progress-logs' ? 'logbuch' : perm)

export async function login(username: string, password: string): Promise<void> {
  const res = await request({ url: '/auth', method: 'POST', data: { username, password } })
  const token = res.headers['authorization'] as string | undefined
  if (!token) throw new Error('Login response has no Authorization header')
  useCookies().cookies.set(TOKEN_COOKIE, token, tokenCookieExpiry(token), '/', '', true, 'Strict')
  loggedIn.value = true
  await check()
}

export function logout(): void {
  useCookies().cookies.remove(TOKEN_COOKIE, '/')
  loggedIn.value = false
  permissions.value = []
}

export function useAuth() {
  return { loggedIn, permissions, check, can }
}
