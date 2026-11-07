import { computed, onScopeDispose, ref } from 'vue'
import { describeError, getPosts } from '@/api/client'
import { isLive, parseApiTime } from '@/lib/time'
import type { Post } from '@/lib/post'
import { useNow } from './useNow'

const POLL_MS = 60_000

// Module-level singleton: nav, home and the logbuch page share one list.
const posts = ref<Post[]>([])
let started = false
let consumers = 0
let timer: ReturnType<typeof setInterval> | undefined

/** Re-fetch the shared list, e.g. after an admin change. */
export async function refreshLogbuch() {
  try {
    const list = await getPosts('logbuch')
    posts.value = [...list].sort(
      (a, b) => parseApiTime(b.created_at).getTime() - parseApiTime(a.created_at).getTime(),
    )
  } catch (e) {
    // Keep the old posts; the next poll or visibility change retries.
    console.error('logbuch refresh failed', describeError(e))
  }
}

const visible = () => document.visibilityState === 'visible'
const liveNow = () => isLive(posts.value[0], new Date())
const onVisible = () => visible() && void refreshLogbuch()
const tick = () => liveNow() && visible() && void refreshLogbuch()

function stop() {
  clearInterval(timer)
  timer = undefined
  document.removeEventListener('visibilitychange', onVisible)
}

/** Test helper: drop shared state, timers and listeners. */
export function __resetLogbuchForTests() {
  stop()
  posts.value = []
  started = false
  consumers = 0
}

export function useLogbuch() {
  const now = useNow(POLL_MS)
  const live = computed(() => isLive(posts.value[0], now.value))

  if (!started) {
    started = true
    void refreshLogbuch()
  }
  if (consumers++ === 0) {
    timer = setInterval(tick, POLL_MS)
    document.addEventListener('visibilitychange', onVisible)
  }
  onScopeDispose(() => {
    if (--consumers === 0) stop()
  })

  return { posts, live, refresh: refreshLogbuch }
}
