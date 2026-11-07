# Frontend redesign: review notes for the owner

Branch `redesign`. All German strings live in `frontend/src/i18n/de.ts`, English in `frontend/src/i18n/en.ts`.

## (a) German UI text that changed (old -> new)

Home (`/`)
- Old: "Wir sind PaRaMeRoS, das First Lego League Team der Pater Rupert Mayer Realschule. Als engagierte Schüler begeistern wir uns für Robotik und Technologie – mit Kreativität, Teamgeist und Leidenschaft." -> New: "Wir sind PaRaMeRoS, das FIRST LEGO League Team der Pater-Rupert-Mayer-Realschule. Als engagierte Schülerinnen und Schüler begeistern wir uns für Robotik und Technologie, mit Kreativität, Teamgeist und Leidenschaft."
- Old teaser "Erleben Sie den Fortschritt unseres Teams ..." -> New "Neu im Logbuch" plus the last three Logbuch entries; text "Was wir in dieser Saison bauen, testen und lernen: Einträge direkt aus der Werkstatt und vom Wettbewerb."
- Old Wald-Projekt card "PRMRS - Projekt der 9. Klassen / Coming soon..." -> removed from home (the project stays in the footer and sitemap).
- Old button "Mehr erfahren" -> "Mehr über uns", "Team kennenlernen", "Zum Logbuch", "Alle Einträge".
- New: "Das Robotik-Team der PRMRS", "Schülerinnen und Schüler der Pater-Rupert-Mayer-Realschule, seit 2017 in der FIRST LEGO League", "Wer wir sind", "Unsere Jahrgänge", "{n} Team(s) seit {oldest}", "Team {season}", "Logbuch".

Navigation and footer
- "Home" -> "Start"; "Über Uns" -> "Über uns"; "Entwicklung" (/progress-logs) -> "Logbuch" (/logbuch, old URL redirects); "Wald Projekt" -> "Wald-Projekt (9. Klassen PRMRS)"; "Cookies", "Impressum", "Datenschutz", "Sitemap" unchanged.
- New: "Menü", "Menü schließen", "Hauptnavigation", "Zum Inhalt springen", "Sprache", "Pater-Rupert-Mayer-Realschule".

About
- h1 "About Us" -> "Über uns".
- Intro rewritten as four paragraphs (facts kept): "Die Pater-Rupert-Mayer-Realschule nimmt seit 2017 mit vielen Teams erfolgreich an der FIRST LEGO League teil ..."; "Soft Skills" dropped; the closing "!" removed.
- "Unsere Jahrgänge" with seven hardcoded years -> removed; seasons now come from the API (team posts). Coaches text now comes from the API `coaches` post.
- New: "Alle Jahrgänge ansehen", "Die Coaches des Teams", "Zur Schulhomepage" (kept).

Team, Logbuch, Wald-Projekt, 404
- New: "Team", "Team der Saison {season}", "Logbuch", "Noch keine Einträge", "Live", "Saison", "Foto: {creator}", "Video: {creator}".
- Wald-Projekt: "Ein Schulprojekt der 9. Klassen der Pater-Rupert-Mayer-Realschule. Es gehört nicht zum FLL-Team PaRaMeRoS."
- 404: "Seite nicht gefunden", "Diese Seite gibt es nicht.", "Zur Startseite".

Login
- "Log In" -> "Anmelden". New hint under the button: "Es gilt unsere Datenschutzerklärung" (EN: "Our privacy policy applies"). Removed: the checkbox "Ich akzeptiere die Datenschutzrichtlinien dieser Website", the error "Du musst die Datenschutzrichtlinien akzeptieren!", and "Ein Fehler ist aufgetreten!".
- New: "Benutzername", "Passwort", "Passwort anzeigen/verbergen", "Benutzername oder Passwort falsch", "Zu viele Versuche. Bitte in {n} Min. erneut versuchen", "Anmeldung fehlgeschlagen. Bitte später erneut versuchen".

