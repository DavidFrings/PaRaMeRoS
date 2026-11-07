# API Rework Implementation Plan (Plan A of 2)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the Rust API safe, tested and ready for the redesign: new post fields, the Logbuch rename, tournament endpoints, the media pipeline (JPEG XL + small variants, videos), Redis rate limiting and caching, sitemap, and embedded migrations. Every request that works today keeps working.

**Architecture:**
- `main.rs` becomes a thin binary over a new `lib.rs`, so integration tests can build the same `App` with `actix_web::test`.
- Migrations are embedded and run at startup.
- Redis is optional at runtime: every Redis call fails open.
- New code follows the existing pattern of one handler per file under `src/api/<area>/`, `HttpError` helpers, and the `query!` macro.

**Tech Stack:** Rust 2024 (MSRV 1.87), actix-web 4, diesel 2 + diesel-async 0.6 (bb8), Postgres, `redis` crate, `image`, `webp`, `jpegxl-rs`, `diesel_migrations`.

**Spec:** `docs/superpowers/specs/2026-11-06-website-rework-design.md` (sections 4, 7, 10, 11, 12)

**Plan B** (`2026-11-07-frontend-redesign.md`) consumes the HTTP contract this plan produces. Finish Plan A first.

## Global Constraints

- **No existing request may break.** Routes, methods, request fields and response fields of `/register`, `/auth`, `/auth/verify`, `/posts/{name}`, `/post/{uuid}` (GET/POST/PUT/DELETE) stay valid. New request fields are optional, and response fields are only added.
- **Rejecting malicious or invalid input that works today is allowed.** Examples: oversized uploads, unknown sections, a CORS empty-`Origin`, and `404` instead of `500` for a missing post.
- Every migration has a working `down.sql`. Migration directories sort after `users`; name them `w01-…`, `w02-…` with no underscore, because diesel takes the version from the text before the first `_`.
- Section names: `logbuch`, `team`, `coaches`, `forest-project`. `progress-logs` is an alias that is stored and served as `logbuch`.
- Timestamps in `posts` stay `TIMESTAMP` holding UTC. New tables use `TIMESTAMPTZ`.
- **Images in:** JPEG, PNG or WebP, at most 15 MB (`15 * 1024 * 1024` bytes), at most 40 megapixels. **Videos in:** MP4 or WebM, at most 100 MB (`100 * 1024 * 1024` bytes). Over the size limit returns `413`, a wrong type returns `400`, and both are checked before decoding.
- **Stored per image:** `<base>.webp` (the existing name and format), `<base>.jxl`, `<base>-800.webp` and `<base>-800.jxl`. The small versions are 800 px wide, never upscaled. Videos are stored as uploaded.
- **Rate limits:** login allows 5 failed attempts per IP per 15 min (900 s). Register allows 3 attempts per IP per hour (3600 s). Over the limit returns `429` plus `Retry-After: <seconds>`.
- **Client IP:** `X-Real-IP` when present and parseable, else the peer address.
- **Cache:** `GET /posts/{name}` and `GET /post/{uuid}` are cached for 600 s. Any write clears them.
- **Redis down:** the API keeps working without cache and rate limit, and logs `warn!` once per failing call.
- New env vars: `REDIS_URL` (optional; without it Redis is off) and `SITE_URL` (required, e.g. `https://parameros.net`, no trailing slash).
- The runtime image stays Chainguard and has no shell.

## Review Focus

