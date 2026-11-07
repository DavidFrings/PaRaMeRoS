// One-time import of legacy team/coach content as posts.
// Usage: API=https://parameros.net/api USERNAME=… PASSWORD=… node scripts/import-legacy-content.mjs [--dry-run]
import { readFile } from 'node:fs/promises'

const { USERNAME, PASSWORD } = process.env
const API = process.env.API?.replace(/\/$/, '')
const dry = process.argv.includes('--dry-run')
if (!API || !USERNAME || !PASSWORD) {
  console.error('Usage: API=<url> USERNAME=<user> PASSWORD=<pass> node scripts/import-legacy-content.mjs [--dry-run]')
  process.exit(2)
}

const dir = new URL('./legacy-content/', import.meta.url)
const content = JSON.parse(await readFile(new URL('content.json', dir), 'utf8'))

async function call(method, path, { headers, body } = {}) {
  const res = await fetch(API + path, { method, headers, body })
  if (!res.ok) {
    console.error(`${method} ${path} -> ${res.status}`)
    process.exit(1)
  }
  return res
}

// Same rule as seasonFromPost in frontend/src/lib/season.ts (season starts 1 August, Europe/Berlin).
const parts = (d) =>
  Object.fromEntries(
    new Intl.DateTimeFormat('en-US', { timeZone: 'Europe/Berlin', year: 'numeric', month: 'numeric' })
      .formatToParts(d).map((p) => [p.type, +p.value]),
  )
const yy = (n) => String(n % 100).padStart(2, '0')
function label(p) {
  const m = p.season?.match(/^(\d{4})\/(\d{2})$/)
  if (m) return `${m[1].slice(2)}/${m[2]}`
  const { year, month } = parts(new Date(`${p.created_at}Z`))
  const start = month >= 8 ? year : year - 1
  return `${yy(start)}/${yy(start + 1)}`
}

const auth = await call('POST', '/auth', {
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ username: USERNAME, password: PASSWORD }),
})
const token = auth.headers.get('Authorization')
if (!token) {
  console.error('POST /auth -> no Authorization header in response')
  process.exit(1)
}

const team = await (await call('GET', '/posts/team')).json()
const coaches = await (await call('GET', '/posts/coaches')).json()

async function create(name, item, season) {
  const form = new FormData()
  form.append('name', name)
  form.append('heading', item.heading)
  form.append('content', item.content)
  if (season) form.append('season', season)
  form.append('media_type', 'img')
  form.append('media_creator', 'Privat')
  // file last: the API reads media_type first for size limits
  const bytes = await readFile(new URL(item.image, dir))
  form.append('media', new Blob([bytes], { type: 'image/webp' }), item.image)
  await call('POST', '/post', { headers: { Authorization: token }, body: form })
}

const verb = dry ? 'would create' : 'create'
if (coaches.length) console.log('skip coaches (exists)')
else {
  console.log(`${verb} coaches`)
  if (!dry) await create('coaches', content.coaches)
}
for (const item of content.team) {
  const want = label({ season: item.season })
  if (team.some((p) => label(p) === want)) {
    console.log(`skip team ${item.season} (exists)`)
    continue
  }
  console.log(`${verb} team ${item.season}`)
  if (!dry) await create('team', item, item.season)
}