Admin (`/admin`, all new)
- Page: "Verwaltung", "Abmelden", "Beiträge", "Neuer Beitrag", "Noch keine Beiträge", "Beiträge konnten nicht geladen werden", "Bearbeiten", "Löschen", "Wirklich löschen?", "Abbrechen", "Schließen", "Löschen fehlgeschlagen".
- Session: "Sitzung abgelaufen. Bitte erneut anmelden", "Wieder angemeldet. Bitte erneut speichern".
- Editor: "Neuer Beitrag: {section}", "Beitrag bearbeiten: {section}", "Deutsch", "English (optional)", "Überschrift", "Text (Markdown)", "Saison", "Format JJJJ/JJ, zum Beispiel 2025/26", "Foto oder Video", "JPEG, PNG oder WebP bis 15 MB, MP4 oder WebM bis 100 MB", "Foto oder Video von", "Vorschau", "Veröffentlichen", "Speichern", plus validation and error messages (see `admin.editor` in `de.ts`).
- Turnier: "Turnier", "Name", "Ort (optional)", "Beginn", "Ende (optional)", "Zeiten in deutscher Zeit (Europe/Berlin)", "Entfernen", "Gespeichert", "Entfernt".

Legal (owner already approved)
- Imprint: wording unchanged, markup only.
- Privacy: Google Fonts section removed; "Schriftarten (selbst gehostet)", "Rate-Limiting mit Redis" and "Spracheinstellung (pm-lang)" added; "Stand: 17. August 2025" -> "Stand: 7. November 2026".

## (b) English text for review

Everything in `frontend/src/i18n/en.ts` is a draft by the implementer and needs a native read. Page-level texts to check:
- Home hero, "Who we are", Logbuch teaser, season strip (`home.*`).
- Logbuch, Team, About (`about.p1` to `p4`), Wald-Projekt note, 404 (`logbuch.*`, `team.*`, `about.*`, `forest.*`, `notFound.*`).
- Login and admin editor/tournament texts (`login.*`, `admin.*`).
- Legal: `frontend/src/components/legal/ImprintEn.vue` (starts "The German version is legally binding.") and `PrivacyEn.vue` (a full translation of the German text; a lawyer should review it).
- Post bodies are not translated by code: English post text comes from the `heading_en`/`content_en` fields editors fill in.

## (c) Owner decisions

Decided by the owner:
1. Login page has no privacy-consent checkbox any more: accepted. Instead a small line sits under the login button (also in the re-login modal): "Es gilt unsere Datenschutzerklärung" / "Our privacy policy applies", with the link going to the locale-aware privacy page. The footer "Datenschutz" link stays on every page.
2. The 2023/24 import text keeps its Markdown link "Zum Algorithmus" (GitHub wiki).
3. Contrast fixes kept: light `--muted` #7b7266 -> #6f665a, dark `--green-tag` #2f8f5a -> #2a8052 (WCAG AA).

Approved earlier: Task 7 text and legal changes.

Still to confirm:
4. `frontend/public/config.js` is tracked, and plain `pnpm dev` (outside docker) targets `http://api.test` instead of `localhost:8080`. Docker dev and prod overwrite the file.
5. The Datenschutz "Stand" date is 7. November 2026. Confirm or change.
6. About now says "seit 2017" (taken from the home page and coaches text). Confirm.
7. Markdown images are disabled in posts (`![x](url)` renders nothing), so post text can never load third-party images. Photos still come through the upload field.

## (d) Security review findings

Medium or higher: none.

Fixed in this task:
- Service worker precached `config.js` (re-graded Important): a changed `VITE_API` would not reach returning visitors. Now excluded from the precache (`dist/sw.js` no longer lists it) and nginx serves `/config.js` with `Cache-Control: no-cache`.
- Failed admin requests logged the whole AxiosError, which contains the `Bearer` token. Now only status and message are logged.
- A rejected (401) token stayed in the `auth_token` cookie. `check()` now removes it. Authed requests that get a 401 keep the cookie on purpose: the editor shows the re-login modal and a new login overwrites it.
- Static `target="_blank"` links in the privacy pages and cookie banner lacked `rel="noopener noreferrer"`. Added.
- `/uploads/` never got `immutable` because the static-asset regex won. Now `location ^~ /uploads/`.
- Also earlier in this task: all nginx locations send the full header set, HSTS added, CSP `connect-src` set from `VITE_API`, API CORS exposes `Authorization` and `Retry-After`.

Informational, no change:
- CSP has no `form-action` (no HTML injection route exists).
- HSTS has `includeSubDomains` but no `preload`: every subdomain must serve HTTPS. A duplicate header from host nginx or Cloudflare is harmless.
- Do not copy `.env.example`'s `http://localhost:3000` into the production `CORS_ORIGINS`.
- `build.sourcemap: true` ships `.map` files (already the case on main).
- Not checked: headers of the running container (no docker on the dev host). nginx syntax was checked with `nginx -t` in a container.

