---
name: new-endpoint
description: Checklist for adding or changing an Actix-web API endpoint in api/src/api/. Use whenever a route is added, renamed or removed.
---

An endpoint touches four places. Do all of them:

1. Handler file in `api/src/api/<area>/<name>.rs`. Follow an existing handler such as `post/update_post.rs`:
   - `#[get|post|put|delete("/path")]` macro on a `pub async fn` returning `Result<HttpResponse, HttpError>`.
   - Take `env: Data<Env>` and get a connection with `db(&env).await?`.
   - Wrap Diesel calls in `query!(...)`.
   - Use `bad_request` / `unauthorized` / `forbidden` / `internal_error` from `crate::utils` for errors.
   - Protected route: call `verify(&req, &env).await?` before anything else.
2. Add `pub mod <name>;` to `api/src/api/<area>/mod.rs`.
3. Import the handler in `api/src/main.rs` and register it with `.service(<handler>)` in the `App::new()` chain. Without this the route returns 404.
4. Add or update the row in the route table in `api/README.md`.

New table or column: add a migration directory in `api/migrations/` with `up.sql` and `down.sql`, then run `diesel migration run` to regenerate `src/db/schema.rs`. Never edit `schema.rs` by hand.

New allowed HTTP method or header: update the CORS setup in `main.rs` too.

Finish with `cargo clippy --all-targets --all-features -- -D warnings` in `api/`.
