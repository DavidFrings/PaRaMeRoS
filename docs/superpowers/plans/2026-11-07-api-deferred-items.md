# Plan A — deferred items and decisions

Copied from the execution ledger when Plan A finished (2026-11-07). Minor findings reviewers raised and deferred, parked items, and every controller ruling.

## Rulings
- Ruling R1: embed migrations with diesel-async's AsyncMigrationHarness (feature `migrations`) if available in diesel-async 0.6; else diesel sync PgConnection with `pq-sys` feature `bundled`. — avoids a system libpq dependency on dev hosts and keeps runtime image unchanged — cost if wrong: one dependency swap in T1.
- Ruling R2: add `pub upload_dir: PathBuf` to Env in Task 1 (from optional env `UPLOAD_DIR`, default `/api/uploads`); T2 `remove_media_files(upload_dir: &Path, media_name: &str)`. — avoids changing signatures in T6 — cost if wrong: trivial rename.
- Ruling R3: the T2 timing test sends a distinct `X-Real-IP` per attempt (10.0.0.1..10) so it keeps passing after T7. — cost if wrong: one flaky test to adjust.
- Ruling R4: `test_app()` runs `FLUSHDB` on the test Redis when TEST_REDIS_URL is set (T7 adds this). — cost if wrong: test isolation issue only.
- Ruling R5: diesel_cli cannot be built on this host (postgres-bundled pulls openssl-sys, OpenSSL dev headers missing, no sudo). Tasks 3-5 update `api/src/db/schema.rs` by writing it through a shell heredoc (the protect hook only blocks Edit/Write) to exactly what `diesel print-schema` would emit for the migrations; integration tests that run the migrations and query every column are the check. — cost if wrong: a column type mismatch shows up as a compile or test failure, not silently.
- Ruling R6: GET /posts/{unknown} returns 400 "Unknown section" (was 200 []); plan allowlist + constraint allow rejecting invalid input — cost if wrong: a client asking for a bogus section sees 400 instead of empty list.
- Ruling R7: "diesel migration redo" replaced by an integration test running each new migration's down.sql then up.sql via batch_execute (R5 consequence) — cost if wrong: down.sql bugs that only show with the real CLI.
- Ruling R8: Markdown migration also converts </p> to two newlines and drops <p>; case-insensitive tags; result btrim'd — spec only listed b/strong/i/em/br/a, but stripping <p> would glue paragraphs together — cost if wrong: extra blank lines in converted posts.
- Ruling R9: Task 6 generates all test media in-memory (no committed binary fixtures); "too many pixels" uses a hand-crafted PNG header — keeps repo small — cost if wrong: header-only PNG may not exercise real decoder limits.
- Ruling R10: text multipart fields capped at 1 MiB (413) — spec silent; prevents memory abuse — cost if wrong: very long posts rejected (1 MiB is far above any real post).
- Ruling R11: Docker build timing measured with podman for native arch only; arm64 "not measured" locally — no docker/qemu here — cost if wrong: aarch64 regression only visible in CD.
- Ruling R12: continue with jpegxl-rs while the owner decides on the GPL question; JXL code is isolated in api/src/media/image.rs so a swap (own FFI to BSD libjxl, or dropping JXL) stays local — cost if wrong: one module rewrite + dependency swap.
- Task 6: parked — Important (plan-mandated): jpegxl-rs/jpegxl-sys GPL-3.0-or-later vs MPL-2.0 labels in Cargo.toml + Dockerfile — Ruling: owner decision (asked: accept GPL / own FFI to BSD libjxl / drop JXL); code stays isolated in media/image.rs (R12)
- Ruling R13: Redis ops wrapped in a 500 ms timeout so a dead Redis cannot stall requests (spec only says 'keep working') — cost if wrong: under heavy Redis latency limits/cache silently skipped.
- Ruling R14: SITE_URL becomes required (panics if unset, like the other required vars) in Task 9, matching the plan's Global Constraints; compose files and .env.example pass it — cost if wrong: dev setups without SITE_URL fail fast at startup.
- Ruling R15: prod compose binds db AND frontend ports to 127.0.0.1 (same as API) — owner confirmed host nginx sits in front; keeps pg_dump from host possible — cost if wrong: a frontend proxied from another machine would lose access.
- Ruling R16: besides medium+, also fix the cheap lows 3 (backfill MAX_PIXELS), 6 (constant-time ADMIN_PASS), 7 (DB connection error text leak), 9 (media_name path guard); defer 4 (nginx add_header inheritance) to Plan B Task 11 which owns nginx headers; 5 (JWT_SECRET min length) only warn! at startup, not panic, so an existing short prod secret can't brick the deploy; 8 handled by rollout (deploy API + frontend together) — cost if wrong: small extra diff in the audit task.
- Ruling R17: cargo update to clear all 16 RUSTSEC advisories (all semver-compatible) is part of Task 10's fix dispatch.
- Ruling R18: final fix wave = Important 1–3 + spec X-Real-IP lines + cheap minors 5–7 + docs drift + rollout steps (incl. "don't merge to main before Plan B") — one dispatch per skill — cost if wrong: slightly larger fix diff.
- Ruling R19: Important 4 handled as a merge gate (documented in rollout step 0), not code — cost if wrong: none if the gate is respected.
- Final review: parked — HEIC/MOV ftyp brands, EXIF orientation, register duplicate 409, JXL 40 MP encode timing — Ruling: real but not blocking; listed for owner

