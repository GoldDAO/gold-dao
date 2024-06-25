'use client';

import Image from 'next/image';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

export default function Navbar({ title }) {
  const pathName = usePathname();

  const items = [
    {
      name: 'home',
      src: 'svg/graph.svg',
      href: '/',
      disabled: false,
    },
    {
      name: 'rewards',
      src: 'svg/wallet.svg',
      href: '/rewards',
      disabled: false,
    },
    // { name: "search", src: "svg/search.svg", href: "/#", disabled: true },
    {
      name: 'about',
      src: 'svg/about.svg',
      href: 'https://docs.gold-dao.org',
      disabled: false,
      blank: true,
    },
  ];

  return (
    <aside
      className={`bg-[#F3F3F3] w-full sm:w-20 h-22 sm:h-screen flex flex-col sm:justify-start items-center fixed sm:static z-20 sm:z-auto bottom-0 sm:pt-10 border-t border-[#C6C6C6] ${['transfer', 'icp neurons', 'confirm claim', 'add a neuron to the dashboard', 'remove neuron', 'add neuron'].includes(title?.toLowerCase()) ? 'sm:hidden' : ''}`}
    >
      <Link href={'/'}>
        <Image
          src="svg/logo.svg"
          alt="Gold Dao"
          width={49}
          height={70}
          className="hidden sm:block"
        />
      </Link>
      <section className="mt-1 sm:mt-28 flex sm:flex-col w-full sm:w-fit px-9 sm:px-0 justify-around sm:gap-2 py-5 sm:pt-0 cursor-pointer">
        {items.map(({
          name, src, href, disabled, blank,
        }) => (disabled ? (
          <div
            className={`flex justify-center items-center rounded-full size-12 ${pathName === href ? 'bg-[#D3B871]' : 'bg-white'} ${disabled ? 'opacity-50 cursor-not-allowed' : ''}`}
            key={name}
          >
            <Image
              src={src}
              alt={name}
              width={20}
              height={20}
              className={`${pathName === href ? 'invert' : 'invert-0'} size-5`}
            />
          </div>
        ) : (
          <Link
            className={`flex justify-center items-center rounded-full size-12 ${pathName === href ? 'bg-[#D3B871]' : 'bg-white'} ${disabled ? 'opacity-50 cursor-not-allowed' : ''}`}
            href={href}
            key={name}
            target={blank ? '_blank' : '_self'}
            rel={blank ? 'noopener noreferrer' : ''}
          >
            <Image
              src={src}
              alt={name}
              width={20}
              height={20}
              className={`${pathName === href ? 'invert' : 'invert-0'} size-5`}
            />
          </Link>
        )))}
      </section>
    </aside>
  );
}
