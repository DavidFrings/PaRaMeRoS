import type de from './de'

const en: typeof de = {
  nav: {
    start: 'Home',
    about: 'About us',
    team: 'Team',
    logbuch: 'Logbook',
    menu: 'Menu',
    menuClose: 'Close menu',
    main: 'Main navigation',
    skip: 'Skip to content',
    languageGroup: 'Language',
    imprint: 'Legal notice',
    privacy: 'Privacy policy',
    sitemap: 'Sitemap',
    cookies: 'Cookies',
    forest: 'Forest project (Year 9 PRMRS)',
    school: 'Pater-Rupert-Mayer-Realschule',
  },
  home: {},
  logbuch: {
    title: 'Logbook',
    empty: 'No entries yet',
    live: 'Live',
    season: 'Season',
    photoBy: 'Photo: {creator}',
    videoBy: 'Video: {creator}',
  },
  team: {},
  about: {},
  admin: {},
  legal: {},
  notFound: {
    title: 'Page not found',
    text: 'This page does not exist.',
    home: 'Back to home',
  },
  time: {},
  countdown: {
    days: '{name} in 1 day | {name} in {n} days',
    clock: '{name} in {hms}',
    running: 'Happening now',
  },
}

export default en