## (e) Deferred minor findings (plain English)

Task 1
- Old pages lost styling until later tasks rewrote them (now done).
- Nested `<main>` on the old home page until Task 6 (done).
- Footer lost the "Cookies" preferences link (restored in Task 4).
- Mobile menu lacked Escape/focus return (done in Task 4).
- Cookie banner button size under 44 px not checked (checked in Task 10 on the phone project only).
- `entrypoint.sh` relies on `tr` and `sed` in the Chainguard `latest-dev` image; unverified without docker.

Task 2
- A link starting with `//host` counts as internal (no new tab; `rel` is still set).
- Markdown images: decided in Task 11, now disabled.
- The javascript-link test only checks the missing href; autolink and uppercase cases are untested.
- The countdown throws on an invalid start date (no guard).
- Missing tests: summer-time start day, exact 2 h live boundary, English "Sept".

Task 3
- Any non-video file type counts as an image in the client (the API rejects bad types anyway).
- No tests for login, logout, check() or the multipart bodies.
- `loggedIn` is declared after the functions using it (works, but fragile).

Task 4
- First-visit language redirect drops query and hash.
- `localePath` has no double-prefix guard and contains a no-op ternary.
- Switching language on `/en?x=1` stays on `/en`.
- "Home" is not marked active at `/en`.
- Router tests lack later navigation to `/`, `localePath`/`switchTo`, and lang-attribute checks.
- English route copies share child records (no nested routes today).
- `twin()` rewrites only the first path pattern; deep paths like `/en/a/b` 404 and redirects with query/hash are untested.
- Header test does not assert the `lang` attribute or the no-op click on the active language.

Task 5
- Logbuch and tournament never retry after a failed first load (only a tab refocus or reload recovers).
- Logbuch data never resets after the last consumer leaves.
- Overlapping refreshes can arrive out of order.
- Missing tests: visibility pause/resume, ref-counting, live ageing, countdown states, edited time, empty list/limit, season prefix, English.
- Each consumer has its own timer (no shared ticker).
- LogbuchList hard-codes `<h3>`.

Task 6
- The hero photo arrives after the first render, so the layout shifts and `fetchpriority` has no effect.
- Strip tile link name is read twice (image alt plus label).
- The picture block is duplicated three times on the home page.
- "{n} Teams" counts posts, not distinct seasons.
- No unit tests for the home page (covered by e2e in Task 10).

Task 7
- A route test only checks the locale of the matched route.
- Post page requests `getPost('undefined')` when leaving the page.
- Post page says "not found" on network errors too.
- Logbuch and Wald-Projekt flash the empty state while loading.
- Duplicate Team section ids if two posts share a season; no Team empty state.
- About says "seit 2017" (see section c).
- Privacy table of contents targets lack scroll margin.

Task 8
- Inline delete confirm lacks Escape to cancel.
- Focus falls to the page body after the re-login modal closes.
- The jsdom-only `showModal` fallback ships in production.
- `Retry-After` was not CORS-exposed (fixed in this task).
- Logbuch cache is not refreshed after deleting from `/post/[id]`.

Task 9
- A network-level failure in the import script throws an uncaught error (still exits non-zero, no secret leak).
- Date helper has an unused NaN key case; `USERNAME` may be preset on Windows.

Task 10
- `public/config.js` tracked with `http://api.test` (see section c).
- E2E: cookie URL hard-coded to `localhost:4173`, `networkidle` waits, service-worker requests not covered by the third-party check, banner size only checked on the phone project.

## (f) Rollout

- Deploy API and frontend together. The new frontend needs the API CORS change (`Authorization` and `Retry-After` exposed) and the API change set from the earlier phases.
- After the deploy, run the import script once: `API=https://parameros.net/api USERNAME=… PASSWORD=… node scripts/import-legacy-content.mjs` (try `--dry-run` first; see `scripts/README.md`). It is safe to re-run.
- HSTS (`max-age=31536000; includeSubDomains`) tells browsers to use HTTPS for one year on the domain and all subdomains. Every subdomain must serve HTTPS before this ships.
- Set `VITE_API` to the real API origin; the CSP `connect-src` is derived from it at container start. A relative or unusual value falls back to same-origin only (a warning appears in the container log).
- Returning visitors get the new service worker on their next visit (`autoUpdate`).
