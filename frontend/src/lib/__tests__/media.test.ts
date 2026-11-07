import { describe, expect, it } from 'vitest'
import { mediaSources } from '../media'

describe('mediaSources', () => {
  it('full', () => {
    expect(mediaSources('x.webp', 'full')).toEqual({ jxl: 'uploads/x.jxl', webp: 'uploads/x.webp' })
  })
  it('small', () => {
    expect(mediaSources('x.webp', 'small')).toEqual({
      jxl: 'uploads/x-800.jxl',
      webp: 'uploads/x-800.webp',
    })
  })
  it('strips any extension', () => {
    expect(mediaSources('a.b.jpg', 'full').webp).toBe('uploads/a.b.webp')
  })
})
