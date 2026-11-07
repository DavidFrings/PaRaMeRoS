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
  home: {
    heroTitle: 'The PRMRS robotics team',
    heroText:
      'Students of the Pater-Rupert-Mayer-Realschule, competing in the FIRST LEGO League since 2017',
    meetTeam: 'Meet the team',
    toLogbuch: 'Go to the logbook',
    teamCaption: 'Team {season}',
    teamAlt: 'Team of the {season} season',
    newInLogbuch: 'New in the logbook',
    allEntries: 'All entries',
    whoTitle: 'Who we are',
    whoText:
      'We are PaRaMeRoS, the FIRST LEGO League team of the Pater-Rupert-Mayer-Realschule. As dedicated students we love robotics and technology, and bring creativity, team spirit and passion to it.',
    moreAbout: 'More about us',
    seasonsTitle: 'Our seasons',
    seasonsCount: '{n} team since {oldest} | {n} teams since {oldest}',
    logbuchTitle: 'Logbook',
    logbuchText:
      'What we build, test and learn this season: entries straight from the workshop and the competition.',
  },
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
