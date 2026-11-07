import { describe, expect, it } from 'vitest'
import { formatEdited, formatTimestamp, isLive, parseApiTime } from '../time'

const now = new Date('2026-02-14T12:00:00Z')
const ago = (ms: number) => new Date(now.getTime() - ms)

describe('parseApiTime', () => {
  it('treats naive strings as UTC', () => {
    expect(parseApiTime('2026-02-14T09:42:00').toISOString()).toBe('2026-02-14T09:42:00.000Z')
  })
})

describe('formatTimestamp', () => {
  it('relative: just now, minutes, hours', () => {
    expect(formatTimestamp(ago(30_000), now, 'de').text).toBe('gerade eben')
    expect(formatTimestamp(ago(30_000), now, 'en').text).toBe('just now')
    expect(formatTimestamp(ago(12 * 60_000), now, 'de').text).toBe('vor 12 Min.')
    expect(formatTimestamp(ago(12 * 60_000), now, 'en').text).toBe('12 min ago')
    expect(formatTimestamp(ago(3 * 3_600_000), now, 'de').text).toBe('vor 3 Std.')
    expect(formatTimestamp(ago(3 * 3_600_000), now, 'en').text).toBe('3 h ago')
  })
  it('future dates count as just now', () => {
    expect(formatTimestamp(new Date(now.getTime() + 60_000), now, 'de').text).toBe('gerade eben')
  })
  it('absolute after 24 h, in Berlin time', () => {
    const d = new Date('2026-02-13T06:00:00Z')
    const r = formatTimestamp(d, now, 'de')
    expect(r.text).toBe('13. Feb. 2026, 07:00')
    expect(r.iso).toBe(d.toISOString())
    expect(r.title).toBe('13. Februar 2026, 07:00')
    expect(formatTimestamp(d, now, 'en').text).toBe('13 Feb 2026, 07:00')
  })
  it('formatTimestamp_handles_dst_end', () => {
    const d = new Date('2026-10-25T00:30:00Z')
    const later = new Date('2026-11-01T00:00:00Z')
    expect(formatTimestamp(d, later, 'de').text).toBe('25. Okt. 2026, 02:30')
  })
})

describe('formatEdited', () => {
  it('formats Berlin time', () => {
    const d = new Date('2026-02-14T10:05:00Z')
    expect(formatEdited(d, 'de')).toBe('bearbeitet 11:05')
    expect(formatEdited(d, 'en')).toBe('edited 11:05')
  })
})

describe('isLive', () => {
  it('is live under 2 h', () => {
    expect(isLive({ created_at: '2026-02-14T10:01:00' }, now)).toBe(true)
    expect(isLive({ created_at: '2026-02-14T09:59:00' }, now)).toBe(false)
    expect(isLive(undefined, now)).toBe(false)
  })
})
