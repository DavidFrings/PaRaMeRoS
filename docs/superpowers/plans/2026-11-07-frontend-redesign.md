# Frontend Redesign Implementation Plan (Plan B of 2)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rebuild the Vue frontend in the approved design "B v3" with Tailwind v4. It adds:
- Logbuch with live marker and auto-refresh
- tournament countdown
- an `/admin` page
- German/English
- safe Markdown
- content loaded from the API
- a one-time import script for today's hardcoded content

**Architecture:**
- Pure logic lives in small, unit-tested TypeScript modules under `src/lib/`: season, time, countdown, markdown and media.
- A typed API client lives in `src/api/`.
- Pages stay file-based (`unplugin-vue-router`). English is served under `/en/…` by duplicating the generated routes with a locale meta.
- Playwright smoke tests mock the API with `page.route`, so they need no backend.

**Tech Stack:** Vue 3.5, TypeScript 5.8, Vite 7, pnpm, Tailwind CSS v4 (`@tailwindcss/vite`), `vue-i18n` 11, `markdown-it` 14, `@fontsource-variable/bricolage-grotesque`, `@fontsource/geist-mono`, `lucide-vue-next`, Vitest 3, `@playwright/test`, `@axe-core/playwright`.

**Spec:** `docs/superpowers/specs/2026-11-06-website-rework-design.md` (sections 1–6, 8, 9, 10, 11). Visual source of truth: `docs/superpowers/specs/assets/2026-11-06-home-mockup-v3.html`.

**Depends on:** Plan A (`2026-11-07-api-rework.md`) merged. The HTTP contract is in Plan A, tasks 3, 5, 6 and 8.

## Global Constraints

**Design tokens** (copy exactly into `@theme` / CSS variables):
- Light: `--bg #f7f4ee`, `--card #fdfbf7`, `--raised #efe9df`, `--text #1f1b16`, `--text-2 #5e554a`, `--muted #7b7266`, `--line rgb(31 27 22 / .09)`, `--green #22774a`, `--green-tag #22774a`.
- Dark: `--bg #161513`, `--card #1c1a18`, `--raised #262421`, `--text #efe9de`, `--text-2 #aaa398`, `--muted #8a847a`, `--line rgb(239 233 222 / .08)`, `--green #4cbb7f`, `--green-tag #2f8f5a`.
- Both themes: `--orange #f17a2e`, `--on-orange #1a120b`.
- **Fonts:** Bricolage Grotesque (400/500/600) and Geist Mono (400/500), self-hosted via `@fontsource`. No request goes to fonts.googleapis.com or any other third-party host.
- **Theme:** follows `prefers-color-scheme` only. There is no toggle.
- **Radius:**
  - tag 6, button 10, image 12, card 14–16, hero image 20 on desktop
  - spacing on the 4 px grid
- **Icons:** lucide line icons, 16–18 px, stroke 1.5.
- **Motion:** only colour, opacity and transform transitions, 120–180 ms, `cubic-bezier(0.22,1,0.36,1)`. Disabled under `prefers-reduced-motion`. Nothing loops.
- **Season rule:** a season starts 1 August, `Europe/Berlin`. The label is `Saison YY/YY` in DE and `Season YY/YY` in EN; tags elsewhere show `YY/YY`.
- **Times:** API post times are UTC naive strings (`"2026-02-14T09:42:00"`), so append `Z` and render in `Europe/Berlin`. Tournament times are RFC 3339 with an offset.
- **Live:** the newest `logbuch` post is less than 2 h old (`7_200_000` ms).
- **Polling:** every 60 s, only while live and while `document.visibilityState === 'visible'`.
- **Markdown:** `markdown-it` with `{ html: false, linkify: true, breaks: true }`. Links get `rel="noopener noreferrer"`, and `target="_blank"` only for external links. No `v-html` with API data anywhere except the output of `renderMarkdown`.
- **Routes:**
  - New `/logbuch`, with a redirect from `/progress-logs`.
  - Unchanged: `/`, `/About`, `/team`, `/post/:id`, `/prmrs/forest-project`, `/Imprint`, `/Privacy-Policy`, `/Login`.
  - New `/admin`.
  - English is the same paths under `/en`.
