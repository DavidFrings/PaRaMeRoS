import { computed, ref } from 'vue'
import { describeError, getTournament } from '@/api/client'
import { countdownState, type Tournament } from '@/lib/countdown'
import { useNow } from './useNow'

// Singleton, fetched once per app load and again after admin changes.
const tournament = ref<Tournament | null>(null)
let started = false

export const refreshTournament = () =>
  getTournament()
    .then((t) => (tournament.value = t))
    .catch((e) => console.error('tournament fetch failed', describeError(e)))

export function __resetTournamentForTests() {
  tournament.value = null
  started = false
}

export function useTournament() {
  const now = useNow(1000)
  if (!started) {
    started = true
    void refreshTournament()
  }
  const state = computed(() => countdownState(tournament.value, now.value))
  return { tournament, state }
}
