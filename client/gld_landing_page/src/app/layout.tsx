import type { Metadata } from 'next'
import { ogImage as image, siteName as title, siteUrl } from '@/site'
import './globals.css'

const description =
  'Gold DAO holds a treasury of staked ICP, OGY and WTN on the Internet Computer. GOLDAO stakers vote on what happens with it and receive a share of what it earns.'

// The home page. The docs pages set their own in src/docs/metadata.ts.
export const metadata: Metadata = {
  metadataBase: new URL(siteUrl),
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
