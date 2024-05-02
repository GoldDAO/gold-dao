import Link from 'next/link';
import { useEffect, useState } from 'react';

const TransactionItem = ({
  index, amount, type, timestamp, from, to,
}) => {
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const mediaQuery = window.matchMedia('(max-width: 1200px)');
    const handleMediaQueryChange = (event) => {
      setIsMobile(event.matches);
    };

    mediaQuery.addEventListener('change', handleMediaQueryChange);
    handleMediaQueryChange(mediaQuery);

    return () => {
      mediaQuery.removeEventListener('change', handleMediaQueryChange);
    };
  }, []);

  const formattedAmount = isMobile ? parseFloat(amount).toFixed(2) : amount;
  return (
    <div className="grid grid-cols-6 text-start py-6 px-10 w-[1200px] sm:w-[100%] border-t-[0.5px] border-[#C6C6C6]">
      <div className="w-full flex flex-row items-center  justify-start min-w-40 ">
        <p className="min-w-40 text-[16px]">{index}</p>
      </div>
      <div className="flex flex-row items-center  justify-start w-[90%]">
        <p className="flex justify-start gap-1 truncate">
          <span className="font-bold text-[16px truncate]">{formattedAmount}</span> GLDGov
        </p>
      </div>
      <div className="w-full flex flex-row items-center  justify-start min-w-40">
        <span className="py-2 px-4 text-white text-[12px] text-center font-bold rounded-[60px] w-[103px] h-[32px] bg-[#C6C6C6] flex justify-center items-center gap-2">
          {type.charAt(0).toUpperCase() + type.slice(1)}
          <div>
            <svg
              width="13"
              height="13"
              viewBox="0 0 13 13"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M0.999999 9.9375L12 9.9375M12 9.9375L9.9375 12M12 9.9375L9.9375 7.875"
                stroke="white"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
              <path
                d="M12 3.0625L0.999999 3.0625M0.999999 3.0625L3.0625 5.125M0.999999 3.0625L3.0625 1"
                stroke="white"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          </div>
        </span>
      </div>
      <div className="flex flex-row items-center  justify-start w-[90%]">
        <p className="font-bold text-[12px] truncate">{timestamp}</p>
      </div>
      <div className="w-full flex flex-row items-center  justify-start min-w-40">
        {type === 'mint' ? (
          <p className={'text-[#C6C6C6] text-[12px] font-bold'}>Minting Account </p>
        ) : (
          <Link
            href={`https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai/account/${from}`}
            target="_blank"
            rel="noopener noreferrer"
            className={'text-[#C6C6C6] underline text-[12px] font-bold'}
          >
            {from?.substring(0, 20)}
          </Link>
        )}
      </div>
      <div className="w-full flex flex-row items-center  justify-start min-w-40">
        {
          type === 'burn'
            ? <p className='text-[#C6C6C6] text-[12px] font-bold'>-</p>
            : <Link
                href={`https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai/account/${to}`}
                target="_blank"
                rel="noopener noreferrer"
                className="text-[#C6C6C6] underline text-[12px] font-bold"
              >
                {to?.substring(0, 20) || '-'}
              </Link>}
      </div>
    </div>
  );
};

export default TransactionItem;