1. **Production data that doesn't match assumptions**, such as section names other than the four plus `progress-logs`, or HTML tags other than `b/strong/i/em/br/a`. Expected: the migration keeps that content readable (tags stripped, text kept) and never deletes rows. Test owner: Task 4, `html_to_md_strips_unknown_tags_keeps_text`.
2. **A post whose media file is missing on disk.** Today that makes `DELETE` fail with 500 forever. Expected: delete succeeds and only logs. Test owner: Task 2, `delete_post_with_missing_file_succeeds`.
3. **An upload stream that never ends or is huge**, with no Content-Length. Expected: reading stops at the limit and returns `413` without buffering the whole body. Test owner: Task 6, `oversized_image_returns_413`.
4. **A JWT for a deleted user, or a header without the `Bearer ` prefix.** Expected: `401`, never `500`. Test owner: Task 2, `verify_rejects_token_without_bearer_prefix` and `verify_rejects_deleted_user`.
5. **Tournament times around the DST change**, for example a start of `2027-03-28T10:00:00+02:00`. Expected: the API round-trips the exact instant as RFC 3339 with offset. Test owner: Task 5, `tournament_roundtrips_offset_instant`.

---

### Task 1: Library split, embedded migrations, test harness, CI job

**Files:**
- Create: `api/src/lib.rs`, `api/src/app.rs`, `api/src/migrate.rs`, `api/tests/common/mod.rs`, `api/tests/health.rs`
- Modify: `api/src/main.rs` (reduce to env loading, migrations, `HttpServer` with `app::configure`), `api/Cargo.toml`, `.github/workflows/ci.yml` (new job `api-test`)

**Interfaces:**
- Produces:
  - `pub struct Env` in `lib.rs`. Keep the existing fields and add `pub redis: Option<redis::aio::ConnectionManager>` (stays `None` until Task 7) and `pub site_url: String`. The crate-private fields become `pub`.
  - `pub fn configure(cfg: &mut actix_web::web::ServiceConfig)` in `app.rs`. It registers every service that `main.rs` registers today, and every later task adds its handlers here.
  - `pub fn cors(origins: &[String]) -> actix_cors::Cors` in `app.rs`.
  - `pub async fn run_migrations(database_url: &str) -> anyhow::Result<()>` in `migrate.rs`. It uses `diesel_migrations::embed_migrations!("migrations")` with a sync `diesel::PgConnection` inside `tokio::task::spawn_blocking`. This needs diesel feature `postgres`; `libpq` is already in the runtime image.
  - Test helper `pub async fn test_app() -> (impl Service<…>, Env)` in `tests/common/mod.rs`:
    - It reads `TEST_DATABASE_URL` and the optional `TEST_REDIS_URL`.
    - It runs migrations and truncates `posts`, `users` and, from Task 5, `tournament`.
    - It creates a user `admin` with password `admin-pass` and `admin=true`.
  - Test helpers `pub async fn login(app) -> String`, which returns `"Bearer …"`, and `pub fn site_url() -> &'static str`, which returns `"https://example.test"`.

- [ ] **Step 1:** Write `tests/health.rs::posts_list_empty`: `GET /posts/logbuch` → `200`, body `[]`.
- [ ] **Step 2:** Run `cd api && TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres cargo test --test health`. Expected: does not compile (`parameros_api` has no lib target).
- [ ] **Step 3:** Implement the lib split and the migrations.
  - `main.rs` calls `run_migrations` before building the pool, and exits non-zero on error.
  - Add dependencies: `diesel_migrations = "^2.2"`, `anyhow = "^1"`, `redis = { version = "^0.32", features = ["tokio-comp", "connection-manager"] }`, and the diesel feature `postgres`.
  - Add `[lib] name = "parameros_api"`.
  - The Dockerfile `lockfile` stage copies `./.build/src`, so also copy `src/lib.rs` into `.build/src` if that stub mirrors `src`. Check `api/.build/src` and keep it compiling.
- [ ] **Step 4:** Run the test again. Expected: PASS. Then run `cargo clippy --all-targets --all-features -- -D warnings`: clean.
- [ ] **Step 5:** Add the CI job `api-test` (needs `api-format`):
  - service containers `postgres:17` with `POSTGRES_PASSWORD=postgres` and `redis:7`
  - env `TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres` and `TEST_REDIS_URL=redis://localhost:6379`
  - run `cargo test --locked -- --test-threads=1`. Tests share one database, so they run serially.
