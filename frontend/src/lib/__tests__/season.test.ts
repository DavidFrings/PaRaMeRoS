import { describe, expect, it } from 'vitest'
import { seasonAnchor, seasonFromPost, seasonOf } from '../season'

describe('seasonOf', () => {
  it('switches at midnight Berlin on 1 August', () => {
    expect(seasonOf(new Date('2026-07-31T21:59:59Z')).label).toBe('25/26')
    expect(seasonOf(new Date('2026-07-31T22:00:00Z')).label).toBe('26/27')
  })
  it('handles spring and century wrap', () => {
    expect(seasonOf(new Date('2027-03-15T12:00:00Z'))).toEqual({ start: 2026, label: '26/27' })
    expect(seasonOf(new Date('2099-09-01T12:00:00Z')).label).toBe('99/00')
  })
})

describe('seasonFromPost', () => {
  it('explicit season wins', () => {
    expect(seasonFromPost({ season: '2017/18', created_at: '2026-02-14T09:42:00' })).toBe('17/18')
  })
  it('falls back to created_at (UTC naive)', () => {
    expect(seasonFromPost({ season: null, created_at: '2026-02-14T09:42:00' })).toBe('25/26')
    expect(seasonFromPost({ created_at: '2026-07-31T22:00:00' })).toBe('26/27')
  })
})

describe('seasonAnchor', () => {
  it('builds a stable id in the 20xx century', () => {
    expect(seasonAnchor('24/25')).toBe('season-2024-25')
    expect(seasonAnchor('17/18')).toBe('season-2017-18')
  })
})
