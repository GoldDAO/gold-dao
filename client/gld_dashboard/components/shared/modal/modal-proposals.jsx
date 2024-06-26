/* eslint-disable no-nested-ternary */
import Image from 'next/image';
import React from 'react';
import ProposalItem from '../../Home/Items/ProposalItem';
import NavbarModal from './NavbarModal';

const ModalProposals = ({ proposals }) => {
  const headerItems = [
    { label: 'ID', iconSrc: '/svg/id.svg', padding: true },
    { label: 'Title', iconSrc: '/svg/title.svg' },
    { label: 'Topic', iconSrc: '/svg/topic.svg' },
    { label: 'Status', iconSrc: '/svg/status.svg' },
    { label: 'Votes', iconSrc: '/svg/votes.svg' },
  ];
  return (
    <div className="mb-[55%]">
      <thead className="border-y-[0.5px] border-y-[#C6C6C6] flex flex-row items-center justify-around py-[20px] w-[100%] px-1">
        <tr className="grid grid-cols-5  w-[100%] gap-5">
          {headerItems.map((item, index) => (
            <th
              key={index}
              className={`flex flex-row justify-start  items-center  w-fit   gap-1 ${item.padding ? 'pl-2 sm:pl-20' : ''} `}
            >
              <Image alt={`${item.label} icon`} height={15} width={15} src={item.iconSrc} />
              <p className="text-xs font-bold ml-0 text-[#D3B871]">{item.label}</p>
            </th>
          ))}
        </tr>
      </thead>
      <div className="divide-y">
        {proposals.loading ? (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              <span className="loading loading-spinner"></span>
              Loading proposals...
            </section>
          </article>
        ) : proposals?.data?.length > 0 ? (
          proposals?.data?.map((c) => <ProposalItem key={c.id} {...c} />)
        ) : (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              Fail to fetch proposals data. Please, retry again.
            </section>
          </article>
        )}
      </div>
      <NavbarModal />
    </div>
  );
};

export default ModalProposals;
