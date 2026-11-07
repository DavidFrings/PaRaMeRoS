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

# CSP connect-src: 'self' plus the API origin (scheme://host[:port]) from VITE_API.
# Only [A-Za-z0-9.:/-] is accepted so the value cannot break out of the header;
# anything else (or a relative path like /api) falls back to 'self'.
api="$(printf '%s' "$VITE_API" | sed -e 's/^"\(.*\)"$/\1/')"
origin=""
case "$api" in
  *[!A-Za-z0-9.:/-]*) [ -n "$api" ] && echo "entrypoint: VITE_API has unsafe characters, CSP connect-src falls back to 'self'" >&2 ;;
  *) origin="$(printf '%s' "$api" | sed -nE 's#^(https?://[A-Za-z0-9.-]+(:[0-9]{1,5})?)(/.*)?$#\1#p')" ;;
esac
echo "add_header Content-Security-Policy \"default-src 'self'; img-src 'self' data: blob:; style-src 'self' 'unsafe-inline'; script-src 'self'; connect-src 'self' $origin; font-src 'self' data:; object-src 'none'; base-uri 'self'; frame-ancestors 'none';\" always;" > /tmp/csp.conf

exec nginx -c /etc/nginx/nginx.conf -e /dev/stderr -g "daemon off;"
