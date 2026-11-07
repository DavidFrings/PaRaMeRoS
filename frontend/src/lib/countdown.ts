import { berlinYMD } from './time'

export type Tournament = {
  name: string
  place: string | null
  starts_at: string
  ends_at: string | null
  updated_at: string
}

export type CountdownState =
  | { kind: 'hidden' }
  | { kind: 'days'; days: number }
  | { kind: 'clock'; hms: string }
  | { kind: 'running' }

const DAY = 86_400_000
const hidden: CountdownState = { kind: 'hidden' }
const dayKey = (d: Date) => Object.values(berlinYMD(d)).join('-')

export function countdownState(t: Tournament | null, now: Date): CountdownState {
  if (!t) return hidden
  const start = new Date(t.starts_at)
  const left = start.getTime() - now.getTime()
  if (left > DAY) return { kind: 'days', days: Math.max(1, Math.floor(left / DAY)) }
  if (left > 0) {
    const s = Math.floor(left / 1000)
    const p = (n: number) => String(n).padStart(2, '0')
    return {
      kind: 'clock',
      hms: `${p(Math.floor(s / 3600))}:${p(Math.floor(s / 60) % 60)}:${p(s % 60)}`,
    }
  }
  // Without an end, "running" lasts until the end of the start day in Berlin.
  const over = t.ends_at ? now >= new Date(t.ends_at) : dayKey(now) !== dayKey(start)
  return over ? hidden : { kind: 'running' }
}
