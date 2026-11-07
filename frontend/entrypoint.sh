#!/bin/sh
# Write each value as a JS string literal: drop control chars and one pair of
# surrounding quotes (compose passes VITE_X="${X}"), escape \ and ".
js() {
  printf '"%s"' "$(printf '%s' "$1" | tr -d '\000-\037' | sed -e 's/^"\(.*\)"$/\1/' -e 's/\\/\\\\/g' -e 's/"/\\"/g' -e 's/</\\u003c/g')"
}

{
  echo "window.__ENV__ = {"
  echo "  NAME: $(js "$VITE_NAME"),"
  echo "  DESC: $(js "$VITE_DESC"),"
  echo "  KEYWORDS: $(js "$VITE_KEYWORDS"),"
  echo "  API: $(js "$VITE_API")"
  echo "};"
} > /usr/share/nginx/html/config.js

exec nginx -c /etc/nginx/nginx.conf -e /dev/stderr -g "daemon off;"
