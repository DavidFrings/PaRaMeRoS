import type { Locale } from './time'

export type Post = {
  uuid: string
  name: string
  heading: string
  content: string
  heading_en: string | null
  content_en: string | null
  season: string | null
  media_type: 'img' | 'vid' | null
  media_name: string | null
  media_creator: string | null
  created_at: string
  updated_at: string | null
}

export function localizedPost(p: Post, locale: Locale) {
  const en = (v: string | null, fallback: string) => (locale === 'en' && v?.trim() ? v : fallback)
  return { heading: en(p.heading_en, p.heading), content: en(p.content_en, p.content) }
}
