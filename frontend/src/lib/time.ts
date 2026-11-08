const TZ = 'Europe/Berlin'
const LOCALE = { de: 'de-DE', en: 'en-GB' } as const
export type Locale = keyof typeof LOCALE

const fmt = (locale: Locale, o: Intl.DateTimeFormatOptions) =>
  new Intl.DateTimeFormat(LOCALE[locale], { timeZone: TZ, ...o })

const hm = (d: Date, locale: Locale) =>
  fmt(locale, { hour: '2-digit', minute: '2-digit', hourCycle: 'h23' }).format(d)

/** Calendar year/month/day of `d` in Berlin. */
export function berlinYMD(d: Date) {
  const p = Object.fromEntries(
    fmt('en', { year: 'numeric', month: 'numeric', day: 'numeric' })
      .formatToParts(d)
      .map((x) => [x.type, Number(x.value)]),
  )
  return { year: p.year as number, month: p.month as number, day: p.day as number }
}

/** API post times are naive UTC strings. */
export const parseApiTime = (naiveUtc: string) => new Date(`${naiveUtc}Z`)

export function formatTimestamp(d: Date, now: Date, locale: Locale) {
  // A future date (clock skew or back-dating) reads as "just now".
  const diff = Math.max(0, now.getTime() - d.getTime())
  const de = locale === 'de'
  const abs = (month: 'short' | 'long') =>
    `${fmt(locale, { day: 'numeric', month, year: 'numeric' }).format(d)}, ${hm(d, locale)}`
  let text: string
  if (diff < 60_000) text = de ? 'gerade eben' : 'just now'
  else if (diff < 3_600_000) {
    const n = Math.floor(diff / 60_000)
    text = de ? `vor ${n} Min.` : `${n} min ago`
  } else if (diff < 86_400_000) {
    const n = Math.floor(diff / 3_600_000)
    text = de ? `vor ${n} Std.` : `${n} h ago`
  } else text = abs('short')
  return { text, iso: d.toISOString(), title: abs('long') }
}

export const formatEdited = (d: Date, locale: Locale) =>
  `${locale === 'de' ? 'bearbeitet' : 'edited'} ${hm(d, locale)}`

/** Clock skew the API allows for a post's `created_at` ahead of now. */
const SKEW_MS = 5 * 60_000

/** Live = newest logbuch post is under 2 h old and not further ahead than the clock skew. */
export function isLive(newestLogbuch: { created_at: string } | undefined, now: Date) {
  if (!newestLogbuch) return false
  const age = now.getTime() - parseApiTime(newestLogbuch.created_at).getTime()
  return age > -SKEW_MS && age < 7_200_000
}

const pad = (n: number) => String(n).padStart(2, '0')

/** Berlin wall time of `d` as a `datetime-local` value ("YYYY-MM-DDTHH:mm"). */
function berlinWall(d: Date) {
  const p = Object.fromEntries(
    fmt('en', {
      year: 'numeric',
      month: 'numeric',
      day: 'numeric',
      hour: 'numeric',
      minute: 'numeric',
      hourCycle: 'h23',
    })
      .formatToParts(d)
      .map((x) => [x.type, Number(x.value)]),
  )
  return { y: p.year, mo: p.month, d: p.day, h: p.hour, mi: p.minute }
}

/** Berlin UTC offset in minutes at instant `ms`. */
function berlinOffset(ms: number) {
  const w = berlinWall(new Date(ms))
  return (Date.UTC(w.y, w.mo - 1, w.d, w.h, w.mi) - Math.floor(ms / 60_000) * 60_000) / 60_000
}

/** `datetime-local` value read as Berlin wall time, to RFC 3339 with its DST-aware offset. */
export function berlinToRfc3339(local: string) {
  const [y, mo, d, h, mi] = local.split(/[-T:]/).map(Number)
  const asUtc = Date.UTC(y, mo - 1, d, h, mi)
  // Second pass corrects the guess when the first lands on the other side of a DST switch.
  const off = berlinOffset(asUtc - berlinOffset(asUtc) * 60_000)
  const sign = off < 0 ? '-' : '+'
  const abs = Math.abs(off)
  return `${y}-${pad(mo)}-${pad(d)}T${pad(h)}:${pad(mi)}:00${sign}${pad(Math.floor(abs / 60))}:${pad(abs % 60)}`
}

/** RFC 3339 instant to a Berlin `datetime-local` value. */
export function rfc3339ToBerlinLocal(iso: string) {
  const w = berlinWall(new Date(iso))
  return `${w.y}-${pad(w.mo)}-${pad(w.d)}T${pad(w.h)}:${pad(w.mi)}`
}