- **Wald-Projekt** appears only in the footer and the sitemap, never in the nav or on home.
- **Accessibility:** WCAG 2.2 AA.
  - a skip link
  - visible focus (`outline 2px var(--orange)`, offset 2px)
  - `<html lang>` that follows the locale
  - touch targets ≥ 44 px under 1024 px width
- **Copy:**
  - Headlines, labels and buttons have no exclamation marks and no trailing periods.
  - Use "Schülerinnen und Schüler".
  - "Über uns", "Anmelden", "Seite nicht gefunden".
- **Breakpoints:** Tailwind defaults. The phone layout is designed at 375 px and the PC layout at 1280 px; both are first-class.

## Review Focus

1. **Visitor in another time zone, or around the DST switch.** A post at `2026-10-25T00:30:00` UTC must show "25. Okt. 2026, 02:30" and never shift by an extra hour. Test owner: Task 2, `formatTimestamp_handles_dst_end`.
2. **A post with an English heading but German text only.** Expected: on `/en` the English heading shows with the German text. Each field falls back on its own. Test owner: Task 2, `localizedPost_falls_back_per_field`.
3. **A Markdown link with a `javascript:` URL, or an image tag in content.** Expected: rendered as plain text, never executable. Test owner: Task 2, `renderMarkdown_blocks_javascript_urls` and `renderMarkdown_escapes_html`.
4. **The JWT expires while the admin is editing** (`JWT_EXPIRE` is in minutes). Expected: the save fails with a clear "Bitte erneut anmelden" message, the typed text is kept, and the cookie lifetime equals the token `exp`. Test owner: Task 3, `cookie_expiry_matches_token_exp`, and Task 9, `editor_keeps_text_on_401`.
5. **The tournament end is missing and it is past 23:59 Berlin on the start day.** Expected: the countdown is hidden. Test owner: Task 2, `countdownState_hides_after_start_day_without_end`.

---

### Task 1: Tooling, Tailwind, fonts, tokens, app shell

**Files:**
- Modify:
  - `frontend/package.json`
  - `frontend/vite.config.ts` (add `@tailwindcss/vite`; set the PWA manifest `theme_color`/`background_color` to `#161513`)
  - `frontend/index.html` (load `/config.js` once before `main.ts`; `<html lang="de">`)
  - `frontend/entrypoint.sh` (emit JSON-safe values: build `config.js` with `printf '%s'` of values passed through `jq -Rs`, or quote them, so a `"` in `DESC` cannot break out)
  - `frontend/src/main.ts`, `frontend/src/App.vue`
  - `.github/workflows/ci.yml` (frontend `test` job: `pnpm test`, then `pnpm exec playwright install --with-deps chromium && pnpm test:e2e`)
- Create:
  - `frontend/src/assets/css/app.css` (Tailwind import, `@theme`, tokens for `:root` and `@media (prefers-color-scheme: dark)`, base styles, focus ring)
  - `frontend/src/components/layout/{SiteHeader,SiteFooter,SkipLink}.vue`
  - `frontend/vitest.config.ts`
- Delete:
  - `frontend/src/assets/css/{base,main}.css`
  - `frontend/src/composables/Navigation.ts` (the letter-scramble animation)
  - `frontend/src/components/{NavbarComponent,FooterComponent}.vue`

**Interfaces:**
- Keep `vanilla-cookieconsent` ("necessary only", same texts), restyled with the tokens in `src/assets/css/cookieconsent.css`, so it no longer blocks page interaction. Its config moves from `main.ts` to `src/cookieconsent.ts`.
- Produces:
  - Scripts `test` (`vitest run`) and `test:e2e` (`playwright test`).
  - `SiteHeader` props: none. It reads the locale and live state from the composables built in Tasks 4 and 5. Until then, render the static links.
  - `SiteFooter` contains © `2023–{current year}` PaRaMeRoS · Pater-Rupert-Mayer-Realschule, Impressum, Datenschutz, Sitemap (`/sitemap.xml`) and "Wald-Projekt (9. Klassen PRMRS)".

