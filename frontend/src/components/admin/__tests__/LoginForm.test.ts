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

  it('privacy_link_opens_new_tab_and_keeps_route', async () => {
    const router = createAppRouter(createMemoryHistory())
    await router.push('/admin')
    await router.isReady()
    const w = mount(LoginForm, { global: { plugins: [router, i18n] } })
    const a = w.find('a[href="/Privacy-Policy"]')
    expect(a.attributes('target')).toBe('_blank')
    expect(a.attributes('rel')).toContain('noopener')
    await a.trigger('click')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin')
  })
})
