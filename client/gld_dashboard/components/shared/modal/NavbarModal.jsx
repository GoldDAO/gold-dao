import Image from 'next/image';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

export default function NavbarModal() {
  const pathName = usePathname();

  return (
    <div className="fixed bottom-[16%] sm:hidden bg-SoftGrey w-full h-22 flex px-9 py-5 items-center justify-around border-t-[0.5px] border-DarkGrey">
      <Link
        href="/"
        className={`flex justify-center items-center rounded-full size-12 ${pathName === '/' ? 'bg-[#D3B871]' : 'bg-white'}`}
      >
        <Image
          src="svg/graph.svg"
          alt="Grapgh Icon"
          width={20}
          height={20}
          className={`${pathName === '/' ? 'invert' : 'invert-0'} size-5`}
        />
      </Link>
      <Link
        href={'/rewards'}
        className={`flex justify-center items-center rounded-full size-12 ${pathName === '/rewards' ? 'bg-[#D3B871]' : 'bg-white'}`}
      >
        <Image
          src="svg/wallet.svg"
          alt="Wallet Icon"
          width={20}
          height={20}
          className={`${pathName === '/rewards' ? 'invert' : 'invert-0'} size-5`}
        />
      </Link>
      <Link
        href={'https://docs.gold-dao.org'}
        className="flex justify-center items-center rounded-full size-12 bg-white"
        target={'_blank'}
        rel={'noopener noreferrer'}
      >
        <Image src="svg/about.svg" alt="About Icon" width={20} height={20} />
      </Link>
    </div>
  );
}
