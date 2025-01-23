import Link from 'next/link';
import React from 'react';

const MobileRedirectButtons = ({ className }) => (
    <section className={`flex flex-col gap-4 mt-4 text-sm font-bold capitalize ${className}`}>
      <Link href={'/overview'}>
        <div className='w-full rounded-full h-16 bg-SoftGrey border-[0.5px] border-DarkGrey p-5'>
          overview
        </div>
      </Link>
      <Link href={'/proposals'}>
        <div className='w-full rounded-full h-16 bg-SoftGrey border-[0.5px] border-DarkGrey p-5'>
          proposals
        </div>
      </Link>
      <Link href={'/transactions'}>
        <div className='w-full rounded-full h-16 bg-SoftGrey border-[0.5px] border-DarkGrey p-5'>
          GOLDAO transactions
        </div>
      </Link>
      <Link href={'/canisters'}>
        <div className='w-full rounded-full h-16 bg-SoftGrey border-[0.5px] border-DarkGrey p-5'>
          Gold DAO Canisters
        </div>
      </Link>
      <Link href={'/neurons'}>
        <div className='w-full rounded-full h-16 bg-SoftGrey border-[0.5px] border-DarkGrey p-5'>
          Gold DAO Neurons
        </div>
      </Link>
    </section>
);

export default MobileRedirectButtons;
