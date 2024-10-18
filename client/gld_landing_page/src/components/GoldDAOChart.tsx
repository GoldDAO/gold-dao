'use client'

import Image from 'next/image'
import { useTranslation } from 'react-i18next'

const GoldDAOChart = () => {
  const { t } = useTranslation('goldDaoChart')

  return (
    <section
      style={{
        backgroundImage: "url('/static/backgrounds/bar_texture_full.svg')"
      }}
      className='relative w-full h-full flex flex-col items-center px-[6px] md:px-[27px] justify-center bg-cover bg-center pb-[10rem] '
    >
      <div className='text-3xl font-bold py-24'>{t('ecosystemTitle')}</div>
      <div className='border border-[#E8E6E1] md:w-full rounded-xl xl:w-3/4 p-2 md:p-4 items-center 4xl:max-w-screen-2xl justify-center bg-white md:max-h-[538px] xl:h-[450px] 2xl:h-[500px] 3xl:h-[538px]'>
        <Image
          src='/static/illustrations/GoldDAO-Ecosystem.svg'
          alt={t('ecosystemAlt')}
          width={1200}
          height={500}
          className='mx-auto self-center pt-4 hidden md:block'
        />
        <Image
          src='/static/illustrations/GoldDAO-chart-mobile.svg'
          alt={t('ecosystemAlt')}
          width={480}
          height={1200}
          className='mx-auto self-center pt-4 block md:hidden'
        />
        <div className='flex flex-row md:flex-row items-center justify-around pt-4 md:pt-0 md:space-y-0'>
          <div className='absolute bottom-[9%] md:bottom-[12%] left-1/2 transform -translate-x-1/2 flex w-1/2 sm:w-2/3 md:w-1/4 lg:w-1/5 bg-white rounded-[20px] border-[#E8E6E1] border shadow-xl px-4 py-4 flex-col justify-center items-center gap-1 flex-shrink-0'>
            <div className='flex flex-col items-center justify-center'>
              <p className='text-xs md:text-sm text-[#0000007A]'>
                {t('poweredBy')}
              </p>
              <div className='flex items-center gap-2'>
                <Image
                  src='/static/icons/OGY.svg'
                  alt='OGY Icon'
                  width={24}
                  height={24}
                />
                <span className='font-bold text-xl md:text-2xl pt-1'>OGY</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}

export default GoldDAOChart
