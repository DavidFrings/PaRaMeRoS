import { berlinYMD } from './time'

const yy = (n: number) => String(n % 100).padStart(2, '0')

/** A season starts 1 August (Europe/Berlin). */
export function seasonOf(date: Date) {
  const { year, month } = berlinYMD(date)
  const start = month >= 8 ? year : year - 1
  return { start, label: `${yy(start)}/${yy(start + 1)}` }
}

export function seasonFromPost(p: { season?: string | null; created_at: string }) {
  const m = p.season?.match(/^(\d{4})\/(\d{2})$/)
  if (m) return `${m[1].slice(2)}/${m[2]}`
  return seasonOf(new Date(`${p.created_at}Z`)).label
}