- [ ] **Step 1:** Install the dependencies listed under Tech Stack, plus `tailwindcss@4`. Install with pnpm only, which updates `pnpm-lock.yaml`. The protect hook blocks hand edits of the lockfile.
- [ ] **Step 2:** Build the shell. Header and footer match the mockup at 375 px and 1280 px. Delete the old CSS and components.
- [ ] **Step 3:** Run `pnpm run type-check && pnpm exec eslint . --max-warnings 0 && pnpm run build`. Everything passes.
- [ ] **Step 4:** Check with `grep -r "fonts.googleapis\|fonts.gstatic" dist/`. Expected: no match.
- [ ] **Step 5:** Commit `feat(frontend): Tailwind v4 design tokens, self-hosted fonts, new app shell`.

### Task 2: Pure logic modules with unit tests

**Files:**
- Create: `frontend/src/lib/{season,time,countdown,markdown,media,post}.ts`
- Test: `frontend/src/lib/__tests__/*.test.ts`

**Interfaces:**
- Produces:
  - `seasonOf(date: Date): { start: number; label: string }`. The label is `"26/27"`, computed in `Europe/Berlin`.
  - `seasonFromPost(p: { season?: string | null; created_at: string }): string`. With `season` `"2017/18"` it returns `"17/18"`; otherwise it uses `seasonOf` on `created_at`.
  - `parseApiTime(naiveUtc: string): Date`
  - `formatTimestamp(d: Date, now: Date, locale: 'de' | 'en'): { text: string; iso: string; title: string }`:
    - Under 1 min: DE "gerade eben", EN "just now".
    - Under 60 min: DE "vor 12 Min.", EN "12 min ago".
    - Under 24 h: DE "vor 3 Std.", EN "3 h ago".
    - Otherwise DE "14. Feb. 2026, 10:42" and EN "14 Feb 2026, 10:42", in Berlin time.
  - `formatEdited(d: Date, locale): string`. Returns DE "bearbeitet 11:05" and EN "edited 11:05".
  - `isLive(newestLogbuch: { created_at: string } | undefined, now: Date): boolean`
  - `type Tournament = { name: string; place: string | null; starts_at: string; ends_at: string | null; updated_at: string }`
  - `type CountdownState` is the union below, and `countdownState(t: Tournament | null, now: Date): CountdownState` returns one of:
    - `{ kind: 'hidden' }`
    - `{ kind: 'days'; days: number }` (more than 24 h left; days rounded down, minimum 1)
    - `{ kind: 'clock'; hms: string }` (24 h or less, `"03:12:45"`)
    - `{ kind: 'running' }` (between start and end; without an end, until 23:59:59 Berlin on the start day)
  - `renderMarkdown(src: string): string`
  - `mediaSources(media_name: string, size: 'full' | 'small'): { jxl: string; webp: string }`. For `"x.webp"`: full returns `uploads/x.jxl` and `uploads/x.webp`, and small returns `uploads/x-800.jxl` and `uploads/x-800.webp`.
  - `type Post = { uuid: string; name: string; heading: string; content: string; heading_en: string | null; content_en: string | null; season: string | null; media_type: 'img' | 'vid' | null; media_name: string | null; media_creator: string | null; created_at: string; updated_at: string | null }`
  - `localizedPost(p: Post, locale): { heading: string; content: string }`

- [ ] **Step 1: Write the failing tests.**
  - `seasonOf`:
    - 2026-07-31T21:59:59Z → `"25/26"`
    - 2026-07-31T22:00:00Z → `"26/27"` (midnight Berlin in summer)
    - 2027-03-15 → `"26/27"`
    - 2099-09-01 → `"99/00"`
  - `seasonFromPost`: an explicit season wins.
  - `formatTimestamp`:
    - 12 min ago gives `"vor 12 Min."`
    - 30 h ago gives the absolute date
    - `formatTimestamp_handles_dst_end`: `2026-10-25T00:30:00` UTC → `"25. Okt. 2026, 02:30"`. Pin `now` to a week later.
  - `isLive`: 1h59m → true; 2h01m → false; undefined → false.
  - `countdownState`:
    - null gives hidden
    - 12.5 days gives `days 12`
    - 3h12m45s gives `clock "03:12:45"`
    - during gives running
    - after the end gives hidden
    - `countdownState_hides_after_start_day_without_end`: start 2027-02-14T09:00+01:00, now 2027-02-15T00:00:01+01:00 → hidden; now 2027-02-14T23:59+01:00 → running
  - `renderMarkdown`:
    - `**a**` gives `<strong>`
    - `renderMarkdown_escapes_html`: `<img src=x onerror=alert(1)>` comes out escaped
    - `renderMarkdown_blocks_javascript_urls`: `[x](javascript:alert(1))` produces no `href="javascript:`
    - external links get `rel="noopener noreferrer"`
  - `mediaSources`: the cases above.
  - `localizedPost_falls_back_per_field`
