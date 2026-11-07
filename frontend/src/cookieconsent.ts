import type { CookieConsentConfig } from 'vanilla-cookieconsent'

// Necessary cookies only (auth_token); texts unchanged from the old main.ts
export const consentOptions: CookieConsentConfig = {
  root: 'body',
  autoShow: true,
  disablePageInteraction: false,
  revision: 0,
  cookie: {
    name: 'cookie-consent',
  },
  guiOptions: {
    consentModal: {
      layout: 'box',
      position: 'bottom right',
      equalWeightButtons: true,
      flipButtons: false,
    },
    preferencesModal: {
      layout: 'box',
      equalWeightButtons: true,
      flipButtons: false,
    },
  },

  categories: {
    necessary: {
      enabled: true,
      readOnly: true,
    },
  },

  language: {
    default: 'de',
    translations: {
      de: {
        consentModal: {
          title: 'Wir nutzen Cookies',
          description:
            'Wir setzen ausschließlich technisch notwendige Cookies ein. Sie sichern Login‑/Sitzungs\-Funktionen, schützen vor Missbrauch und ermöglichen das Verwalten unserer Posts. Keine Tracking- oder Marketing-Cookies.',
          //acceptAllBtn: 'Accept all',
          acceptNecessaryBtn: 'Notwendige akzeptieren',
          showPreferencesBtn: 'Mehr Informationen',
          footer: `
            <a href="/imprint" target="_blank">Impressum</a>
            <a href="/privacy-policy" target="_blank">Datenschutzerklärung</a>
          `,
        },
        preferencesModal: {
          title: 'Cookie Einstellungen verwalten',
          //acceptAllBtn: 'Accept all',
          acceptNecessaryBtn: 'Notwendige akzeptieren',
          //savePreferencesBtn: 'Accept current selection',
          closeIconLabel: 'Schließen',
          sections: [
            {
              title: 'Wir verwenden nur notwendige Cookies',
              description:
                'Diese Cookies sind erforderlich, damit Kernfunktionen (Session, Authentifizierung, CSRF-/Missbrauchsschutz) zuverlässig arbeiten. Ohne sie funktioniert diese Seite nicht korrekt.',
            },
            {
              title: 'Kontoverwaltung',
              description:
                'Sitzungs- und Verifizierungstoken halten Sie angemeldet und schützen unsere Inhalte.',
              linkedCategory: 'necessary',
            },
            {
              title: 'Datenschutzerklärung',
              description:
                'Wir speichern nur minimale, zweckgebundene Daten zur Sicherstellung des Betriebs. Keine Profilbildung, kein Tracking.',
              linkedCategory: 'necessary',
            },
            {
              title: 'Mehr Informationen',
              description:
                'Bei Fragen zu unseren Cookie-Richtlinien oder zu Ihren Einstellungen <a href="/Imprint">kontaktieren</a> Sie uns',
            },
          ],
        },
      },
    },
  },
}