- [ ] **Step 6:** Commit `refactor(api): split lib, embed migrations, add integration test harness`.

### Task 2: Security and correctness fixes in existing handlers

**Files:**
- Modify: `api/src/utils.rs`, `api/src/app.rs`, `api/src/api/post/{get_post,posts,delete_post,update_post}.rs`, `api/src/api/users/auth.rs`
- Test: `api/tests/security.rs`

**Interfaces:**
- Produces:
  - `pub fn not_found(msg: impl Into<String>) -> HttpError` in `utils.rs`.
  - The `query!` macro maps `diesel::result::Error::NotFound` to `not_found("Not found")`. Every other error becomes `internal_error("Database error")`, and the detail is only logged with `error!`.
  - `pub fn remove_media_files(media_name: &str)` in a new `api/src/media/files.rs`. It deletes `<base>.webp|.jxl|-800.webp|-800.jxl` or the video file and ignores `NotFound`. Task 6 extends this file.

- [ ] **Step 1: Write the failing tests in `tests/security.rs`.**
  - `cors_rejects_empty_origin`: a preflight `OPTIONS /post` with `Origin: ` and `Access-Control-Request-Method: POST` gets no `access-control-allow-origin` header.
  - `cors_allows_configured_origin`: with `Origin: http://localhost:3000` the header equals that origin.
  - `get_unknown_post_returns_404`: `GET /post/<random v4>` returns `404`.
  - `db_errors_do_not_leak`: the body of that 404 does not contain `"diesel"` or `"db"`.
  - `verify_rejects_token_without_bearer_prefix`: `Authorization: <raw token>` on `DELETE /post/<uuid>` returns `401`.
  - `verify_rejects_deleted_user`: log in, delete the user row, then call → `401`.
  - `delete_post_with_missing_file_succeeds`: insert a post with `media_name = "gone.webp"` directly through diesel, then `DELETE` → `202`.
  - `posts_sorted_newest_first`: insert two posts with `created_at` one minute apart → the newer one comes first.
  - `login_unknown_user_takes_bcrypt_time`: ten logins with an unknown username each take at least 50 ms. This stops timing-based user enumeration. Implement it with a bcrypt verify against a fixed dummy hash.
- [ ] **Step 2:** Run `cargo test --test security -- --test-threads=1`. Expected: those tests FAIL.
- [ ] **Step 3: Implement.**
  - Remove the `allowed_origin_fn` empty-origin rule.
  - `verify` uses `strip_prefix("Bearer ")` and returns `unauthorized` when the prefix is missing. A missing user maps to `unauthorized("Invalid token")`, not to a DB error.
  - `posts` orders by `created_at.desc()`.
  - `delete_post` calls `remove_media_files`.
  - The `auth` unknown-user branch runs `bcrypt::verify(password, DUMMY_HASH)` before it returns `401`.
- [ ] **Step 4:** Run the tests: PASS. Run clippy: clean.
- [ ] **Step 5:** Commit `fix(api): harden auth, CORS, errors and post deletion`.

### Task 3: Post fields, section allowlist, Logbuch rename

**Files:**
- Create: `api/migrations/w01-post-fields/{up,down}.sql`, `api/migrations/w02-logbuch/{up,down}.sql`, `api/src/api/post/section.rs`
- Modify: `api/src/db/models.rs`, `api/src/db/schema.rs` (regenerate with `diesel print-schema`, not by hand), `api/src/api/post/{new_post,update_post,posts}.rs`
- Test: `api/tests/posts.rs`

