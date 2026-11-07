/**
 * `next` target after login: only same-site paths. Rejects "//host" and "/\host"
 * (browsers read "\" as "/") and anything with whitespace or control characters.
 */
export const safeNext = (next: unknown): string | null =>
  typeof next === 'string' && /^\/(?![/\\])[^\s\\]*$/.test(next) ? next : null
