# PaRaMeRoS website rework: design

Date: 2026-11-06 · Branch: `redesign` (from `main`; Copilot commit `3c42ad0` is intentionally dropped)

## 1. Intent

PaRaMeRoS.net is the site of the FIRST LEGO League (FLL) team of the Pater-Rupert-Mayer-Realschule (PRMRS). The current design looks dated and amateur, the code has bugs and at least one security hole, and admins cannot manage most content. The rework gives the site a modern, deliberate design, makes it safe and tested, and lets the coach and admins keep it current, especially on tournament day.

**Audience, in priority order**

1. Tournament judges and reviewers: little time, need to find team, history and live progress in under a minute.
2. Parents: follow what the team does, especially during a tournament.
3. Students: current and future team members.

**Devices:** design for PC first, but phones are equally important (judges and parents check on phones at the venue, often on mobile data).

## 2. Goals

- A new visual design (direction "B v3", section 5) that only this team could have, without the usual "AI-generated" look.
- No known bugs; no security findings rated medium or higher.
- All team content editable through the existing post API: Logbuch, Team seasons, Coaches, Wald-Projekt.
- A Logbuch that works as a live ticker on tournament day.
- A countdown to the next tournament, set by admins.
- German first, English second.
- Fast on phones on mobile data.
- Automated tests in CI.

## 3. Non-goals

- No full rewrite and no stack change: Vue 3 + Vite + TypeScript frontend, Rust Actix-web + Diesel + Postgres API stay.
- No machine translation; English post text is written by hand and optional.
- No per-user or per-section permissions: every logged-in user may manage every section (the teacher is also the team coach).
- No token blocklist / server-side logout.
- No analytics or tracking.

## 4. Anti-goals (failure even if it "works")

- Losing or corrupting existing posts, users or uploaded media.
- Any existing API request that works today stops working (route, method, request or response shape). Additions are allowed; rejecting malicious or invalid input that is accepted today is allowed.
- Loading fonts, scripts or other assets from third-party hosts at runtime (GDPR: visitor IPs must not go to Google etc.).
- A generic template look: gradients, glows, emoji icons, uppercase tracked labels, everything centered.

## 5. Design

Reference: `assets/2026-11-06-home-mockup-v3.html` (approved mockup, Tailwind via CDN only for the mockup) and the screenshots `assets/shot-pc-light.png`, `assets/shot-phone-dark.png`. The mockup is the visual source of truth for the home page; other pages follow the same system.

**Direction**
- Character: sporty, exact, proud.
- World: tournament day, with the competition table, the pit area and team photos.
- Moments, the two places that get rich colour and detail: the team photo in the hero, and the season strip of past teams.
- Signature: a small green monospace season tag like `Saison 26/27`, the same everywhere seasons appear.

**Tokens** (exact values from the mockup)
- Light: bg `#f7f4ee`, card `#fdfbf7`, raised `#efe9df`, text `#1f1b16`, text-2 `#5e554a`, muted `#7b7266`, hairline = text at 9%.
- Dark: bg `#161513`, card `#1c1a18`, raised `#262421`, text `#efe9de`, text-2 `#aaa398`, muted `#8a847a`, hairline = text at 8%.
- Orange `#f17a2e` with text `#1a120b` on it: primary action only.
- Green: links and season tags. Text green is `#22774a` on light and `#4cbb7f` on dark; the tag background is `#22774a` on light and `#2f8f5a` on dark.
- Fonts: Bricolage Grotesque (400/500/600) for everything, Geist Mono (400/500) for season tags, timestamps and meta. Both are self-hosted via `@fontsource` packages.
- Radius: 6 for tags, 10 for buttons, 12 for images, 14 to 16 for cards, 20 for the hero image on desktop. Spacing is on a 4 px grid.
- Icons: lucide line icons, 16 to 18 px, stroke 1.5, in the text colour.
- Theme follows `prefers-color-scheme`. There is no manual toggle.
- Motion: colour and opacity transitions only, 120 to 180 ms, `cubic-bezier(0.22,1,0.36,1)`. No looping animation. Respect `prefers-reduced-motion`.
- Implement with Tailwind CSS v4 (`@tailwindcss/vite`), tokens in `@theme` / CSS variables. The current `base.css`, `main.css` and the restyle of `cookieconsent.css` are replaced.

**Season rule.** An FLL season starts on 1 August, in `Europe/Berlin` time. So 6 Nov 2026 is season `26/27`, and 15 Mar 2027 is still `26/27`. One shared frontend function computes it, and it is used everywhere.

**Accessibility:** WCAG 2.2 AA. That means contrast, full keyboard use, visible focus, alt texts, a skip link, `lang` set per language, and touch targets of at least 44 px on phones.

