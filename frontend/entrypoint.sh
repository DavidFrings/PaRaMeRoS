#!/bin/sh
echo "window.__ENV__ = {
  NAME: ${VITE_NAME},
  DESC: ${VITE_DESC},
  KEYWORDS: ${VITE_KEYWORDS},
  API: ${VITE_API}
};" > /usr/share/nginx/html/config.js

exec nginx -c /etc/nginx/nginx.conf -e /dev/stderr -g "daemon off;"