**Interfaces:**
- Produces:
  - Migration w01: `ALTER TABLE posts ADD COLUMN heading_en VARCHAR, ADD COLUMN content_en VARCHAR, ADD COLUMN season VARCHAR;`
  - Migration w02: `UPDATE posts SET name='logbuch' WHERE name='progress-logs'`. Its down reverses it.
  - `pub fn normalize_section(raw: &str) -> Result<&'static str, HttpError>` in `section.rs`. It maps `progress-logs` to `logbuch`, accepts `logbuch|team|coaches|forest-project`, and returns `bad_request("Unknown section")` for anything else.
  - `pub fn validate_season(s: &str) -> Result<(), HttpError>`. The format is `^\d{4}/\d{2}$`, and the second part must be the first year + 1 modulo 100.
  - `Post`, `NewPost` and `PostResponse` gain `heading_en: Option<String>`, `content_en: Option<String>` and `season: Option<String>`.
  - Multipart `POST /post` accepts the optional fields `heading_en`, `content_en` and `season`.
  - JSON `PUT /post/{uuid}` `UpdatePostRequest` gains the same three as `Option<String>`. An empty string clears the field to NULL.

- [ ] **Step 1: Write the failing tests in `tests/posts.rs`.**
  - `create_post_with_english_and_season`: multipart with `name=team`, `heading`, `content`, `heading_en=Team`, `content_en=Text` and `season=2017/18`. Then `GET /post/{id}` returns those values.
  - `progress_logs_alias_reads_logbuch`: create a post with `name=progress-logs`. Then `GET /posts/progress-logs` and `GET /posts/logbuch` both contain it with `"name":"logbuch"`.
  - `unknown_section_rejected`: `name=evil` → `400`.
  - `invalid_season_rejected`: `season=2017/19` → `400`, and `season=17/18` → `400`.
  - `put_clears_english_with_empty_string`: setting `heading_en=""` makes the next GET return `heading_en: null`.
  - `old_clients_still_work`: a multipart without the new fields → `200`, and the response JSON still has `id`.
- [ ] **Step 2:** Run the tests. Expected: FAIL.
- [ ] **Step 3:** Write the migrations, run them against the dev DB with `diesel migration run`, and regenerate `schema.rs`. Implement the handler changes.
  - `posts` filters on the normalized section.
  - `new_post` and `update_post` store the normalized section.
- [ ] **Step 4:** Run the tests and `diesel migration redo` twice: PASS, and `down` works.
- [ ] **Step 5:** Commit `feat(api): add English and season fields, rename progress-logs to logbuch`.

### Task 4: HTML to Markdown migration with exact rollback

**Files:**
- Create: `api/migrations/w03-content-markdown/{up,down}.sql`
- Test: `api/tests/markdown_migration.rs`

**Interfaces:**
- Produces:
  - Column `posts.content_html_backup VARCHAR` (nullable).
  - SQL function `html_to_md(text) RETURNS text`, kept in the DB so tests can call it. `down.sql` drops it.
- Conversion order inside `html_to_md`:
  1. `<br\s*/?>` → `E'\n'`
  2. `<(b|strong)>(.*?)</\1>` → `**\2**`
  3. `<(i|em)>(.*?)</\1>` → `*\2*`
  4. `<a\s+[^>]*href="([^"]*)"[^>]*>(.*?)</a>` → `[\2](\1)`
  5. Any other `<[^>]+>` → `''`
  6. The entities `&amp; &lt; &gt; &quot; &#39; &nbsp;` → their characters
- `up.sql`: `UPDATE posts SET content_html_backup = content, content = html_to_md(content) WHERE content ~ '<[a-zA-Z/]'`. Repeat the same for `content_en`; its backup is not needed, because the column is new and empty.
- `down.sql`: `UPDATE posts SET content = content_html_backup WHERE content_html_backup IS NOT NULL`, then drop the column and the function.

