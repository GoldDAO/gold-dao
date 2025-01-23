/* eslint-disable @next/next/no-img-element */
'use client'

import { useTranslation } from 'react-i18next'
import Image from 'next/image'

const TokensInfos: React.FC = () => {
  const { t } = useTranslation('tokens')

  return (
    <section className='flex flex-col items-center justify-center gap-4 w-full mb-[96px] px-2 4xl:max-w-screen-3xl'>
      <div className='my-[96px]  font-inter leading-[36px] text-center px-5 md:px-0 md:max-w-2xl lg:max-w-6xl  text-[#000000CC]'>
        <p className='font-semibold text-[30px]'>{t('subtitle')}</p>
        <p className='font-light pt-4 text-[25px]'>{t('description')}</p>
      </div>
      <div className='lg:max-w-7xl xl:max-w-[1289px] 2xl:max-w-[91rem] 3xl:max-w-[90rem]'>
        <CardGrid />
      </div>
    </section>
  )
}

interface CardProps {
  logoSrc: string
  title: string
  description: string
  learnMoreLink: string
  learnMoreButton: string
  button: string
  buyLink?: string
  imageSrc: string
}

const Card: React.FC<CardProps> = ({
  logoSrc,
  title,
  description,
  learnMoreLink,
  learnMoreButton,
  button,
  buyLink,
  imageSrc
}) => {
  const { t } = useTranslation('tokens')

  return (
    <div className='shadow-lg bg-white rounded-[20px]'>
      <div className='px-2 p-2'>
        <div className='p-4 space-y-8'>
          <div className='flex flex-row items-center align-middle space-x-4 mt-6'>
            <Image
              src={logoSrc}
              alt={title}
              className='w-[40px] h-[40px]'
              width={40}
              height={40}
            />
            <div className='font-semibold text-[36px]'>{title}</div>
          </div>
          <p className='text-gray-700 text-base h-[72px]'>{description}</p>
          <div className='flex flex-col space-y-6'>
            {learnMoreLink ? (
              <a
                href={learnMoreLink}
                target='_blank'
                className='text-[#000000A3] underline underline-offset-[3px] hover:text-secondary duration-300 ease-in-out'
              >
                {learnMoreButton}
              </a>
            ) : (
              <p className='text-[#000000A3] underline underline-offset-[3px]'>
                {learnMoreButton}
              </p>
            )}

            {buyLink ? (
              <a href={buyLink} target='_blank' className='main-button w-fit'>
                {button}
              </a>
            ) : (
              <button className='main-button-disabled w-fit cursor-default'>
                {button}
              </button>
            )}
          </div>
        </div>
        <img
          className='min-h-[215px] object-cover rounded-[20px] mt-8'
          src={imageSrc}
          alt={title}
        />
      </div>
    </div>
  )
}

const CardGrid: React.FC = () => {
  const { t } = useTranslation('tokens')

  const cards = [
    {
      logoSrc: '/static/icons/Gold-Gov.svg',
      title: t('tokens.goldao.title'),
      description: t('tokens.goldao.description'),
      learnMoreButton: t('tokens.goldao.learnMoreButton'),
      learnMoreLink: 'https://docs.gold-dao.org/',
      buyLink:
        'https://info.icpswap.com/swap/token/details/tyyy3-4aaaa-aaaaq-aab7a-cai',
      imageSrc: '/static/illustrations/Gold-gov.png',
      button: t('tokens.goldao.button')
    },
    {
      logoSrc: '/static/icons/Gold-NFT.svg',
      title: t('tokens.gld_nft.title'),
      description: t('tokens.gld_nft.description'),
      learnMoreButton: t('tokens.gld_nft.learnMoreButton'),
      learnMoreLink: 'https://gold.bity.com/en/about',
      buyLink: 'https://gold.bity.com/',
      imageSrc: '/static/illustrations/Gold-NFT.png',
      button: t('tokens.gld_nft.button')
    },
    {
      logoSrc: '/static/icons/Gold-GLDT.svg',
      title: t('tokens.gldt.title'),
      description: t('tokens.gldt.description'),
      learnMoreButton: t('tokens.gldt.learnMoreButton'),
      learnMoreLink: 'https://gldt.org/',
      buyLink: 'https://gldt.org/swap',
      imageSrc: '/static/illustrations/Gold-GLDT.png',
      button: t('tokens.gldt.button')
    },
    {
      logoSrc: '/static/icons/Gold-USDG.svg',
      title: t('tokens.usdg.title'),
      description: t('tokens.usdg.description'),
      learnMoreButton: t('tokens.usdg.learnMoreButton'),
      learnMoreLink:
        'https://docs.gold-dao.org/gold-dao-whitepaper/the-ecosystem/products/usd-pegged-stablecoin-usdg',
      buyLink: '',
      imageSrc: '/static/illustrations/Gold-USDG.png',
      button: t('tokens.usdg.button')
    }
  ]

  return (
    <div className='grid grid-cols-1 gap-[40px] md:grid-cols-2 xl:grid-cols-4 lg:gap-[10px] md:px-[22px] xl:px-0'>
      {cards.map((card, index) => (
        <Card key={index} {...card} />
      ))}
    </div>
  )
}

export default TokensInfos
