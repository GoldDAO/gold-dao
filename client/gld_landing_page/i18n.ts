import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'
import LanguageDetector from 'i18next-browser-languagedetector'
import HttpApi from 'i18next-http-backend'

i18n
  .use(HttpApi)
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    fallbackLng: 'en',
    supportedLngs: ['en'],
    debug: process.env.NODE_ENV === 'development',
    backend: {
      loadPath: '/locales/{{lng}}/{{ns}}.json'
    },
    ns: [
      'header',
      'hero',
      'tokens',
      'what',
      'goldDaoChart',
      'cards',
      'partners',
      'secure',
      'whitepaper',
      'footer'
    ],
    interpolation: {
      escapeValue: false
    },
    react: {
      useSuspense: true,
      transSupportBasicHtmlNodes: true,
      transKeepBasicHtmlNodesFor: ['a', 'b', 'strong', 'i', 'p']
    }
  })

export default i18n