- [ ] **Step 1: Write the failing tests.** Each calls `SELECT html_to_md($1)` on the test DB:
  - `html_to_md_converts_bold_italic_links`: `'Wir sind <b>PaRaMeRoS</b>, <i>das</i> <a href="https://prmrs.de">Team</a>'` → `'Wir sind **PaRaMeRoS**, *das* [Team](https://prmrs.de)'`
  - `html_to_md_converts_br`: `'a<br>b<br/>c'` → `"a\nb\nc"`
  - `html_to_md_strips_unknown_tags_keeps_text`: `'<span class="x">Hallo</span> <script>x</script>'` → `'Hallo x'`
  - `html_to_md_leaves_plain_text`: `'Kein HTML & so'` → `'Kein HTML & so'`
  - `markdown_migration_down_restores_original`: insert an HTML row, run `up`, then the `down` SQL, and the content equals the original byte for byte.
- [ ] **Step 2:** Run the tests. Expected: FAIL (the function does not exist).
- [ ] **Step 3:** Write the migration and regenerate `schema.rs`.
- [ ] **Step 4:** Run the tests: PASS.
- [ ] **Step 5:** Commit `feat(api): migrate post content from HTML to Markdown with backup`.

### Task 5: Tournament endpoints

**Files:**
- Create: `api/migrations/w04-tournament/{up,down}.sql`, `api/src/api/tournament/{mod,get,put,delete}.rs`
- Modify: `api/src/api/mod.rs`, `api/src/app.rs`, `api/src/db/models.rs`, `api/src/db/schema.rs` (regenerated)
- Test: `api/tests/tournament.rs`

**Interfaces:**
- Produces:
  - Table `tournament(id INT PRIMARY KEY DEFAULT 1 CHECK (id = 1), name VARCHAR NOT NULL, place VARCHAR, starts_at TIMESTAMPTZ NOT NULL, ends_at TIMESTAMPTZ, updated_at TIMESTAMPTZ NOT NULL)`.
  - `GET /tournament` returns `200 {"name","place","starts_at","ends_at","updated_at"}` with RFC 3339 times including the offset, or `204` when no tournament is set.
  - `PUT /tournament` takes the JSON body `{"name": String, "place": Option<String>, "starts_at": DateTime<FixedOffset>, "ends_at": Option<DateTime<FixedOffset>>}`. It requires `verify`, upserts on `id=1`, and returns `200` with the stored object. It returns `400` when `name` is empty, longer than 120 chars, or `ends_at <= starts_at`.
  - `DELETE /tournament` requires `verify` and returns `204`.

- [ ] **Step 1: Write the failing tests.**
  - `tournament_empty_returns_204`
  - `put_requires_auth`: `401`
  - `put_then_get`
  - `tournament_roundtrips_offset_instant`: `starts_at` `2027-03-28T10:00:00+02:00` comes back as the same instant.
  - `end_before_start_rejected`: `400`
  - `delete_clears`
- [ ] **Step 2:** Run the tests. Expected: FAIL.
- [ ] **Step 3:** Implement. Store as `chrono::DateTime<Utc>` and serialize with `to_rfc3339`.
- [ ] **Step 4:** Run the tests: PASS.
- [ ] **Step 5:** Commit `feat(api): add tournament endpoints`.

### Task 6: Media pipeline, limits, JPEG XL, media endpoints, backfill

**Files:**
- Create: `api/src/media/{mod,files,image,video,backfill}.rs`, `api/src/api/post/media.rs`
- Modify: `api/src/api/post/new_post.rs` (move `process_field`, `to_webp` and `save_media` into `media/`), `api/src/main.rs` (spawn backfill), `api/src/app.rs`, `api/Cargo.toml`, `api/Dockerfile`, `api/dev.Dockerfile`
- Test: `api/tests/media.rs`, with fixture files in `api/tests/fixtures/` (a 1600×1200 JPEG, a PNG, a tiny MP4, a tiny WebM, a text file named `x.jpg`)

