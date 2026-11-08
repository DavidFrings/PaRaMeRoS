## What does this PR do?

<!-- Short summary. Link issues with "Closes #123". -->

## Type of change

<!-- Ticked boxes become labels automatically (see .github/workflows/pr-labels.yml). -->

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation
- [ ] Refactoring / chore
- [ ] CI / build / dependencies

## Review checklist

Formatting, linting, type checks, tests, docs and the dependency checks for the API and the frontend run in CI.
The following points cannot be checked by a tool (API / Rust):

- [ ] New public items are documented; `# Errors` and `# Panics` sections are present where relevant
- [ ] No `clone()`, `Rc` or `Arc` just to appease the borrow checker
- [ ] Parameters are as general as sensible (`&str` instead of `&String`, `&[T]` instead of `&Vec<T>`)
- [ ] Easily confused values use a newtype instead of a bare `u64` or `String`
- [ ] No `bool` parameters whose meaning is unclear at the call site (use an enum)
- [ ] `unwrap()` is only used with a good reason, otherwise `expect("...")` or proper error handling
- [ ] Every `unsafe` block has a `SAFETY:` comment
