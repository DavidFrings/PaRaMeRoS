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
  const diff = now.getTime() - d.getTime()
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

/** Live = newest logbuch post is under 2 h old. */
export const isLive = (newestLogbuch: { created_at: string } | undefined, now: Date) =>
  !!newestLogbuch && now.getTime() - parseApiTime(newestLogbuch.created_at).getTime() < 7_200_000
