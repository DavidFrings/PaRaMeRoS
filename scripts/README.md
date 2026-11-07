# Scripts

## import-legacy-content.mjs

One-time import of the old hardcoded About content (coaches text and the seven season texts with photos, in `legacy-content/`) into the API as `coaches` and `team` posts.

Prerequisites: Node 22 and an admin account of the site.

```sh
API=https://parameros.net/api USERNAME=… PASSWORD=… node scripts/import-legacy-content.mjs [--dry-run]
```

- Creates one `coaches` post unless a coaches post exists.
- Creates a `team` post for each season 2017/18, 2019/20 to 2024/25, unless a team post for that season exists (by its `season` field or, without one, by the season of its creation date).
- `--dry-run` logs in and reads, prints what it would create and writes nothing.
- Prints one line per item. Exits non-zero on any HTTP error.

Safe to re-run: existing items are skipped. Run it once, right after the deploy that ships the redesign.
