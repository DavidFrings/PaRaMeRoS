# Contributing

Thanks for your interest in contributing! Please read the [Code of Conduct](CODE_OF_CONDUCT.md) first.

## Before you start

- For bugs and feature ideas, open an issue using one of the templates.
- For anything bigger than a small fix, please discuss it in an issue before writing code.
- Security problems: see [SECURITY.md](SECURITY.md), not a public issue.

## Local checks

CI runs the same commands, so you can find problems before pushing.

API (in `api/`):

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked
cargo deny check
# Integration tests need a real Postgres and Redis; without TEST_DATABASE_URL they are skipped.
TEST_DATABASE_URL=postgres://... TEST_REDIS_URL=redis://... cargo test --locked -- --test-threads=1
```

The lint configuration lives in the `[lints]` tables of `api/Cargo.toml`, so your editor shows the same warnings.

Frontend (in `frontend/`):

```sh
pnpm install --frozen-lockfile
pnpm exec prettier --check src/
pnpm exec eslint . --max-warnings 0
pnpm run type-check
pnpm test
```

## Pull requests

- Keep a pull request focused on one change.
- Tick the matching boxes under "Type of change"; they are turned into labels automatically.
- Go through the review checklist in the template.
- Pull requests are squash-merged, so the PR title becomes the commit message. Use a
  [Conventional Commits](https://www.conventionalcommits.org/) title (e.g. `fix(api): retry failed uploads`).
