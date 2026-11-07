import { describe, expect, it } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createMemoryHistory } from 'vue-router'
import { createAppRouter } from '@/router'
import { i18n } from '@/i18n'
import SiteHeader from '../SiteHeader.vue'

describe('SiteHeader language switch', () => {
  it('exposes DE and EN as separate controls with visible text as name', async () => {
    const router = createAppRouter(createMemoryHistory())
    await router.push('/team')
    const w = mount(SiteHeader, { global: { plugins: [router, i18n] } })
    await flushPromises()
    const group = w.get('[role="group"]')
    const [de, en] = group.findAll('button')
    expect(de.text()).toBe('DE')
    expect(en.text()).toBe('EN')
    expect(de.attributes('aria-label')).toBeUndefined()
    expect(de.attributes('aria-current')).toBe('true')
    expect(en.attributes('aria-current')).toBeUndefined()
    expect(group.get('[aria-hidden="true"]').text()).toBe('·')
  })
})
