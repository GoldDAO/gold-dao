'use client';

import Image from 'next/image';
import React from 'react';
import { parseNumbers } from '../../utils/parsers';

const DataCardCopy = ({
  title,
  amount,
  image,
  info,
  isPrice,
  setInfoModal,
  data,
  openModal,
}) => {
  const mql = window.matchMedia('(max-width: 600px)');

  return (
    <article
      className='flex flex-col justify-between w-full h-21 sm:h-[164px] p-5 sm:p-[36px] border-[0.5px] border-DarkGrey bg-SoftGrey rounded-3xl sm:rounded-4xl shadow-[0_0_6px_0_#00000026]'
      onClick={() => {
        setInfoModal({
          title,
          image,
          info,
          amount,
          data,
        });
        if (openModal && mql.matches === true) {
          document.getElementById('chartmodalheader').showModal();
        }
      }}
    >
      <section className="flex">
        <div className="flex gap-2 items-center">
          <h2 className="text-xs sm:text-lg font-normal">{title}</h2>
          <div className="tooltip hidden sm:flex" data-tip={info}>
            <button>
              <Image alt="more information" height={20} width={20} src="svg/info.svg" />
            </button>
          </div>
        </div>
      </section>
      <section className="flex gap-2 items-center -mb-2">
        <h1 className="text-[28px] sm:text-[36px] font-bold" style={{wordBreak: "break-word", lineHeight: 1.2}}>
          {isPrice && '$'}
          {parseNumbers(amount)}
        </h1>
        {image && (
          <div className="size-10 justify-start items-center flex">
            <Image alt={title} height={30} width={30} src="svg/g-logo.svg" />
          </div>
        )}
      </section>
    </article>
  );
};

export default DataCardCopy;
