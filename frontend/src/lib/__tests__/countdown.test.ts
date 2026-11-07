import { describe, expect, it } from 'vitest'
import { countdownState, type Tournament } from '../countdown'

const t = (starts_at: string, ends_at: string | null = null): Tournament => ({
  name: 'Cup',
  place: null,
  starts_at,
  ends_at,
  updated_at: '2026-01-01T00:00:00+01:00',
})
const start = '2027-02-14T09:00:00+01:00'
const at = (ms: number) => new Date(new Date(start).getTime() + ms)

describe('countdownState', () => {
  it('hidden without tournament', () => {
    expect(countdownState(null, new Date())).toEqual({ kind: 'hidden' })
  })
  it('days rounded down', () => {
    expect(countdownState(t(start), at(-12.5 * 86_400_000))).toEqual({ kind: 'days', days: 12 })
  })
  it('just over 24 h is 1 day, exactly 24 h is clock', () => {
    expect(countdownState(t(start), at(-86_400_001))).toEqual({ kind: 'days', days: 1 })
    expect(countdownState(t(start), at(-86_400_000))).toEqual({ kind: 'clock', hms: '24:00:00' })
  })
  it('clock within 24 h', () => {
    const ms = ((3 * 60 + 12) * 60 + 45) * 1000
    expect(countdownState(t(start), at(-ms))).toEqual({ kind: 'clock', hms: '03:12:45' })
  })
  it('running during, hidden after end', () => {
    const end = '2027-02-16T18:00:00+01:00'
    expect(countdownState(t(start, end), at(0))).toEqual({ kind: 'running' })
    expect(countdownState(t(start, end), new Date('2027-02-16T17:59:59+01:00'))).toEqual({
      kind: 'running',
    })
    expect(countdownState(t(start, end), new Date(end))).toEqual({ kind: 'hidden' })
  })
  it('countdownState_hides_after_start_day_without_end', () => {
    expect(countdownState(t(start), new Date('2027-02-15T00:00:01+01:00'))).toEqual({
      kind: 'hidden',
    })
    expect(countdownState(t(start), new Date('2027-02-14T23:59:00+01:00'))).toEqual({
      kind: 'running',
    })
  })
})