## 6. Pages and routes

German lives at the root and English at `/en/…` with the same path. Only `/progress-logs` moves (to `/logbuch`); every other existing URL stays. File-based routing via `unplugin-vue-router` stays.

| Route | Content |
|---|---|
| `/` | Home, see 6.1 |
| `/About` (unchanged) | Team story (fixed text), coaches (API post), link to team seasons |
| `/team` | One block per season (API posts in section `team`), newest first, each with season tag, text and team photo |
| `/logbuch` (redirect from `/progress-logs`) | Logbuch feed, see 6.2 |
| `/post/{uuid}` | Single post |
| `/prmrs/forest-project` (unchanged, agreed) | The teacher's 9th-grade surveying project. Not part of the FLL team; the page says so. Linked only from the footer and the sitemap |
| `/Imprint`, `/Privacy-Policy` (unchanged) | Legal pages; the English versions state that the German text is binding |
| `/Login` (unchanged) | Login |
| `/admin` | See 6.3; replaces all `Create.vue` pages |
| 404 | Restyled, German and English |

Top navigation: Start, Über uns, Team, Logbuch, plus a DE · EN switch. On phones it collapses into a menu button. The Wald-Projekt is not in the navigation.

Footer: © line, Impressum, Datenschutz, Sitemap, and "Wald-Projekt (9. Klassen PRMRS)".

### 6.1 Home (order as in the mockup, with these changes)

1. **Hero:** season tag, then the countdown (6.4), then the headline "Das Robotik-Team der PRMRS", subhead, primary button "Team kennenlernen", secondary link "Zum Logbuch", and the newest team photo with a season caption. Desktop is a split with text left and photo right; phone stacks photo first, then text.
2. **"Neu im Logbuch"** replaces the mockup's "Auf einen Blick" shortcut bar. It shows the 3 newest Logbuch entries with timestamp (6.2 rules) and heading, links to each, a "Live" marker, and a link to the full Logbuch.
3. **"Wer wir sind":** short fixed text with a link to Über uns.
4. **"Unsere Jahrgänge" strip:** loaded from `team` posts, newest first, each tile showing the photo's small version, the season tag, and a link to that season on `/team`.
5. **Logbuch card:** the wide card from the mockup, linking to `/logbuch`. The mockup's narrow Wald-Projekt card is removed; let the Logbuch card take the full width or pair it with "Über uns". That choice is the builder's.

### 6.2 Logbuch

- Newest first. Each entry has a heading, Markdown text, an optional photo, its season tag and a timestamp.
- **Timestamps:** the API stores UTC in a `TIMESTAMP` column without a time zone. Always convert from UTC to `Europe/Berlin`.
  - Under 24 hours: relative, like "vor 12 Min.".
  - Older: absolute, like "14. Feb. 2026, 10:42".
  - Full date and time as a `title` / `<time datetime>`.
  - Edited entries add "bearbeitet 11:05".
- **Live marker:** shown next to "Logbuch" wherever it appears (nav, home section, page title) while the newest Logbuch entry is less than 2 hours old.
- **Auto-refresh:** while live, the Logbuch page and the home section refetch every 60 seconds without a page reload. They pause when the tab is hidden. They do not poll when not live.

### 6.3 Admin (`/admin`, logged-in only)

- **Tournament form** (6.4): set, change or clear.
- **"Neuer Beitrag" per section:** Logbuch, Team, Coaches, Wald-Projekt.
- **List of all posts** per section, with edit and delete (with confirmation).
- **Post editor:**
  - heading
  - Markdown text with a preview
  - optional English heading and text in a second tab
  - photo upload with photographer credit (`media_creator`)
  - for `team` posts, a season field (see 7.1)
- The editor is designed for PC first and must also work well on a phone: a large text area, a camera or photo picker (`accept="image/*"`), and a submit button that stays reachable.
- Logged-in users also see edit and delete actions on posts in the public pages.
- The **Coaches** section holds exactly one post: text about all coaches plus a group photo, shown on Über uns. The editor opens that post when it exists instead of creating a second one.

### 6.4 Tournament countdown

- **Shown in the hero:** "{name} in 12 Tagen". In the last 24 hours it shows "{name} in 03:12:45", ticking every second. From the start until the end it shows "Läuft gerade" with a link to the Logbuch.
- It is hidden after the end. When there is no end, it is hidden after 23:59 Berlin time on the start day. It is also hidden when no tournament is set.
- The place is shown as secondary text.

## 7. API changes (all additive)

Existing routes, methods and request/response fields stay valid. New fields are optional in requests. In responses, new fields are only added.

