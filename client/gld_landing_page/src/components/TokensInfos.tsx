'use client'

import { useTranslation } from 'react-i18next'
import Image from 'next/image'

const TokensInfos: React.FC = () => {
  const { t } = useTranslation('tokens')

  return (
    <section className='flex flex-col items-center justify-center gap-4 w-full mb-[96px] px-2 4xl:max-w-screen-3xl'>
      <p className='text-[30px] font-inter font-light leading-[36px] text-center px-5 md:px-0 lg:max-w-4xl my-[96px] text-[#000000CC]'>
        {t('description')}
      </p>
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
  buyLink?: string
  imageSrc: string
}

const Card: React.FC<CardProps> = ({
  logoSrc,
  title,
  description,
  learnMoreLink,
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
                {t('learn_more')}
              </a>
            ) : (
              <p className='text-[#000000A3] underline underline-offset-[3px]'>
                {t('learn_more')}
              </p>
            )}

            {buyLink ? (
              <a href={buyLink} target='_blank' className='main-button w-fit'>
                {t('buy')} {title}
              </a>
            ) : (
              <button className='main-button-disabled w-fit cursor-default'>
                {t('coming_soon')}
              </button>
            )}
          </div>
        </div>
        <Image
          className='w-full min-h-[215px] object-cover rounded-[20px] mt-8'
          src={imageSrc}
          alt={title}
          width={40}
          height={40}
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
      title: t('tokens.gldgov.title'),
      description: t('tokens.gldgov.description'),
      learnMoreLink: 'https://docs.gold-dao.org/',
      buyLink:
        'https://info.icpswap.com/swap/token/details/tyyy3-4aaaa-aaaaq-aab7a-cai',
      imageSrc: '/static/illustrations/Gold-Gov.svg'
    },
    {
      logoSrc: '/static/icons/Gold-NFT.svg',
      title: t('tokens.gld_nft.title'),
      description: t('tokens.gld_nft.description'),
      learnMoreLink: 'https://yumi.io/gold/about',
      buyLink: 'https://yumi.io/gold',
      imageSrc: '/static/illustrations/gold-light-nft.svg'
    },
    {
      logoSrc: '/static/icons/Gold-GLDT.svg',
      title: t('tokens.gldt.title'),
      description: t('tokens.gldt.description'),
      learnMoreLink: 'https://gldt.org/',
      buyLink: 'https://gldt.org/',
      imageSrc: '/static/illustrations/Gold-GLDT.svg'
    },
    {
      logoSrc: '/static/icons/Gold-USDG.svg',
      title: t('tokens.usdg.title'),
      description: t('tokens.usdg.description'),
      learnMoreLink: '',
      imageSrc: '/static/illustrations/Gold-USDG.svg'
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
