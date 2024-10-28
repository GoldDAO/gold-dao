/* eslint-disable @next/next/no-img-element */
import React, { useEffect, useRef } from 'react'
import {
  disableBodyScroll,
  enableBodyScroll,
  clearAllBodyScrollLocks
} from 'body-scroll-lock'
import { TFunction } from 'i18next'

interface LearnMoreModalProps {
  onClose: () => void
  cardKey: 'gldgov' | 'gld_nft' | 'gldt' | 'usdg' | ''
  t: TFunction
}

const LearnMoreModal = ({ onClose, cardKey, t }: LearnMoreModalProps) => {
  const modalRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const modalElement = modalRef.current
    if (modalElement) {
      disableBodyScroll(modalElement)
    }

    return () => {
      if (modalElement) {
        enableBodyScroll(modalElement)
      }
      clearAllBodyScrollLocks()
    }
  }, [cardKey, t])

  const handleOutsideClick = (
    e: React.MouseEvent<HTMLDivElement, MouseEvent>
  ) => {
    if (e.target === e.currentTarget) onClose()
  }

  const points = t(`${cardKey}.points`, { returnObjects: true })
  const description1 = t(`${cardKey}.description1`)
  const description2 = t(`${cardKey}.description2`, { defaultValue: '' })
  const bottomDescription = t(`${cardKey}.bottomDescription`)
  const hasLink = t(`${cardKey}.learnMoreLink`, { defaultValue: '' })

  return (
    <div
      className='fixed inset-0 flex items-center justify-center bg-[#212425] bg-opacity-70 z-50'
      onClick={handleOutsideClick}
    >
      <div className='bg-white rounded-2xl px-6 py-6 md:px-20 md:pb-20 xl:pb-10 2xl:pb-20 md:pt-10 w-[95%] lg:w-3/4 2xl:w-3/4 3xl:w-1/2 4xl:w-1/3 shadow-lg relative space-y-4 max-h-[95vh] overflow-y-auto'>
        <button
          className='absolute top-4 right-4 text-gray-400 text-2xl hover:text-gray-600'
          onClick={onClose}
        >
          &times;
        </button>

        <div className='flex items-center space-x-4'>
          <img
            src={t(`${cardKey}.logoSrc`)}
            alt={t(`${cardKey}.title`)}
            className='w-20 h-20 object-contain'
          />
          <div className='flex flex-col md:flex-row md:items-center md:w-full'>
            <h2 className='text-[#d3b872] text-4xl md:text-5xl font-bold'>
              {t(`${cardKey}.title`)}
            </h2>
            <span className='text-sm text-[#B89143] bg-[#F7EED7] px-4 py-1 w-fit rounded-full md:ml-auto'>
              {t(`${cardKey}.tag`)}
            </span>
          </div>
        </div>

        <p className='text-black/80 text-xl font-bold pt-4'>
          {t(`${cardKey}.subtitle`)}
        </p>

        {description1 && (
          <p className='text-base text-gray-700'>{description1}</p>
        )}

        {description2 && (
          <p className='text-base text-gray-700'>{description2}</p>
        )}

        <div className='space-y-2'>
          {Array.isArray(points) &&
            points.map((point, index) => (
              <div key={index} className='text-base text-gray-700'>
                <span>
                  {index + 1}. {point}
                </span>
              </div>
            ))}
        </div>

        {bottomDescription && (
          <p className='text-base text-gray-700 pb-4'>{bottomDescription}</p>
        )}
        {hasLink && (
          <a
            href={t(`${cardKey}.learnMoreLink`)}
            target='_blank'
            rel='noopener noreferrer'
            className='main-button w-fit'
          >
            {t(`${cardKey}.button`)}
          </a>
        )}
        {!hasLink && (
          <button className='main-button-disabled w-fit cursor-default'>
            Coming soon
          </button>
        )}
      </div>
    </div>
  )
}

export default LearnMoreModal
