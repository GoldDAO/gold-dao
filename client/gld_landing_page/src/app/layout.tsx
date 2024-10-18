/* eslint-disable react/jsx-no-undef */
'use client'
import { Inter } from 'next/font/google'
import { I18nextProvider } from 'react-i18next'
import i18n from '../../i18n'
import './globals.css'
import ReactQueryProvider from '@/providers/ReactQueryProvider'

const inter = Inter({ subsets: ['latin'] })

export default function RootLayout ({
  children,
  params
}: Readonly<{
  children: React.ReactNode
  params: { locale: string }
}>) {
  return (
    <I18nextProvider i18n={i18n}>
      <html lang={'en'}>
        <body className={` ${inter.className} bg-[#FAF9F8]`}>
          <ReactQueryProvider>{children}</ReactQueryProvider>
        </body>
      </html>
    </I18nextProvider>
  )
}