- [ ] **Step 2:** Run `pnpm test`. Expected: FAIL.
- [ ] **Step 3:** Implement with `Intl.DateTimeFormat(…, { timeZone: 'Europe/Berlin' })`. Do not add a date library.
- [ ] **Step 4:** Run `pnpm test`: PASS.
- [ ] **Step 5:** Commit `feat(frontend): season, time, countdown, markdown and media helpers`.

### Task 3: API client and auth

**Files:**
- Create: `frontend/src/api/{client,auth}.ts`, `frontend/src/api/__tests__/auth.test.ts`
- Modify: `frontend/env.d.ts`

**Interfaces:**
- Consumes: `Post` and `Tournament` from Task 2.
- Produces: everything in `client.ts` uses `window.__ENV__.API` as the base and the existing `axios`.
  - `getPosts(section: 'logbuch' | 'team' | 'coaches' | 'forest-project'): Promise<Post[]>`
  - `getPost(uuid): Promise<Post>`
  - `type PostInput = { name; heading; content; heading_en?; content_en?; season?; media_creator?; file?: File }`
  - `createPost(input: PostInput): Promise<string>`. It sends multipart and derives `media_type` from `file.type`: `image/*` gives `img` and `video/*` gives `vid`. It returns the uuid.
  - `updatePost(uuid, fields: Partial<Pick<PostInput, 'name' | 'heading' | 'content' | 'heading_en' | 'content_en' | 'season'>>): Promise<void>`
  - `replaceMedia(uuid, file: File, media_creator: string): Promise<string>`
  - `deleteMedia(uuid): Promise<void>`
  - `deletePost(uuid): Promise<void>`
  - `getTournament(): Promise<Tournament | null>` (`204` gives `null`)
  - `putTournament(t: Omit<Tournament, 'updated_at'>): Promise<Tournament>`
  - `deleteTournament(): Promise<void>`
  - `class AuthExpiredError extends Error`. Every authed call throws it on `401`.
  - `auth.ts`:
    - `login(username, password): Promise<void>` stores the `Authorization` header value in the cookie `auth_token`, with `expires` equal to the JWT `exp`, flags `Secure` and `SameSite=Strict`, and path `/`.
    - `logout(): void`
    - `useAuth(): { loggedIn: Ref<boolean>; check(): Promise<void> }`. `check` calls `/auth/verify`.

- [ ] **Step 1: Write the failing test.**
  - `cookie_expiry_matches_token_exp`: build a fake JWT with `exp = now + 180 s`, call the cookie-writing helper `tokenCookieExpiry(token): Date`, and expect the `exp` instant ±1 s.
  - `401_throws_AuthExpiredError`: mock axios.
- [ ] **Step 2:** Run `pnpm test`. Expected: FAIL.
- [ ] **Step 3:** Implement. Decode the JWT payload with `atob` on the middle segment; do not add a JWT library.
- [ ] **Step 4:** Run `pnpm test`: PASS.
- [ ] **Step 5:** Commit `feat(frontend): typed API client and auth with token-bound cookie`.

### Task 4: i18n and routing

**Files:**
- Create:
  - `frontend/src/i18n/{index,de,en}.ts`
  - `frontend/src/composables/useLocale.ts`
  - `frontend/src/router.ts` (move the router creation out of `main.ts`)
  - `frontend/src/pages/logbuch/index.vue` (placeholder, filled in Task 7)
- Delete: `frontend/src/pages/progress-logs/` (both files)

