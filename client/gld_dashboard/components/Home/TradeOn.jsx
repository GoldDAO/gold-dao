import Image from 'next/image';
import Link from 'next/link';

export default function TradeOn() {
  const data = [
    {
      title: 'ICP SWAP',
      img: 'svg/icp-logo.svg',
      link: 'https://app.icpswap.com/swap?input=ryjl3-tyaaa-aaaaa-aaaba-cai&output=tyyy3-4aaaa-aaaaq-aab7a-cai',
    },
    {
      title: 'SONIC',
      img: 'svg/sonic-logo.svg',
      link: 'https://app.sonic.ooo/swap?from=ryjl3-tyaaa-aaaaa-aaaba-cai&to=tyyy3-4aaaa-aaaaq-aab7a-cai',
    },
    { title: 'ICDex', img: 'svg/icdex.svg', link: 'https://iclight.io/ICDex/GLDGov/ICP' },
    // { title: "MEXC", img: "svg/mexc-logo.svg", link: "" },
    // { title: "BITGET", img: "svg/bitget-logo.svg", link: "" },
  ];

  return (
    <article className="flex h-fit flex-row shadow-[0_0_12px_0_#00000026] w-full mt-5 border-[0.5px] border-[#C6C6C6] card bg-[#F3F3F3] rounded-box">
      <section className="basis-1/3 flex justify-center items-center border-r-2">
        <h1 className="font-bold text-xs sm:text-xl">TRADE ON</h1>
      </section>
      <section className="grid grid-cols-3 sm:flex sm:flex-1 w-full">
        {data.map(({ title, img, link }, index) => (
          <Link
            href={link}
            key={title}
            className={`flex flex-col basis-1/3 justify-center sm:justify-between items-center  p-2 sm:p-6 gap-4 cursor-pointer ${index === data.length - 1 ? ' ' : 'border-r-2'}`}
            target="_blank"
            rel="noopener noreferrer"
          >
            <div className="h-8 w-8 sm:w-20 sm:h-20 flex   justify-center items-center ">
              <Image alt={title} height={100} width={100} src={img} objectFit="full" />
            </div>
            <p className={'text-[8px] sm:text-xl text-center'}>{title}</p>
          </Link>
        ))}
      </section>
    </article>
  );
}
