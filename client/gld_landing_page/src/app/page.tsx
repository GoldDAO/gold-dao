import { Metadata } from 'next'
import ClientSidePage from '../components/ClientSidePage'

export async function generateStaticParams () {
  const locales = ['en']
  return locales.map(locale => ({ locale }))
}

export const metadata: Metadata = {
  title: 'The Gold DAO Website',
  description:
    'Gold is a timeless and trusted asset. Gold DAO enables you to directly own and trade physical gold, using cutting-edge blockchain technology for unrivalled transparency, security, and ease.',
  openGraph: {
    type: 'website',
    url: 'https://www.gold-dao.org/',
    title: 'The Gold DAO Website',
    description:
      'Gold is a timeless and trusted asset. Gold DAO enables you to directly own and trade physical gold, using cutting-edge blockchain technology for unrivalled transparency, security, and ease.',
    images: [
      {
        url: 'https://www.gold-dao.org/static/illustrations/gold_dao.png',
        width: 800,
        height: 600,
        alt: 'The Gold DAO Website'
      }
    ]
  },

  twitter: {
    card: 'summary_large_image',
    title: 'The Gold DAO Website',
    description:
      'Gold is a timeless and trusted asset. Gold DAO enables you to directly own and trade physical gold, using cutting-edge blockchain technology for unrivalled transparency, security, and ease.',
    images: ['https://www.gold-dao.org/static/illustrations/gold_dao.png']
  }
}

export default function Page ({ params }: { params: { locale: string } }) {
  return <ClientSidePage params={params} />
}
