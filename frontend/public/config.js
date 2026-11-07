// Dev/preview defaults (API is the e2e mock host). In the Docker image entrypoint.sh overwrites this file from VITE_* env vars.
window.__ENV__ = {
  NAME: 'PaRaMeRoS',
  DESC: 'Das Robotik-Team der Pater-Rupert-Mayer-Realschule',
  KEYWORDS: 'PaRaMeRoS, FIRST LEGO League, Robotik, PRMRS',
  API: 'http://api.test',
}
