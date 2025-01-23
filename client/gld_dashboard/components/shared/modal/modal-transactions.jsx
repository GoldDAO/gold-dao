/* eslint-disable no-nested-ternary */

import Link from 'next/link';
import React from 'react';
import NavbarModal from './NavbarModal';

const ModalTransactions = ({ transactions }) => {
  const headerItems = [
    { label: 'Index', infoSrc: '/svg/info.svg' },
    { label: 'Amount', infoSrc: '/svg/info.svg' },
    { label: 'Type', infoSrc: '/svg/info.svg' },
    { label: 'Timestamp', infoSrc: '/svg/info.svg' },
    { label: 'From', infoSrc: '/svg/info.svg' },
    { label: 'To', infoSrc: '/svg/info.svg' },
  ];
  return (
    <div className="w-full mb-8">
      <thead className="border-y-[0.5px] border-y-[#C6C6C6] flex flex-row items-center justify-around py-[20px] w-full  px-1 ">
        <tr className="grid grid-cols-6  ">
          {headerItems.map((item, index) => (
            <th key={index} className="w-full flex items-center  justify-center ">
              <p className="text-xs font-bold text-[#D3B871] ">{item.label}</p>
            </th>
          ))}
        </tr>
      </thead>{' '}
      <div className="divide-y">
        {transactions.loading ? (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              <span className="loading loading-spinner"></span>
              Loading transactions...
            </section>
          </article>
        ) : transactions?.data?.length > 0 ? (
          transactions?.data?.map(({
            index, amount, type, timestamp, from, to,
          }) => (
            <div key={index} className="grid grid-cols-6 text-center py-6 px-1  w-full ">
              <div className="w-full flex flex-row items-center  justify-center min-w-fit">
                <p className="min-w-fit text-[8px]">{index}</p>
              </div>
              <div className="w-full flex flex-row items-center  justify-center min-w-fit">
                <p className="flex flex-wrap justify-center gap-1 min-w-fit text-[8px]">
                  <span className="font-bold">{amount}</span> GOLDAO
                </p>
              </div>
              <div className="flex flex-row items-center  justify-center min-w-fit">
                <span className="py-2 px-2 text-white text-[8px] text-center font-bold rounded-[60px] w-fit h-[25px] bg-[#C6C6C6] flex justify-center items-center gap-1">
                  {type.charAt(0).toUpperCase() + type.slice(1)}
                </span>
              </div>
              <div className="w-full flex flex-row items-center justify-center ">
                <p className="flex flex-wrap font-bold text-[8px] ">
                  {timestamp?.split(',')?.join(' ')}
                </p>
              </div>

              <div className="w-full flex flex-row items-center  justify-center">
                {type === 'mint' ? (
                  <p className={'text-[#C6C6C6] text-[10px] font-bold'}>Minting Account </p>
                ) : (
                  <Link
                    href={`https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai/account/${from}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-[#C6C6C6] underline text-[10px] font-bold"
                  >
                    {from?.substring(0, 5)}
                  </Link>
                )}
              </div>
              <div className="w-full flex flex-row items-center justify-center font-bold">
                <Link
                  href={`https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai/account/${to}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-[#C6C6C6] underline text-[10px]"
                >{`${to.substring(0, 5)}...`}</Link>
              </div>
            </div>
          ))
        ) : (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              Fail to fetch transactions data. Please, retry again.
            </section>
          </article>
        )}
        <NavbarModal />
      </div>
    </div>
  );
};

export default ModalTransactions;
