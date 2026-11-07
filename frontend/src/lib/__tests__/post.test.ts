import { describe, expect, it } from 'vitest'
import { localizedPost, type Post } from '../post'

const base: Post = {
  uuid: 'u',
  name: 'logbuch',
  heading: 'Titel',
  content: 'Inhalt',
  heading_en: 'Title',
  content_en: null,
  season: null,
  media_type: null,
  media_name: null,
  media_creator: null,
  created_at: '2026-02-14T09:42:00',
  updated_at: null,
}

describe('localizedPost', () => {
  it('localizedPost_falls_back_per_field', () => {
    expect(localizedPost(base, 'en')).toEqual({ heading: 'Title', content: 'Inhalt' })
    expect(localizedPost({ ...base, heading_en: '  ', content_en: 'Body' }, 'en')).toEqual({
      heading: 'Titel',
      content: 'Body',
    })
  })
  it('de always German', () => {
    expect(localizedPost({ ...base, content_en: 'Body' }, 'de')).toEqual({
      heading: 'Titel',
      content: 'Inhalt',
    })
  })
})