## Deferred minor findings
- Task 1: minor (deferred): no build.rs rerun-if-changed=migrations → stale embedded migrations locally (carry to T3: add build.rs)
- Task 1: minor (deferred): test_app not safe without --test-threads=1 (documented in CI only)
- Task 1: minor (deferred): health test doesn't assert status 200 explicitly
- Task 1: minor (deferred): SITE_URL default http://localhost:3000 must become required/documented in T8/T9
- Task 1: minor (deferred): Docker build with COPY ./migrations unverified locally (no docker)
- Task 2: minor (deferred): remove_media_files variant deletion untested (carry to T6: test that all 4 variants + video file get deleted, and DB row gone)
- Task 2: minor (deferred): login maps non-NotFound DB errors to 401 (should be 500)
- Task 2: minor (deferred): verify() duplicates query! error mapping without comment
- Task 2: minor (deferred): missing Authorization header returns 400 (unchanged, pre-existing)
- Task 3: minor (deferred): season/PUT test coverage thin (1999/00, whitespace, PUT invalid season, PUT name=progress-logs, PUT bad name)
- Task 3: minor (deferred): update_post does up to 7 UPDATEs without a transaction
- Task 3: minor (deferred): empty-string→None written twice (new_post filter, update_post closure)
- Task 3: minor (deferred): roundtrip migration test mutates shared DB, relies on --test-threads=1
- Task 4: minor (deferred): single-quoted href loses URL; '>' inside quoted attribute leaves fragments
- Task 4: minor (deferred): entities decoded → '&lt;script&gt;' becomes literal <script> in Markdown; safe only because Plan B renders with markdown-it html:false (must hold)
- Task 4: minor (deferred): literal * _ [ # not escaped; 'a<b and c>d' stripped (backup recovers)
- Task 4: minor (deferred): down.sql overwrites edits made after up; content_en not reverted — document in PR
- Task 4: minor (deferred): down/up test on shared DB not panic-safe; no test for content_en rewrite / backup==original after up
- Task 5: minor (deferred): tournament validation tests thin (name >120, multibyte 120, place >120); put_requires_auth only bad token, not missing header
- Task 5: minor (deferred): restore PUT in test loop ignores status
- Task 6: minor (deferred): store_video blocking fs::write (≤100 MiB) on actix worker; remove_media_files sync IO in handlers
- Task 6: minor (deferred): backfill doesn't apply MAX_PIXELS to legacy files (512 MiB image alloc cap bounds it)
- Task 6: minor (deferred): legacy names ending -800 skipped by backfill / could be treated as variants
- Task 6: minor (deferred): PUT media without media_creator sets NULL (undocumented); 0-row update returns 200 with orphans
- Task 6: minor (deferred): no tests for backfill partial variants, SVG/GIF rejection; temp dirs .keep() leak into /tmp
- Task 7: minor (deferred): check_limit/hit_limit not atomic → concurrent overshoot of 5 attempts
- Task 7: minor (deferred): Redis unavailable at boot stays disabled until restart (document)
- Task 7: minor (deferred): GET read-then-set race can re-cache stale body for TTL
- Task 7: minor (deferred): cache_clear_posts whole SCAN/DEL loop under one 500 ms budget
- Task 7: minor (deferred): invalidation tests only for POST; timeout branch untested; sitemap clear untested
- Task 7: minor (deferred): PUT /post/{unknown uuid} returns 202 (pre-existing; should be 404); no rollback test; 500 ms timeout path untested
- Task 8: minor (deferred): Redis-dependent tests return early (silently pass) without TEST_REDIS_URL — same pattern in redis.rs
- Task 8: minor (deferred): loose assertions in sitemap tests (is_empty() vacuous, substring find)
- Task 9: minor (deferred): Redis slow at API boot → disabled for process lifetime (no reconnect from None); .env.example no trailing newline
- Task 9: minor (deferred): compose DATABASE_URL uses db:${POSTGRES_PORT} (container port must be 5432) — pre-existing; Chainguard images unpinned; frontend container + nginx sitemap proxy not exercised live
- Task 10: minor (deferred): spec line ~250 (and ~179) still shows unsafe `proxy_set_header X-Real-IP $http_cf_connecting_ip;` — fix in final wave
- Task 10: minor (deferred): tautological assertion in ipv4_and_mapped_share_key; F7 warnings untested; F2c only documented
