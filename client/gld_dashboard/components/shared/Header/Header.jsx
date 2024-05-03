'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import LoginButton from './LoginButton.jsx';
import { currentPage } from '../../../utils/parsers';

export default function Header() {
  const path = usePathname();

  return (
    <header className="flex justify-between sm:p-5 sm:mb-11">
      <div className="flex w-2/3 sm:w-auto text-xl sm:block justify-start flex-wrap items-center gap-1 leading-none sm:leading-[64px]">
        <Link href={'/'}>
          <h1 className="text-[#D3B871] sm:text-7xl font-bold sm:block">Gold DAO</h1>
        </Link>
        <h2 className="text-DarkGrey sm:text-7xl font-bold sm:block ">{currentPage(path)}</h2>
      </div>
      <section className="flex sm:w-auto flex-col justify-between py-4">
        <LoginButton />
      </section>
    </header>
  );
}
