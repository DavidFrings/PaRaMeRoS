#!/bin/sh
echo "window.__ENV__ = {
  NAME: \"${VITE_NAME}\",
  DESC: \"${VITE_DESC}\",
  KEYWORDS: \"${VITE_KEYWORDS}\",
  API: \"${VITE_API}\"
}; 
console.log(api);" > /usr/share/nginx/html/config.js