### 7.1 Database migrations (Diesel, each with a working `down.sql`)

1. `posts`: add nullable `heading_en VARCHAR`, `content_en VARCHAR`, `season VARCHAR`.
   - `season` holds a value like `2017/18` and is used by `team` posts. When it is null, the frontend derives the season from `created_at`.
   - *My call:* the import script creates historic seasons today, so their `created_at` would give the wrong season.
2. `posts.name`: rename `progress-logs` to `logbuch` (data migration).
3. New table `tournament` with one row at most, as an id-less singleton or with a constant id: `name VARCHAR NOT NULL`, `place VARCHAR`, `starts_at TIMESTAMPTZ NOT NULL`, `ends_at TIMESTAMPTZ NULL`, `updated_at TIMESTAMPTZ NOT NULL`.
4. **Content to Markdown:** convert existing HTML in `content` to Markdown.
   - Known tags: `<b>`/`<strong>` become `**…**`, `<i>`/`<em>` become `*…*`, `<br>` becomes a newline, `<a href>` becomes `[text](href)`. Strip any other tag and keep its inner text.
   - *My call:* the migration first copies `content` into a new column `content_html_backup`. `down.sql` restores from it. A later release drops the column once production looks right.
   - Before writing the migration, list which tags production content actually contains (the builder asks the owner for a DB dump or runs a read-only query). Report anything unexpected first.

### 7.2 Endpoints

| Change | Detail |
|---|---|
| `GET /posts/{name}` | Unchanged. `progress-logs` is treated as an alias of `logbuch`. Responses gain `heading_en`, `content_en`, `season`, `uuid` (if not already present) |
| `GET /post/{uuid}` | Gains the same optional fields |
| `POST /post` (multipart), `PUT /post/{uuid}` | Accept optional `heading_en`, `content_en`, `season`. `name` `progress-logs` is stored as `logbuch` |
| `PUT /post/{uuid}/media` (new, multipart) and `DELETE /post/{uuid}/media` | Replace or remove a post's media and credit while keeping its UUID. Removes the old files (all variants) and clears the cache. *My call:* deleting and recreating a post would change its UUID and break links |
| `GET /tournament` | Public. Returns `200` with the tournament or `204` when none is set |
| `PUT /tournament` | Auth, same `verify()` as posts. Body is name, place, starts_at, ends_at. `DELETE /tournament` clears it |
| `GET /sitemap.xml` | Public. Lists all fixed pages and every post page in DE and EN with `<xhtml:link rel="alternate" hreflang>`, `lastmod` from `updated_at`/`created_at`, and a `x-default` pointing to DE. The base URL comes from a new env var `SITE_URL` |

### 7.3 Images

- **Videos stay supported** (owner decision): MP4 or WebM, up to 100 MB, stored as uploaded with no conversion. They play with `<video controls preload="metadata">`. In lists and the season strip, video posts show a placeholder or still image instead of the video. The backfill skips videos.
- **Images accepted:** JPEG, PNG or WebP, up to 15 MB. Anything else gets `400` and over-size gets `413`, both checked before decoding. Also enforce a maximum pixel count against decompression bombs (*my call: 40 MP*).
- **Stored per upload** in `UPLOAD_DIR`:
  - `<name>.webp` (unchanged name and format, so existing `media_name` values keep working)
  - `<name>.jxl`
  - `<name>-800.webp`
  - `<name>-800.jxl`
- The frontend derives the other names from `media_name`.
- `<picture>` offers JXL first with WebP fallback. Lists and strips use the `-800` versions.
- JXL encoding uses `jpegxl-rs` (libjxl). Measure the AArch64 Docker build time before and after, and report it if it grows noticeably (it is currently about 5 min).
- Existing uploads get the missing variants from a background task in the API after startup (7.6).

### 7.4 Redis

- New `redis` service in both compose files, using a Chainguard image to match the others. The API connects through the env `REDIS_URL`.
- **Rate limits:**
  - Login: 5 failed attempts per IP per 15 min.
  - Register: 3 attempts per IP per hour.
  - Over the limit, return `429` with `Retry-After`.
- **Client IP:** the API reads it from `X-Real-IP`, which the owner's host nginx sets from Cloudflare's `CF-Connecting-IP`. When the header is missing, it uses the peer address. In `docker-compose.yml` the API port is published on `127.0.0.1` only, so the header cannot be spoofed from outside.
- **Cache:** `GET /posts/{name}` and `GET /post/{uuid}` responses are cached for 10 min. Any create, update or delete clears the affected keys.
- **Redis down:** the API keeps working. It skips caching and rate limiting and logs a warning.

### 7.6 Migrations and rollout (owner decision)