**Interfaces:**
- Produces:
  - `useLocale(): { locale: Ref<'de' | 'en'>; localePath(path: string): string; switchTo(l: 'de' | 'en'): void }`
    - `localePath('/team')` gives `/en/team` when the locale is EN.
    - `switchTo` stores `'pm-lang'` in `localStorage` (inside try/catch) and navigates to the same page in the other locale.
  - Router behaviour:
    - For every generated route it adds an `/en` twin, with `meta.locale = 'en'` and the same component.
    - It redirects `/progress-logs` → `/logbuch` and `/en/progress-logs` → `/en/logbuch`.
    - `afterEach` sets `document.documentElement.lang` and the i18n locale.
    - The first visit to exactly `/` with no stored choice and `navigator.language` not starting with `de` redirects to `/en/`. No other path ever redirects by language.
  - Message keys: group them per page (`nav.*`, `home.*`, `logbuch.*`, `team.*`, `about.*`, `admin.*`, `legal.*`, `notFound.*`, `time.*`, `countdown.*`). DE is the source; Claude drafts the EN text for owner review in the PR.

- [ ] **Step 1:** Write the failing unit tests with `router.resolve`:
  - `en_twin_routes_exist`: `/en/team` resolves.
  - `progress_logs_redirects`
  - `first_visit_root_redirects_only_for_non_german`: stub `navigator.language` to `en-US` and `de-DE`.
  - `deep_link_never_redirects_by_language`
- [ ] **Step 2:** Run `pnpm test`. Expected: FAIL.
- [ ] **Step 3:** Implement.
- [ ] **Step 4:** Run `pnpm test`: PASS. Run `pnpm run type-check`: clean.
- [ ] **Step 5:** Commit `feat(frontend): German/English routing and messages`.

### Task 5: Shared components and live/countdown composables

**Files:**
- Create:
  - `frontend/src/components/{SeasonTag,LiveBadge,PostMedia,PostTime,TournamentCountdown,LogbuchList,MarkdownContent}.vue`
  - `frontend/src/composables/{useLogbuch,useNow,useTournament}.ts`
- Delete: `frontend/src/components/{SectionComponent,CardComponent,CardsComponent}.vue` and `frontend/src/components/icons/*` (replaced by lucide)

**Interfaces:**
- Consumes: Task 2 helpers and Task 3 client.
- Produces:
  - `useNow(intervalMs = 1000): Ref<Date>`
  - `useLogbuch(): { posts: Ref<Post[]>; live: ComputedRef<boolean>; refresh(): Promise<void> }`
    - It is a singleton, so the nav, home and the page share it.
    - It polls 60 s while `live`, pauses while the tab is hidden, and refreshes once on `visibilitychange` to visible.
  - `useTournament(): { tournament: Ref<Tournament | null>; state: ComputedRef<CountdownState> }`
  - Components:
    - `<SeasonTag :label="'26/27'" :prefix="true|false" />`
    - `<LiveBadge />` renders only when live, as a green dot plus "Live". It has no animation.
    - `<PostMedia :post size="full|small" :alt />` renders `<picture>` with the jxl and webp sources for `img`, `<video controls preload="metadata">` for `vid`, and the media credit as a `figcaption`.
    - `<PostTime :created :updated />` renders `<time :datetime :title>` with "· bearbeitet hh:mm" when edited.
    - `<TournamentCountdown />`
    - `<LogbuchList :limit="3|undefined" />`
    - `<MarkdownContent :source />`, the only place that uses `v-html`.

- [ ] **Step 1:** Write the failing component tests with Vitest + `@vue/test-utils`:
  - `LiveBadge_hidden_when_not_live`
  - `PostMedia_renders_picture_with_jxl_first`
  - `PostMedia_renders_video_without_autoplay`
  - `TournamentCountdown_shows_days_text`: DE "Regionalwettbewerb Augsburg in 12 Tagen"
  - `useLogbuch_does_not_poll_when_not_live`: fake timers; the API is called once over 5 min
  - `useLogbuch_polls_every_60s_when_live`
- [ ] **Step 2:** Run `pnpm test`. Expected: FAIL.
- [ ] **Step 3:** Implement.
- [ ] **Step 4:** Run `pnpm test`: PASS.
- [ ] **Step 5:** Commit `feat(frontend): shared post, live and countdown components`.

