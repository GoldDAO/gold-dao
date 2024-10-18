'use client';

import Image from 'next/image';
import Link from 'next/link';
import { FooterGoldDaoIcon } from '../../utils/svgs';

export default function Footer() {
  const footerInfo = [
    {
      type: true,
      title: 'Gold DAO Dashboard',
      content: [
        { text: 'Home', link: '/' },
        { text: 'My rewards', link: '/rewards' },
      ],
    },
    {
      type: false,

      title: 'Token',
      content: [
        // { text: "GLDGov", link: "https://www.gold-dao.org/" },
        { text: 'GLD NFT', link: 'https://6mrot-oaaaa-aaaal-qdfna-cai.icp0.io/gold' },
        { text: 'GLDT', link: 'https://gldt.org/' },
        // { text: "GLDT Swap", link: "#" },
        { text: 'OGY', link: 'https://dashboard.origyn.com/' },
        { text: 'ICP', link: 'https://dashboard.internetcomputer.org/' },
      ],
    },
    {
      type: false,

      title: 'Support',
      content: [
        { text: 'Gold DAO Website', link: 'https://www.gold-dao.org/' },
        { text: 'GitBook', link: 'https://docs.gold-dao.org' },
      ],
    },
    {
      type: false,

      title: 'Follow Us',
      content: [
        { text: 'X', link: 'https://twitter.com/gldrwa', icon: 'svg/x.svg' },
        {
          text: 'Linkedin',
          link: 'https://www.linkedin.com/showcase/gold-dao/',
          icon: 'svg/linkedin.svg',
        },
        { text: 'Medium', link: 'https://medium.com/@GoldDAO', icon: 'svg/medium.svg' },
        { text: 'Telegram', link: 'https://t.me/TheGoldDAO', icon: 'svg/telegram.svg' },
        { text: 'OpenChat', link: 'https://oc.app/community/qi5v6-wiaaa-aaaar-axvia-cai/?ref=g5b5b-zaaaa-aaaar-bdivq-cai', icon: 'svg/openchat.svg' },
      ],
    },
  ];

  return (
    <footer className="bg-[#FBFBFB] w-full pt-20 pb-8 sm:pr-4 mb-20 sm:mb-0 sm:pl-[30px] px-5">
      <section className="w-full mx-auto flex flex-col lg:px-20 ">
        <div className="sm:flex block gap-10">
          <div className="w-[100%] sm:w-[40%] mb-10">
            <div className="flex items-center justify-start sm:px-2">
              <Link href={'/'} className='hidden sm:block'>
                <FooterGoldDaoIcon w={188} h={60} />
              </Link>
              <Link href={'/'} className='sm:hidden'>
                <FooterGoldDaoIcon w={149} h={47} />
              </Link>
            </div>
          </div>

          <div className="w-full sm:w-60vw md:w-[100%] ">
            <div className="grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-4 xl:grid-cols-4 w-full justify-end gap-5">
              {footerInfo.map((item, index) => (
                <div key={index} className="sm:flex sm:justify-start block ">
                  <div className="px-0 pb-4">
                    <p className="font-bold  text-lg sm:text-[16px]"> {item.title}</p>
                    <div className=" mt-0 sm:mt-5 text-xs sm:text-[14px]  flex flex-col justify-center gap-0 sm:gap-2">
                      {item.content.map((e, i) => (
                        <div key={i}>
                          {item.type === false ? (
                            <a href={e.link} target="_blank" rel="noopener noreferrer">
                              <div className="flex justify-start items-center gap-2">
                                {' '}
                                {e.icon && (<Image
                                  className={`${e.icon ? '' : 'hidden'}`}
                                  src={e.icon}
                                  height={20}
                                  width={20}
                                  alt="gold dao"
                                />)}
                                {' '}
                                <p className="py-2 text-xs sm:text-[14px]"> {e.text} </p>
                              </div>
                            </a>
                          ) : (
                            <Link href={e.link}>
                              <div className="flex justify-start items-center gap-2">
                                {' '}
                                { e.icon && (<Image
                                  className={`${e.icon ? '' : 'hidden'}`}
                                  src={e.icon}
                                  height={20}
                                  width={20}
                                  alt="gold dao"
                                />)}
                                {' '}
                                <p className="py-2"> {e.text} </p>
                              </div>
                            </Link>
                          )}
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className="flex justify-start mt-5">
          <div className="border-t-2 flex justify-start w-full pt-5">
            <div className="w-full text-xs">
              <p>© 2024 Gold DAO. All rights reserved.</p>
            </div>
          </div>
        </div>
      </section>
    </footer>
  );
}
