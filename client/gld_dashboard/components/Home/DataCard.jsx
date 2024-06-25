'use client';

import Image from 'next/image';
import React from 'react';
import { parseNumbers } from '../../utils/parsers';

const DataCard = ({
  title, amount, image, info, isPrice, className, setDataNeuron,
}) => (
    <article
      className={`shadow-[0_0_12px_0_#00000026] border-[0.5px] border-DarkGrey grid card bg-SoftGrey rounded-[24px] sm:rounded-[36px] py-5 sm:py-10 px-6 mb-5 sm:mb-0 ${className}`}
      onClick={() => {
        if (window.innerWidth < 768) {
          if (title === 'OGY Neuron') return;
          document.getElementById('neuronmodal').showModal();
          setDataNeuron({
            name: title.substring(0, 3),
            amount,
            image,
            info,
          });
        }
      }}
    >
      {title === 'OGY Neuron' && (
        <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center bg-black bg-opacity-20    rounded-[24px] sm:rounded-[36px]">
          <div className="text-white text-xl sm:text-4xl font-bold flex justify-center items-center gap-2 z-10">
            <p>Coming Soon</p>
            <div
              className="tooltip cursor-pointer font-normal text-lg text-white"
              data-tip={
                'The neuron of 500 million OGY will be released later this year. The ORIGYN Foundation is currently working on an upgrade.'
              }
            >
              <div className="h-5 w-5 sm:w-10 sm:h-10 hidden sm:flex">
                {' '}
                <Image src={'svg/infoWhite.svg'} alt="f" height={30} width={30} />
              </div>
            </div>
          </div>
        </div>
      )}
      <section
        className={`flex justify-between mb-4 ${title === 'OGY Neuron' ? 'opacity-20' : ''}  `}
      >
        <div className="flex gap-2 items-center">
          <p className="whitespace-nowrap text-[18px]">{title}</p>
          <div
            className={`${title === 'OGY Neuron' ? '' : 'tooltip'} hidden sm:flex`}
            data-tip={info}
          >
            <button>
              <Image alt="more information" height={20} width={20} src="svg/info.svg" />
            </button>
          </div>
        </div>
        <button
          className="hidden sm:flex justify-center items-center  rounded-full bg-[#C6C6C6] h-8 w-8 mt-[-10px] "
          onClick={() => {
            if (title === 'OGY Neuron') return;
            document.getElementById('neuronmodal').showModal();
            setDataNeuron({
              name: title.substring(0, 3),
              amount,
              image,
              info,
            });
          }}
          // disabled={true}
        >
          <Image alt="expand data" height={10} width={10} src="svg/expand.svg" />
        </button>
      </section>
      <section className={`flex gap-2 items-center ${title === 'OGY Neuron' ? 'opacity-20' : ''}`}>
        <h1 className="sm:text-[32px] font-bold">
          {isPrice && '$'}
          {parseNumbers(amount)}
        </h1>
        {image && (
          <div className={'h-5 w-5 sm:w-10 sm:h-10 justify-start items-center flex '}>
            <Image alt={title} height={30} width={30} src={image} />
          </div>
        )}
      </section>
    </article>
);

export default DataCard;