### Task 6: Home page

**Files:**
- Modify: `frontend/src/pages/index.vue` (full rewrite; drop the intro animation, the scroll lock and the cookie-consent polling)

**Interfaces:**
- Consumes: Task 5 components, plus `getPosts('team')` and `getPosts('coaches')` (not used here).
- The order follows spec 6.1:
  1. Hero:
     - Desktop: a `lg:grid-cols-[5fr_7fr]` split, text left and photo right.
     - Phone: the photo comes first.
     - The photo is the newest `team` post's media, with the caption `Team {season}`.
     - The hero holds the season tag, then the countdown, the headline "Das Robotik-Team der PRMRS", the subhead "Schülerinnen und Schüler der Pater-Rupert-Mayer-Realschule, seit 2017 in der FIRST LEGO League", the primary button "Team kennenlernen" (→ `/team`) and the secondary link "Zum Logbuch".
  2. "Neu im Logbuch": `<LogbuchList :limit="3" />`, the LiveBadge, and "Alle Einträge" (→ `/logbuch`).
  3. "Wer wir sind": the text from the mockup, with "Mehr über uns" → `/About`.
  4. "Unsere Jahrgänge": a horizontal snap strip of `team` posts sorted by `seasonFromPost` descending, using `PostMedia size="small"`. Each tile links to `/team#season-YYYY-YY`. The count text reads "{n} Teams seit {oldest}". The scroller uses `scroll-px-5 md:scroll-px-8`.
  5. Logbuch card: full width, image left and text right on desktop.
- Empty states:
  - No team posts: hide the photo and let the text take the full width.
  - No Logbuch entries: "Noch keine Einträge".

- [ ] **Step 1:** Implement it to match `docs/superpowers/specs/assets/2026-11-06-home-mockup-v3.html`, with the changes listed above.
- [ ] **Step 2:** Run `pnpm dev` against a dev API that has one team post and two logbuch posts. Compare screenshots at 375 px and 1280 px, in light and dark, against the mockup screenshots in `docs/superpowers/specs/assets/`. Check:
  - no horizontal scroll at 375 px
  - the strip's first tile aligns with the page edge
  - buttons do not wrap
- [ ] **Step 3:** Commit `feat(frontend): new home page`.

### Task 7: Logbuch, Team, About, post, Wald-Projekt, legal, 404

**Files:**
- Modify:
  - `frontend/src/pages/logbuch/index.vue`
  - `frontend/src/pages/team/index.vue`
  - `frontend/src/pages/About.vue`
  - `frontend/src/pages/post/[id].vue`
  - `frontend/src/pages/prmrs/forest-project/index.vue`
  - `frontend/src/pages/{Imprint,Privacy-Policy,[...404]}.vue`
- Delete: `frontend/src/pages/{team,prmrs/forest-project}/Create.vue`

**Interfaces:**
- Consumes: Task 5 components and Task 2 `localizedPost`.
- Page content:
  - **Logbuch:**
    - The title "Logbuch" with the LiveBadge.
    - The full list, newest first.
    - Each entry has its time, heading, Markdown body, media and season tag.
  - **Team:** one `<section id="season-2024-25">` per team post, newest season first. Each section has the SeasonTag, the heading, the Markdown text and the team photo.
  - **About:**
    - Fixed team story text (i18n).
    - The single `coaches` post (heading, text, group photo). When it is missing, hide that block.
    - A link to `/team`. All hardcoded year sections and their images are removed.
  - **Post:** heading, time, season tag, media and Markdown.
  - **Wald-Projekt:**
    - Header note, DE: "Ein Schulprojekt der 9. Klassen der Pater-Rupert-Mayer-Realschule. Es gehört nicht zum FLL-Team PaRaMeRoS."
    - Then the `forest-project` posts.
  - **Imprint and Privacy:** the existing facts, restyled.
    - The EN versions carry the note "The German version is legally binding."
    - Privacy adds paragraphs on:
      - Redis rate limiting, which processes IP addresses for at most 1 h
      - self-hosted fonts, with no third-party requests
      - the `pm-lang` choice in `localStorage`
    - The owner reviews the legal wording.
  - **404:** "Seite nicht gefunden" / "Page not found", with a link home.

