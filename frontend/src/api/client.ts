import axios, { type AxiosRequestConfig } from 'axios'
import { useCookies } from 'vue3-cookies'
import type { Post } from '@/lib/post'
import type { Tournament } from '@/lib/countdown'

export const TOKEN_COOKIE = 'auth_token'

export class AuthExpiredError extends Error {
  constructor() {
    super('Anmeldung abgelaufen')
    this.name = 'AuthExpiredError'
  }
}

export const readToken = (): string | null => useCookies().cookies.get(TOKEN_COOKIE) || null

/** Safe to log: an AxiosError itself carries the Authorization header in `config`. */
export const describeError = (e: unknown): string => {
  const status = (e as { response?: { status?: number } })?.response?.status
  return `${status ?? ''} ${e instanceof Error ? e.message : String(e)}`.trim()
}

// API base is read per call so tests and late config.js can set it.
export async function request<T = unknown>(config: AxiosRequestConfig, authed = false) {
  const headers = { ...config.headers } as Record<string, string>
  if (authed) {
    const token = readToken()
    if (token) headers.Authorization = token
  }
  try {
    return await axios.request<T>({ ...config, baseURL: window.__ENV__.API, headers })
  } catch (e) {
    if (authed && (e as { response?: { status?: number } })?.response?.status === 401) {
      throw new AuthExpiredError()
    }
    throw e
  }
}

export type Section = 'logbuch' | 'team' | 'coaches' | 'forest-project'

export type PostInput = {
  name: string
  heading: string
  content: string
  heading_en?: string
  content_en?: string
  season?: string
  media_creator?: string
  file?: File
}

type PostFields = Partial<
  Pick<PostInput, 'name' | 'heading' | 'content' | 'heading_en' | 'content_en' | 'season'>
>

export const getPosts = async (section: Section) =>
  (await request<Post[]>({ url: `/posts/${section}`, method: 'GET' })).data

export const getPost = async (uuid: string) =>
  (await request<Post>({ url: `/post/${uuid}`, method: 'GET' })).data

const mediaType = (file: File) => (file.type.startsWith('video/') ? 'vid' : 'img')

function form(fields: Record<string, string | undefined>, file?: File) {
  const fd = new FormData()
  for (const [k, v] of Object.entries(fields)) if (v) fd.append(k, v)
  // The API reads the file part after the text fields.
  if (file) fd.append('media', file)
  return fd
}

export async function createPost({ file, ...p }: PostInput): Promise<string> {
  const fd = form(
    { ...p, media_type: file && mediaType(file), media_creator: file && p.media_creator },
    file,
  )
  const res = await request<{ id: string }>({ url: '/post', method: 'POST', data: fd }, true)
  return res.data.id
}

// "" clears heading_en/content_en/season on the API, so empty strings are sent as-is.
export async function updatePost(uuid: string, fields: PostFields): Promise<void> {
  await request({ url: `/post/${uuid}`, method: 'PUT', data: fields }, true)
}

export async function replaceMedia(uuid: string, file: File, media_creator: string) {
  const fd = form({ media_type: mediaType(file), media_creator }, file)
  const res = await request<{ media_name: string }>(
    { url: `/post/${uuid}/media`, method: 'PUT', data: fd },
    true,
  )
  return res.data.media_name
}

export async function deleteMedia(uuid: string): Promise<void> {
  await request({ url: `/post/${uuid}/media`, method: 'DELETE' }, true)
}

export async function deletePost(uuid: string): Promise<void> {
  await request({ url: `/post/${uuid}`, method: 'DELETE' }, true)
}

export async function getTournament(): Promise<Tournament | null> {
  const res = await request<Tournament>({ url: '/tournament', method: 'GET' })
  return res.status === 204 ? null : res.data
}

export async function putTournament(t: Omit<Tournament, 'updated_at'>): Promise<Tournament> {
  return (await request<Tournament>({ url: '/tournament', method: 'PUT', data: t }, true)).data
}

export async function deleteTournament(): Promise<void> {
  await request({ url: '/tournament', method: 'DELETE' }, true)
}