- The API embeds its migrations (`diesel_migrations`) and applies any pending ones at startup, before it serves requests. A failed migration stops the API. No manual `diesel migration run` is needed in production any more.
- After startup, a background task creates the missing `.jxl` and `-800` image variants for old uploads. It is idempotent and skips files that already exist.
- **Before the first deploy, the owner takes a full Postgres backup** (`pg_dump`) and copies the uploads volume. The Markdown and `logbuch` migrations change existing rows.
- The API and frontend images of this release are deployed together.

### 7.5 nginx (frontend container)

- Pass `/sitemap.xml` through to `http://api:8080/sitemap.xml`.
- SPA fallback for `/en/...`.

## 8. Content and languages

- `vue-i18n`, with German as the default and English as the second language.
- The UI, home, Über uns, Impressum, Datenschutz and 404 are translated. I draft the English and the owner reviews it in the PR.
- **Posts:** show `heading_en`/`content_en` on `/en/…` when present, otherwise the German text.
- **First visit to `/`:** go to `/en/` only when the browser language is not German and no choice is stored. A manual DE/EN switch is stored in `localStorage` as a functional, necessary preference. Never redirect away from a deep link.
- **German text polish** (owner reviews in the PR):
  - Fix English headings: "About Us" becomes "Über uns", "Log In" becomes "Anmelden", "Not Found" becomes "Seite nicht gefunden".
  - Replace "Coming soon..." with real or neutral text.
  - Use "Schülerinnen und Schüler".
  - No facts change.
- **Post text is Markdown,** rendered with `markdown-it` with `html: false`, so no raw HTML ever reaches the page. Links get `rel="noopener noreferrer"`. All `v-html` of API data is removed.
- **Privacy policy update:**
  - Redis rate limiting processes IP addresses for a short time.
  - Fonts are self-hosted.
  - The DE/EN choice is kept in `localStorage`.
  - Cookie consent stays "necessary only".

## 9. One-time import script

This is a script in the repo (*my call: a small Rust binary or a Node script in `scripts/`; builder decides*). The owner runs it once after deploy. It logs in with an admin account and creates posts through the API only:

- **One `coaches` post:** the current coach text and photo from `About.vue`.
- **One `team` post per historic season**, with photo, `season` value, and text from `About.vue`. Seasons: 17/18, 19/20, 20/21, 21/22, 22/23, 23/24, 24/25.

It is idempotent: it skips a season or the coaches post when one already exists. After import, those hardcoded texts and images are removed from the frontend.

## 10. Security

- **Full audit** of API, frontend, nginx and compose. Fix every finding rated medium or higher, and list the low ones for the owner. Known items to fix:
  - Stored XSS through `v-html` (fixed by Markdown, section 8).
  - CORS accepts an empty `Origin` together with `supports_credentials()`.
  - Upload size and type limits.
  - `entrypoint.sh` writes env values unquoted into `config.js`.
  - `index.html` loads `/config.js` twice.
- **JWT:** it stays in the `auth_token` cookie with the `Authorization` header, because the API contract stays. The audit decides whether hardening is possible additively, for example the API also setting an `HttpOnly` cookie it accepts.
- **Security headers** in nginx: CSP without `unsafe-inline` for scripts, HSTS, Referrer-Policy, Permissions-Policy.

## 11. Testing and CI

- **API integration tests** against real Postgres and Redis, for example as CI service containers:
  - auth: success, failure, rate limit
  - post CRUD
  - the `progress-logs` alias
  - tournament CRUD
  - sitemap
  - upload limits
  - Redis down
- **Frontend unit tests** with Vitest: the season function, timestamp formatting (DST edge cases), the countdown states, and the Markdown rendering safety.
- **Playwright smoke tests** for home, Logbuch, Team, Über uns, login and the admin post-create flow, at desktop and phone width, DE and EN.
- All of these are added to the existing `ci.yml` jobs.

## 12. Constraints

- Docker images stay Chainguard-based, which means no shell in runtime images.
- Production runs behind Cloudflare and the owner's own nginx. The owner adds `proxy_set_header X-Real-IP $http_cf_connecting_ip;`.
- Rust edition 2024, MSRV 1.87. pnpm frontend.
- Hooks in `.claude/settings.json` auto-format edited files. `schema.rs`, lock files and `.env` are protected from direct edits.

## 13. Left to the builder

- Exact component structure, store/composable layout, and the Tailwind class organisation.
- The Redis client crate and the key naming.
- How the integration tests start Postgres and Redis.
- The layout of the Logbuch card on home after the Wald-Projekt card is removed.
- The order of implementation. Suggested order: security fixes, then API additions and tests, then the frontend redesign, then i18n, then the import script.
