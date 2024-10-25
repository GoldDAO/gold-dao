/* eslint-disable @next/next/no-img-element */
'use client'

import { useTranslation } from 'react-i18next'
import Image from 'next/image'
import { useQuery } from '@tanstack/react-query'
import { useEffect, useRef, useState } from 'react'
import { useTokenMetrics } from '@/hooks/useTokenMetrics'

interface InfoCardProps {
  iconSrc: string
  iconAlt: string
  text: string
  value: string
  loading: boolean
}

const InfoCard = ({
  iconSrc,
  iconAlt,
  text,
  value,
  loading
}: InfoCardProps) => (
  <section className='flex h-10 px-4 pl-2 min-w-full md:min-w-0 justify-center items-center rounded-3xl border gap-[8px] mx-2 border-[#D3B872] bg-white'>
    {iconSrc && (
      <Image
        src={iconSrc}
        alt={iconAlt}
        width={24}
        height={24}
        className='w-[24px] h-[24px] flex-shrink-0'
      />
    )}
    <span className='font-inter font-normal leading-[16px] text-[#262C2E]'>
      {text}
    </span>
    <span className='h-full w-0.5 bg-[#D3B872] rounded-3xl mx-[8px]'></span>
    {loading ? (
      <span className='loading-skeleton'></span>
    ) : (
      <span className='font-bold'>{value}</span>
    )}
  </section>
)

const Hero = () => {
  const videoRef = useRef<HTMLVideoElement | null>(null)
  // const { data, isLoading, error } = useTokenMetrics()

  const { t } = useTranslation('hero')

  // const totalGoldLockedKg = data ? data.total_gold_kg.toFixed(2) : null
  // const marketCapUSD = data ? `${data.tvl.toLocaleString('en-US')}` : null

  return (
    <div className='h-[100vh] md:h-[75vh] w-full flex flex-col items-center justify-center px-2 md:px-10 '>
      <video
        ref={videoRef}
        autoPlay
        loop
        muted
        preload='auto'
        playsInline
        className={`absolute inset-0 w-full h-[100vh] md:h-3/4 object-cover transition-opacity duration-500`}
        src='https://daolink-gold-dao-website-medias.sos-ch-gva-2.exo.io/Gold_DAO_bg_video.mp4'
      />
      <div className='relative text-center'>
        <h1
          className='text-[53px] md:text-[82px] font-inter font-bold text-white leading-[90px] text-shadow-lg md:max-w-2xl mx-auto'
          style={{
            textShadow:
              '0px 10px 15px rgba(0, 0, 0, 0.10), 0px 4px 6px rgba(0, 0, 0, 0.05)'
          }}
        >
          {t('title')}
        </h1>
        <p
          className='text-[40px] md:text-[46px] font-inter font-light leading-[90px] text-[rgba(0,0,0,0.80)]  w-full mx-auto  md:max-w-6xl'
          style={{
            textShadow:
              '0px 10px 15px rgba(0, 0, 0, 0.10), 0px 4px 6px rgba(0, 0, 0, 0.05)'
          }}
        >
          {t('subtitle')}
        </p>
        {/* <div className='mt-10 sm:mt-[64px] flex-col space-y-6 xl:space-y-0 xl:w-full flex lg:flex-row justify-around items-center'>
          <InfoCard
            iconSrc='/static/icons/Gold-Light-1g.svg'
            iconAlt='Total Gold Icon'
            text={t('total_gold_locked')}
            value={`${totalGoldLockedKg} kg`}
            loading={isLoading}
          />
          <InfoCard
            iconSrc='/static/icons/Gold-Marketcap.svg'
            iconAlt='Marketcap Icon'
            text={t('gldt_marketcap')}
            value={`$${marketCapUSD}`}
            loading={isLoading}
          />
        </div> */}
      </div>
    </div>
  )
}

export default Hero
