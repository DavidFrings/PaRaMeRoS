# API security audit: findings

Scope: `redesign` vs `main`, plus `cargo audit`. SHAs refer to the `redesign` branch.

| ID | Severity | Status | Note |
|----|----------|--------|------|
| F1 | High | fixed in 2287bd1 | Prod compose published Postgres and frontend on 0.0.0.0 (Docker bypasses host firewalls). Both now bind `127.0.0.1`. |
| F2a | Medium | fixed in 2287bd1 | Rate-limit keys use the IPv6 /64 (IPv4-mapped become IPv4), so address rotation no longer evades limits. |
| F2b | Medium | rollout note | Host nginx must use `set_real_ip_from <Cloudflare ranges>; real_ip_header CF-Connecting-IP;` and `proxy_set_header X-Real-IP $remote_addr;`, or firewall the origin to Cloudflare. Documented in `CLAUDE.md`; plan rollout line corrected. A missing header still puts all clients on the peer address (global lockout after 5 bad logins). |
| F3 | Low | fixed in 56828f6 | Backfill checked no pixel limit; huge legacy image could crash-loop the API. Now skipped with a warning. |
| F4 | Low | fixed in 56828f6 | `ADMIN_PASS` compared in constant time (`subtle`). |
| F5 | Low | fixed in 56828f6 | DB pool errors were returned to clients; now logged, client gets "Database error". |
| F6 | Low | fixed in 56828f6 | `remove_media_files` refuses empty names and `/`, `\`, `..`. |
| F7 | Low | fixed in 56828f6 | Startup warns (no panic) if `JWT_SECRET` < 32 bytes or `ADMIN_PASS` < 12 chars. Set strong values in prod. |
| F8 | High (deps) | fixed in 8345a58 | 16 RUSTSEC vulnerabilities cleared, see below. |
| D1 | Medium | deferred to Plan B Task 11 | nginx `add_header` inside `location` drops server-level security headers for `/uploads/` and the static-asset regex. Also add `image/jxl jxl` mime type. |
| D2 | Medium | rollout note | Migration w03 turns `&lt;`/`&gt;` into raw `<`/`>`; the OLD frontend renders posts with `v-html`. Deploy API and new frontend together. |
| D3 | Low | owner decision / known | Login rate limit is check-then-increment: concurrent requests can exceed the limit slightly. |
| D4 | Low | known | If Redis is down at boot, rate limiting and caching stay disabled until restart (fail-open). |
| D5 | Legal | owner decision | `jpegxl-rs` is GPL-3.0; decided by owner 2026-11-07: accepted; image labelled MPL-2.0 AND GPL-3.0-or-later. |

## cargo audit

- Before: 16 vulnerabilities (actix-http, actix-web, diesel, h2, tokio-postgres, postgres-protocol, zstd family, crossbeam-epoch, ctutils), 25 warnings.
- After: 0 vulnerabilities, 2 warnings.
- `cargo update` alone cleared 7 of them but not diesel and h2.
- diesel 2.2 was pinned by `diesel_migrations 2.2` / `diesel-async 0.6`. Bumped `diesel ^2.3`, `diesel_migrations ^2.3`, `diesel-async ^0.7` (0.7.4).
- h2 0.3 (RUSTSEC-2026-0258) has no 0.3 fix and actix-http still uses 0.3. Disabled the `http2` feature of actix-web (nginx talks HTTP/1.1 to the API). Revisit when actix-http moves to h2 0.4.
- Remaining warnings: `paste` unmaintained (transitive), `diesel-async` RUSTSEC-2026-0138 (MySQL backend only; this project uses Postgres; reported as warning, no patched 0.7 release).
- Docker images now build with `cargo build --locked` from the committed `Cargo.lock` (the old `lockfile` stage regenerated it), so the 0-vulnerability result covers the shipped image.