**Interfaces:**
- Produces:
  - `pub async fn read_limited(field: &mut Field, max: usize) -> Result<Vec<u8>, HttpError>`. It returns `HttpError` `413` as soon as the read bytes exceed `max`. Add `pub fn payload_too_large(msg)` to `utils.rs`.
  - `pub enum MediaKind { Image, Video(VideoFormat) }` and `pub enum VideoFormat { Mp4, Webm }`.
  - `pub fn sniff(bytes: &[u8]) -> Result<MediaKind, HttpError>`:
    - Images through `image::guess_format`, allowing only Jpeg, Png and WebP.
    - MP4 through `ftyp` at offset 4.
    - WebM through `1A 45 DF A3`.
    - Anything else → `400`.
    - The declared `media_type` must agree: `img` for images, `vid` for videos.
  - `pub fn store_image(base: &str, bytes: &[u8]) -> Result<String, HttpError>`:
    - Checks the dimensions with `image::ImageReader::with_guessed_format()?.into_dimensions()` before decoding, and returns `400` when `w*h > 40_000_000`.
    - Writes the 4 variants.
    - Returns `"<base>.webp"`.
  - `pub fn store_video(base: &str, fmt: VideoFormat, bytes: &[u8]) -> Result<String, HttpError>`. It returns `"<base>.mp4"` or `"<base>.webm"`.
  - **New uploads get the base name `"<post-uuid>-<8 random hex>"`.** Client file names are no longer used. Existing `media_name` values are untouched.
  - `PUT /post/{uuid}/media` takes the same `media_type`, `media_creator` and file fields as `POST /post`. It requires `verify`, replaces the media, removes the old files and sets `updated_at`. It returns `200 {"media_name": …}`.
  - `DELETE /post/{uuid}/media` requires `verify`, clears all three media columns, removes the files and returns `204`.
  - `pub async fn backfill(upload_dir: PathBuf)`. For every `*.webp` that is not `*-800.webp` and lacks a variant, it creates the missing ones, logs a summary and never panics. `main.rs` runs it with `tokio::spawn` after the server binds.
  - Uses `jpegxl-rs` with the `vendored` feature: effort 7, distance 1.0 for full size and 1.5 for `-800`.

- [ ] **Step 1: Write the failing tests.**
  - `jpeg_upload_creates_four_variants`: all four files exist, the `-800.webp` is 800 px wide, and the `.jxl` starts with `FF 0A` or the container signature `00 00 00 0C 4A 58 4C 20`.
  - `small_image_not_upscaled`: a 400 px PNG gives a 400 px wide `-800.webp`.
  - `oversized_image_returns_413`: 15 MB + 1 byte.
  - `fake_jpeg_returns_400`: a text file named `x.jpg`.
  - `too_many_pixels_returns_400`: a fixture PNG of 8000×6000.
  - `webm_video_stored_as_webm`
  - `oversized_video_returns_413`: generate 100 MB + 1 byte in the test. Do not commit it.
  - `media_type_mismatch_returns_400`: `media_type=img` with an MP4.
  - `replace_media_keeps_uuid_and_removes_old_files`
  - `delete_media_clears_columns`
  - `backfill_creates_missing_variants`: copy a legacy `.webp` into a temp upload dir, run `backfill`, and three new files appear.
  - Each test uses its own temp upload dir: make `UPLOAD_DIR` a field `Env.upload_dir`, defaulting to `/api/uploads`.
- [ ] **Step 2:** Run the tests. Expected: FAIL.
- [ ] **Step 3:** Implement the media code and the routes.
  - The build stages switch to `cgr.dev/chainguard/rust:latest-dev` and add `RUN apk add --no-cache cmake clang` for libjxl.
  - The runtime image needs `libstdc++` (`apk add libstdc++` in the `wolfi-base` stage).
- [ ] **Step 4:** Run the tests: PASS. Then build both architectures locally or in CI:
  - `docker buildx build --platform linux/amd64,linux/arm64 api/`
  - Record both build times in the PR description next to today's (~5 min aarch64).
- [ ] **Step 5:** Commit `feat(api): validated media pipeline with JPEG XL variants and media endpoints`.

### Task 7: Redis rate limiting and caching

