import { onScopeDispose, ref, type Ref } from 'vue'

export function useNow(intervalMs = 1000): Ref<Date> {
  const now = ref(new Date())
  const id = setInterval(() => (now.value = new Date()), intervalMs)
  onScopeDispose(() => clearInterval(id))
  return now
}