- [ ] **Step 1:** Implement the pages. Then run `grep -rn "v-html" frontend/src`. Expected: only `MarkdownContent.vue`.
- [ ] **Step 2:** Run `pnpm run type-check && pnpm exec eslint . --max-warnings 0`: clean.
- [ ] **Step 3:** Commit `feat(frontend): Logbuch, Team, About, post and legal pages`.

### Task 8: Login and admin

**Files:**
- Modify: `frontend/src/pages/Login.vue`
- Create:
  - `frontend/src/pages/admin/index.vue`
  - `frontend/src/components/admin/{PostEditor,PostList,TournamentForm}.vue`
- Delete: `frontend/src/components/CreatePostComponent.vue`, `frontend/src/pages/progress-logs/Create.vue` (if Task 4 left it)

**Interfaces:**
- Consumes: Task 3 client and auth.
- Login: username, password and "Anmelden". Show the error text for 401, and for 429 "Zu viele Versuche. Bitte in {n} Min. erneut versuchen" using `Retry-After`.
- `/admin`:
  - It is guarded and redirects to `/Login?next=/admin` when not logged in.
  - It has 3 areas: `TournamentForm`, the "Neuer Beitrag" buttons per section (Logbuch, Team, Coaches, Wald-Projekt), and `PostList` grouped by section with edit and delete. Delete asks for confirmation first.
  - **Coaches:** "Neuer Beitrag" opens the existing coaches post when there is one.
- `TournamentForm`:
  - Fields: name, place, start and end, as `datetime-local` interpreted in Europe/Berlin and converted to an offset string before `putTournament`.
  - Buttons: "Speichern" and "Entfernen".
- `PostEditor`:
  - Tabs "Deutsch" and "English (optional)". Each tab has a heading and a Markdown textarea with a live preview through `MarkdownContent`.
  - A `season` field for `team` posts, with the placeholder `2025/26` and the default from `seasonOf(now)`.
  - The file input uses `accept="image/jpeg,image/png,image/webp,video/mp4,video/webm"`, so on phones the camera is offered. It has a media-credit field, and is required when a file is chosen.
  - The client checks the size: images ≤ 15 MB and videos ≤ 100 MB, with a message before upload.
  - Layout:
    - PC: two columns, editor and preview.
    - Phone: one column with a sticky "Veröffentlichen" bar at the bottom.
  - Edit mode calls `updatePost`, plus `replaceMedia` or `deleteMedia` when the media changed.
  - On `AuthExpiredError` the editor keeps all fields, shows "Sitzung abgelaufen. Bitte erneut anmelden" and offers to log in through a modal without leaving the page.
- Logged-in users also see small "Bearbeiten" links on posts on the public pages, which go to `/admin?edit=<uuid>`.

- [ ] **Step 1:** Write the failing component tests:
  - `editor_keeps_text_on_401`: mock `createPost` to throw `AuthExpiredError`; the heading and content stay.
  - `editor_rejects_oversized_image_before_upload`
  - `tournament_form_sends_berlin_offset`: input `2027-03-28T10:00` gives `starts_at` `2027-03-28T10:00:00+02:00`
  - `coaches_new_opens_existing`
- [ ] **Step 2:** Run `pnpm test`. Expected: FAIL.
- [ ] **Step 3:** Implement.
- [ ] **Step 4:** Run `pnpm test`: PASS.
- [ ] **Step 5:** Commit `feat(frontend): admin page with post editor and tournament form`.

### Task 9: One-time import script

**Files:**
- Create:
  - `scripts/import-legacy-content.mjs` (Node 22, built-in `fetch`/`FormData`/`Blob`; no dependencies)
  - `scripts/legacy-content/` (move the year photos `2017-18.webp`…`2024-25.webp` and `coaches.webp` here from `frontend/src/assets/imgs/`, plus `content.json` with the texts copied verbatim from the current `About.vue`)
  - `scripts/README.md`