**Files:**
- Create: `api/src/redis_store.rs`, `api/src/client_ip.rs`
- Modify: `api/src/lib.rs` (`Env.redis` connect in `main.rs` from `REDIS_URL` with a 2 s timeout; on failure log a warning and use `None`), `api/src/api/users/{auth,register}.rs`, `api/src/api/post/*.rs`, `api/src/api/post/media.rs`
- Test: `api/tests/redis.rs`

**Interfaces:**
- Produces:
  - `pub fn client_ip(req: &HttpRequest) -> IpAddr`
  - `pub async fn hit_limit(env: &Env, key: &str, max: u32, window_secs: u64) -> Option<u64>`:
    - Uses `INCR` plus `EXPIRE NX`.
    - Returns `Some(retry_after_secs)` when over the limit, and `None` when under it or when Redis is down.
  - `pub async fn reset_limit(env: &Env, key: &str)`
  - `pub async fn cache_get(env: &Env, key: &str) -> Option<String>`
  - `pub async fn cache_set(env: &Env, key: &str, json: &str, ttl_secs: u64)`
  - `pub async fn cache_clear_posts(env: &Env)`
  - **Key names:**
    - `rl:login:<ip>`, `rl:register:<ip>`
    - `cache:posts:<section>`, `cache:post:<uuid>`
    - `cache_clear_posts` deletes `cache:posts:*` and `cache:post:*` with `SCAN` + `DEL`. Never use `KEYS`.
  - **Login:** counts only failed attempts. A success calls `reset_limit`. Over the limit returns `429` with `Retry-After`, checked before bcrypt.
  - **Register:** counts every attempt.
  - **Every write handler** calls `cache_clear_posts`: `POST /post`, `PUT /post/{uuid}`, `DELETE /post/{uuid}`, and both `/post/{uuid}/media` routes.

- [ ] **Step 1: Write the failing tests.** They need `TEST_REDIS_URL`.
  - `sixth_failed_login_returns_429_with_retry_after`
  - `successful_login_resets_counter`
  - `fourth_register_attempt_returns_429`
  - `rate_limit_uses_x_real_ip`: two IPs are counted separately.
  - `posts_cached_then_invalidated_on_create`: GET, insert a row directly through diesel, and GET again still shows the old list. Then POST through the API, and GET shows all rows.
  - `api_works_when_redis_down`: `Env.redis` points at `redis://127.0.0.1:1`, login and GET posts succeed, and there are no 429s.
- [ ] **Step 2:** Run the tests. Expected: FAIL.
- [ ] **Step 3:** Implement.
- [ ] **Step 4:** Run the tests: PASS.
- [ ] **Step 5:** Commit `feat(api): Redis rate limiting and post caching with fail-open`.

### Task 8: Sitemap

**Files:**
- Create: `api/src/api/sitemap.rs`
- Modify: `api/src/api/mod.rs`, `api/src/app.rs`, `frontend/nginx.conf` (add `location = /sitemap.xml { proxy_pass http://api:8080/sitemap.xml; }`)
- Test: `api/tests/sitemap.rs`

**Interfaces:**
- Produces:
  - `GET /sitemap.xml` with `Content-Type: application/xml; charset=utf-8`.
  - Fixed paths: `/`, `/About`, `/team`, `/logbuch`, `/prmrs/forest-project`, `/Imprint`, `/Privacy-Policy`.
  - Plus `/post/{uuid}` for every post.
  - Each path appears as a `<url>` for DE (`{SITE_URL}{path}`) and for EN (`{SITE_URL}/en{path}`, and for `/` that is `/en/`). Each `<url>` carries the alternates `hreflang="de"`, `hreflang="en"` and `hreflang="x-default"` (the DE URL).
  - Post `lastmod` is `updated_at` or `created_at` as `YYYY-MM-DD`.
  - Cached under the key `cache:sitemap`, cleared by `cache_clear_posts`.

