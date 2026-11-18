import type { Metadata } from 'next'
import './globals.css'

const title = 'Gold DAO'
const description =
  'Gold DAO holds a treasury of staked ICP, OGY and WTN on the Internet Computer. GOLDAO stakers vote on what happens with it and receive a share of what it earns.'
// public/og.png, 1200 x 630
const image = {
  url: '/og.png',
  width: 1200,
  height: 630,
  alt: 'Gold DAO. Stake GOLDAO. Govern the treasury.'
}

export const metadata: Metadata = {
  metadataBase: new URL('https://www.gold-dao.org'),
  title,
  description,
  alternates: { canonical: '/' },
  icons: { icon: '/favicon.ico' },
  openGraph: {
    type: 'website',
    url: '/',
    siteName: title,
    title,
    description,
    images: [image]
  },
  twitter: {
    card: 'summary_large_image',
    title,
    description,
    images: [image]
  }
}

export default function RootLayout ({
  children
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang='en'>
      <head>
        {/* Polymath, the typeface of the Gold DAO dApp, from the same Adobe Fonts kit */}
        <link rel='preconnect' href='https://use.typekit.net' crossOrigin='' />
        <link rel='stylesheet' href='https://use.typekit.net/chv2tas.css' />
      </head>
      <body>{children}</body>
    </html>
  )
}
