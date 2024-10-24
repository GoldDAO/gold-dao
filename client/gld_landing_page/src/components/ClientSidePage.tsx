'use client'
import dynamic from 'next/dynamic'
const Header = dynamic(() => import('@/components/Header'), { ssr: false })
const Hero = dynamic(() => import('@/components/Hero'), { ssr: false })
const TokensInfos = dynamic(() => import('@/components/TokensInfos'), {
  ssr: false
})
const WhatInfos = dynamic(() => import('@/components/WhatInfos'), {
  ssr: false
})
const GoldDAOChart = dynamic(() => import('@/components/GoldDAOChart'), {
  ssr: false
})
const TokensCards = dynamic(() => import('@/components/TokensCards'), {
  ssr: false
})
const Partners = dynamic(() => import('@/components/Partners'), { ssr: false })
const Governance = dynamic(() => import('@/components/Secure'), {
  ssr: false
})
const WhitePaper = dynamic(() => import('@/components/WhitePaper'), {
  ssr: false
})
const Footer = dynamic(() => import('@/components/Footer'), { ssr: false })

export default function Home ({ params }: { params: { locale: string } }) {
  return (
    <main className='flex flex-col items-center'>
      <Header />
      <Hero />
      <TokensInfos />
      <WhatInfos />
      <GoldDAOChart />
      <TokensCards />
      <Partners />
      <Governance />
      <WhitePaper />
      <Footer />
    </main>
  )
}
