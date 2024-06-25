/* eslint-disable no-nested-ternary */
import React from 'react';
import CanistersItemMobile from '../../Home/Items/CanisterItemMobile';
import NavbarModal from './NavbarModal';

const ModalCanisters = ({ canisters }) => {
  const headerItems = [
    { label: 'ID', infoSrc: '/svg/info.svg' },
    { label: 'Type', infoSrc: '/svg/info.svg' },
    { label: 'Cycles Balance', infoSrc: '/svg/info.svg' },
    { label: 'Freezing Threshold Cycles', infoSrc: '/svg/info.svg' },
    { label: 'Idle Cycles Burned Per Day', infoSrc: '/svg/info.svg' },
    { label: 'Memory Size', infoSrc: '/svg/info.svg' },
    { label: 'Status', infoSrc: '/svg/info.svg' },
  ];
  return (
    <>
      <div className="w-full mb-[58%]">
        <thead className="border-y-[0.5px] border-y-[#C6C6C6] flex flex-row items-center justify-around py-[20px] pl-6 w-[800px]">
          <div className="w-5"></div>
          <tr className="grid grid-cols-7  gap-5">
            {headerItems.map((item, index) => (
              <th key={index} className="w-full flex flex-row items-center justify-start ">
                <p className="text-xs font-bold text-[#D3B871] text-start">{item.label}</p>
              </th>
            ))}
          </tr>
        </thead>
        <div className="w-[800px]">
          {/* Contenedor adicional para permitir el desplazamiento horizontal */}
          {canisters.loading ? (
            <article className="collapse w-fit border-[1px] rounded-none flex justify-center">
              <section className="collapse-title flex gap-2 justify-center items-center h-20">
                <span className="loading loading-spinner"></span>
                Loading canisters...
              </section>
            </article>
          ) : canisters?.data?.length > 0 ? (
            canisters?.data?.map((c) => <CanistersItemMobile key={c.id} {...c} />)
          ) : (
            <article className="collapse w-fit border-[1px] rounded-none flex justify-center">
              <section className="collapse-title flex gap-2 justify-center items-center h-20">
                Fail to fetch canisters data. Please, retry again.
              </section>
            </article>
          )}
        </div>
      </div>
      <NavbarModal />
    </>
  );
};

export default ModalCanisters;
