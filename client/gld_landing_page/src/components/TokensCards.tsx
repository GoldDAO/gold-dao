/* eslint-disable @next/next/no-img-element */
/* TokensCards.tsx */
'use client'

import Image from 'next/image'
import { useState, useCallback } from 'react'
import { useTranslation } from 'react-i18next'
import LearnMoreModal from './LearnMoreModal'

interface Card {
  title: string
  subtitle: string
  tag: string
  description: string
  imageSrc?: string
  videoSrc?: string
  points: string[]
  cardKey: 'goldao' | 'gld_nft' | 'gldt' | 'usdg'
}

const TokensCards = () => {
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [selectedCardKey, setSelectedCardKey] = useState<
    'goldao' | 'gld_nft' | 'gldt' | 'usdg'
  >('goldao')

  const openModal = useCallback(
    (cardKey: 'goldao' | 'gld_nft' | 'gldt' | 'usdg') => {
      setSelectedCardKey(cardKey)
      setIsModalOpen(true)
    },
    []
  )

  const closeModal = useCallback(() => {
    setIsModalOpen(false)
  }, [])

  const { t } = useTranslation('cards')
  const { t: tLearnModal } = useTranslation('learnMoreModal')

  const cards: Card[] = t('cards', { returnObjects: true }) as Card[]

  return (
    <section className='flex flex-col items-center justify-center gap-[24rem] md:gap-[14rem] px-[7px] md:w-[calc(100%-45px)] pt-[96px] 3xl:max-w-[90rem] mb-[96px] bg-[#FBF8F1]'>
      {cards &&
        cards.map((card, index) => (
          <div
            key={index}
            className='card shadow-lg bg-white rounded-[20px] flex flex-col md:flex-row items-center h-[90vh] max-h-[90vh] md:max-h-[612px] w-full'
          >
            <div className='flex flex-col md:w-1/2 p-8 md:py-16 md:px-8 xl:px-16 2xl:px-32'>
              <div className='bg-[#F7EED7] text-[#B89143] hidden md:inline rounded-full w-fit px-4 py-1 mb-4'>
                {card.tag}
              </div>
              <div className='flex flex-row items-center mb-4 pt-8 md:pt-0 xl:pt-8'>
                <div className='font-semibold text-3xl '>{card.title}</div>
                <div className='bg-[#F7EED7] text-[#B89143] rounded-full w-fit px-4 py-1 md:hidden ml-6'>
                  {card.tag}
                </div>
              </div>
              <p className='text-black/80 text-xl font-bold py-2'>
                {card.subtitle}
              </p>
              <p className='text-base mb-4'>{card.description}</p>

              <button
                onClick={() => openModal(card.cardKey)}
                className='main-button w-fit mt-4'
              >
                Learn More
              </button>
            </div>

            <div className='flex-1 relative w-full md:w-1/2 h-full flex items-stretch overflow-hidden rounded-b-[20px] md:rounded-r-[20px] md:rounded-br-[20px] md:rounded-bl-[0px]'>
              {card.videoSrc && (
                <video
                  src={card.videoSrc}
                  autoPlay
                  loop
                  muted
                  playsInline
                  className='rounded-b-[20px] md:rounded-r-[20px] md:rounded-b-none object-cover w-full h-full'
                  controlsList='nodownload'
                />
              )}
              {card.imageSrc && (
                <img
                  src={card.imageSrc}
                  alt={card.title}
                  className='object-cover w-full h-full'
                  sizes='(max-width: 768px) 100vw, 50vw'
                />
              )}
            </div>
          </div>
        ))}
      {isModalOpen && (
        <LearnMoreModal
          onClose={closeModal}
          cardKey={selectedCardKey}
          t={tLearnModal}
        />
      )}
    </section>
  )
}

export default TokensCards