**Interfaces:**
- Usage: `API=https://parameros.net/api USERNAME=… PASSWORD=… node scripts/import-legacy-content.mjs [--dry-run]`.
- It logs in through `POST /auth`, reads `GET /posts/team` and `GET /posts/coaches`, and then:
  - Creates one `coaches` post if none exists.
  - For each season `2017/18, 2019/20, 2020/21, 2021/22, 2022/23, 2023/24, 2024/25`, creates a `team` post with the `season` field, unless a team post with that `season` already exists, or one without `season` whose `seasonFromPost` label matches (the label logic is duplicated in about 6 lines, with a comment pointing to `frontend/src/lib/season.ts`).
- `--dry-run` prints the plan only. It prints one line per created or skipped item and exits non-zero on any HTTP error.

- [ ] **Step 1:** Run it against the dev stack twice with `node scripts/import-legacy-content.mjs`. Expected: the first run creates 8 posts, the second run creates 0 and skips 8.
- [ ] **Step 2:** Check that `frontend/src/assets/imgs/` keeps only `logo.webp` plus the images still referenced, using `grep -rn "assets/imgs" frontend/src`.
- [ ] **Step 3:** Commit `feat: one-time import of legacy team and coach content`.

### Task 10: Playwright smoke and accessibility tests

**Files:**
- Create:
  - `frontend/playwright.config.ts` (projects `desktop` at 1280×800 and `phone` at 375×760; `webServer: pnpm build && pnpm preview --port 4173`)
  - `frontend/e2e/fixtures/api.ts` (`page.route` handlers serving JSON fixtures for posts per section, the tournament and auth)
  - `frontend/e2e/smoke.spec.ts`
  - `frontend/public/config.js` for preview, with `API: 'http://api.test'`. In the Docker image the entrypoint overwrites it.

**Interfaces:**
- Tests, each run in DE and EN where a page exists in both:
  - `home_shows_hero_logbuch_and_strip`
  - `home_shows_live_badge_when_recent_entry`
  - `countdown_renders_days`
  - `logbuch_lists_entries_newest_first`
  - `team_has_season_sections`
  - `about_shows_coaches_post`
  - `forest_project_only_in_footer` (absent from the header nav, present in the footer)
  - `progress_logs_redirects_to_logbuch`
  - `admin_create_logbuch_post_flow` (login, create, and the request body has `name=logbuch`)
  - `no_horizontal_scroll_on_phone`
  - `axe_has_no_serious_violations` on home, logbuch, team, about, login and admin, via `@axe-core/playwright` with tags `wcag2a`, `wcag2aa`, `wcag22aa`
  - `no_third_party_requests`: every request URL's origin is the preview origin or `http://api.test`

- [ ] **Step 1:** Write the tests.
- [ ] **Step 2:** Run `pnpm test:e2e`. Fix the failures in the owning page or component, not in the test.
- [ ] **Step 3:** Commit `test(frontend): Playwright smoke and accessibility tests`.

### Task 11: Frontend security pass and copy review list

**Files:**
- Modify: `frontend/nginx.conf`, plus whatever the findings touch
- Create: `docs/superpowers/plans/2026-11-07-frontend-review-notes.md`

- [ ] **Step 1:** In `nginx.conf`:
  - Add `Strict-Transport-Security "max-age=31536000; includeSubDomains" always`.
  - Extend the cache rule to `jxl|webm|woff2`.
  - Add `location /en/ { try_files $uri $uri/ /index.html; }`. The `/` rule already covers it; keep it only if a test shows it is needed.
  - Keep the CSP with `script-src 'self'`. If `config.js` or cookieconsent needs `unsafe-inline` for scripts, fix the code instead.
  - Set `connect-src 'self' <API origin>` through the entrypoint, because the API origin is runtime config.
- [ ] **Step 2:** Dispatch `security-reviewer` on `git diff main...HEAD -- frontend`. Fix the findings rated medium or higher, each with a test, the same way as in Plan A Task 10.
- [ ] **Step 3:** Write the review notes for the owner, then commit:
  - every German text that changed (old → new)
  - every English text, for review
  - the low-severity findings
- [ ] **Step 4:** Run the full check: `pnpm exec prettier --check src/ && pnpm exec eslint . --max-warnings 0 && pnpm run type-check && pnpm test && pnpm test:e2e && pnpm run build`. Everything must be green.
- [ ] **Step 5:** Commit `chore(frontend): security headers, review notes`.
