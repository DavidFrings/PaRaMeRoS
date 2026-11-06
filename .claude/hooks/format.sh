#!/bin/sh
# PostToolUse: format the edited file so CI format checks pass.
file=$(jq -r '.tool_input.file_path // empty')
[ -f "$file" ] || exit 0
root="$CLAUDE_PROJECT_DIR"

case "$file" in
  *.rs) rustfmt --edition 2024 "$file" ;;
  "$root"/frontend/src/*)
    # shortcut: skips silently until `pnpm install` has run in frontend/
    prettier="$root/frontend/node_modules/.bin/prettier"
    [ -x "$prettier" ] && "$prettier" --write --log-level warn "$file"
    ;;
esac
exit 0
