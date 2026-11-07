---
name: ci-check
description: Run the same format, lint and type checks as .github/workflows/ci.yml locally, for the API and the frontend.
disable-model-invocation: true
---

Run each check below and report pass or fail per check. Quote the first decisive error line for each failure. Do not fix anything unless asked.

API (in `api/`):

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --locked --all-targets --all-features
```

Frontend (in `frontend/`). Run `pnpm install --frozen-lockfile` first if `node_modules/` is missing:

```sh
pnpm exec prettier --check src/
pnpm exec eslint . --max-warnings 0
pnpm run type-check
```

If `$ARGUMENTS` is `api` or `frontend`, run only that half.
