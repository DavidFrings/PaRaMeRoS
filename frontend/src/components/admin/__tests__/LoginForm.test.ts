import { describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { createMemoryHistory } from 'vue-router'
import { createAppRouter } from '@/router'
import { i18n } from '@/i18n'

vi.mock('@/api/auth', () => ({ login: vi.fn() }))
import LoginForm from '../LoginForm.vue'

describe('LoginForm', () => {
  it('login_has_privacy_link', async () => {
    const router = createAppRouter(createMemoryHistory())
    await router.push('/Login')
    await router.isReady()
    const w = mount(LoginForm, { global: { plugins: [router, i18n] } })
    await flushPromises()
    const a = w.find('a[href="/Privacy-Policy"]')
    expect(a.exists()).toBe(true)
    expect(a.text()).toBe('Datenschutzerklärung')
    expect(w.text()).toContain('Es gilt unsere')
  })
})
