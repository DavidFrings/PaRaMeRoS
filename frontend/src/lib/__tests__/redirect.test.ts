import { describe, expect, it } from 'vitest'
import { safeNext } from '../redirect'

describe('safeNext', () => {
  it('keeps same-site paths', () => {
    expect(safeNext('/admin?edit=u1')).toBe('/admin?edit=u1')
    expect(safeNext('/en/admin')).toBe('/en/admin')
  })
  it('rejects other hosts and junk', () => {
    for (const n of [
      '//evil.com',
      '/\\evil.com',
      'https://evil.com',
      'admin',
      '/\t/evil.com',
      '',
      undefined,
      ['/a'],
    ])
      expect(safeNext(n)).toBeNull()
  })
})
