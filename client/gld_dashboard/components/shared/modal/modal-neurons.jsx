/* eslint-disable no-nested-ternary */
import React from 'react';
import NeuronsItemMobile from '../../Home/Items/NeuronsItemMobile';
import NavbarModal from './NavbarModal';

const ModalNeurons = ({ neurons }) => {
  const headerItems = [
    { label: 'ID', iconSrc: '/svg/info.svg' },
    { label: 'State', iconSrc: '/svg/info.svg' },
    { label: 'Staked GOLDAO', iconSrc: '/svg/info.svg' },
    { label: 'Maturity', iconSrc: '/svg/info.svg' },
    { label: 'Dissolve Delay', iconSrc: '/svg/info.svg' },
    { label: 'Age', iconSrc: '/svg/info.svg' },
    { label: 'Voting Power', iconSrc: '/svg/info.svg' },
  ];

  return (
    <div className="">
      <thead className="border-y-[0.5px] border-y-[#C6C6C6] flex flex-row items-center justify-start py-[20px] w-[1000px]">
        <div className='w-10 h-10 flex justify-center items-center'></div>
        <tr className="grid grid-cols-7 px-5  w-[1000px] gap-5">
          {headerItems.map((item, index) => (
            <th key={index} className="flex items-center justify-start gap-2">
              <p className="text-xs font-bold text-[#D3B871] text-start">{item.label}</p>
            </th>
          ))}
        </tr>
      </thead>

      <div className="w-[1000px]">
        {neurons.loading ? (
          <article className="collapse w-screen border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              <span className="loading loading-spinner"></span>
              Loading neurons...
            </section>
          </article>
        ) : (neurons?.data?.length > 0 ? (
          neurons?.data?.map((c) => <NeuronsItemMobile key={c.id} neuron={c} />)
        ) : (
          <article className="collapse w-fit border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              Fail to fetch neurons data. Please, retry again.
            </section>
          </article>
        ))}
      </div>
      <NavbarModal />
    </div>
  );
};

export default ModalNeurons;