- [ ] **Step 1: Write the failing tests.**
  - `sitemap_lists_fixed_pages_in_both_languages`
  - `sitemap_includes_post_with_lastmod`
  - `sitemap_escapes_xml`: `SITE_URL` with `&` gets escaped.
  - `sitemap_has_hreflang_alternates`
- [ ] **Step 2:** Run the tests. Expected: FAIL.
- [ ] **Step 3:** Implement with `format!` and XML escaping of the 5 entities. No XML crate is needed.
- [ ] **Step 4:** Run the tests: PASS.
- [ ] **Step 5:** Commit `feat(api): generate sitemap.xml with hreflang`.

### Task 9: Deployment config

**Files:**
- Modify: `docker-compose.yml`, `docker-compose.dev.yml`, `.env.example`, `dev.sh`, `dev.ps1`, `CLAUDE.md` (commands and architecture), `api/README.md` (route table)

**Interfaces:**
- Produces:
  - **Service `redis`:** image `cgr.dev/chainguard/redis:latest`, no published port, `restart: unless-stopped`.
  - **API env** gains `REDIS_URL=redis://redis:6379` and `SITE_URL=${SITE_URL}`. `.env.example` gains `SITE_URL="https://example.net"`.
  - **Production `docker-compose.yml`** publishes the API as `"127.0.0.1:${API_PORT}:8080"`.
  - **`dev.sh` / `dev.ps1`:** drop the manual `diesel migration run` step, because the API migrates itself.

- [ ] **Step 1:** Run `docker compose -f docker-compose.dev.yml up -d --build`, then check:
  - `curl -s localhost:${API_PORT}/posts/logbuch` returns `[]`
  - `curl -s localhost:${API_PORT}/sitemap.xml | head -3` shows the XML declaration
  - `docker compose logs api` shows that migrations were applied
- [ ] **Step 2:** Stop Redis with `docker compose stop redis`, then check that `curl` on `/posts/logbuch` still returns `200`.
- [ ] **Step 3:** Commit `chore: add Redis service, bind API to localhost, self-migrating dev setup`.

### Task 10: Security audit of the API

**Files:** whatever the findings touch, plus `docs/superpowers/plans/2026-11-07-api-audit-findings.md`

- [ ] **Step 1:** Dispatch the project agent `security-reviewer` on `git diff main...HEAD -- api frontend/nginx.conf docker-compose*.yml`. Also run `cargo audit`.
- [ ] **Step 2:** For every finding rated critical, high or medium:
  1. Write a failing test in the matching `api/tests/*.rs`.
  2. Fix it.
  3. Make the test pass.
  4. Commit `fix(api): <finding>`.
- [ ] **Step 3:** List the low findings in the findings file for the owner, then commit it.
- [ ] **Step 4:** Run the full suite `cargo test -- --test-threads=1`, `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt --check`. Everything must be green.

---

## Rollout (owner, after Plans A and B merge)

0. Do not merge `redesign` to `main` before Plan B is done and the version is bumped: CD publishes `:latest`, and migration w03 breaks the old frontend.
1. Back up the database with `pg_dump` and copy the `uploads` volume.
2. Check the production data with read-only queries:
   - `SELECT DISTINCT name FROM posts;` should list only the 4 sections plus `progress-logs`.
   - `SELECT DISTINCT (regexp_matches(content, '<(/?[a-zA-Z]+)', 'g'))[1] FROM posts;` lists the HTML tags in use. Report any tag outside `b strong i em br a`.
   - `SELECT * FROM __diesel_schema_migrations;` must list `posts` and `users`. If not, stop: the embedded runner would try to re-create the tables.
3. Deploy the API and frontend images together. The API applies migrations w01 to w04 on start, and the backfill runs in the background.
4. Host nginx: `set_real_ip_from <Cloudflare ranges>; real_ip_header CF-Connecting-IP;` and `proxy_set_header X-Real-IP $remote_addr;` (plain `$http_cf_connecting_ip` is spoofable if the origin is reachable directly).
5. Once production looks right, a later release drops `content_html_backup` in a migration w05.
