'use client'
import { useTranslation } from 'react-i18next'
import Image from 'next/image'
const Footer = () => {
  const { t } = useTranslation('footer')

  const links = [
    {
      name: 'X',
      href: 'https://x.com/thegolddao',
      iconSrc: '/static/icons/gold-menu-x.svg',
      iconAlt: 'X'
    },
    {
      name: 'Telegram',
      href: 'https://t.me/gldrwa',
      iconSrc: '/static/icons/gold-menu-telegram.svg',
      iconAlt: 'Telegram'
    },
    {
      name: 'LinkedIn',
      href: 'https://www.linkedin.com/showcase/gold-dao/',
      iconSrc: '/static/icons/gold-menu-linkedin.svg',
      iconAlt: 'LinkedIn'
    },

    {
      name: 'Medium',
      href: 'https://medium.com/@GoldDAO',
      iconSrc: '/static/icons/gold-menu-medium.svg',
      iconAlt: 'Medium'
    }
  ]

  return (
    <footer className='flex flex-row w-full bg-black py-2 px-6 md:px-16 h-[50px] items-center'>
      <p className='text-xs font-normal text-white '>{t('from')}</p>
      <div className='flex flex-row gap-4 items-center ml-auto'>
        {links.map(link => (
          <a
            href={link.href}
            target='_blank'
            rel='noreferrer'
            key={link.name}
            className='flex flex-row items-center gap-2 hover:opacity-80 duration-200 ease-in-out '
          >
            <Image
              src={link.iconSrc}
              alt={link.iconAlt}
              width={30}
              height={30}
            />
          </a>
        ))}
      </div>
    </footer>
  )
}

export default Footer
