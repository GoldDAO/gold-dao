import { Bounce, toast } from 'react-toastify';
import React, { useEffect, useState } from 'react';
import Image from 'next/image';
import { copyContent } from '../../../utils/functions';
import { parseNumbers } from '../../../utils/parsers';

const ModalNeuron = ({ name, image, neurons }) => {
  const [copyState, setCopyState] = useState(false);

  useEffect(() => {
    if (copyState) {
      toast.success('Copied', {
        position: 'top-right',
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      setCopyState(false);
    }
  }, [copyState]);

  return (
    <div className="overflow-y-auto mb-[58%] sm:mb-0">
      {/* General Information */}
      <div className="flex justify-between w-full pt-10 mb-5">
        <div className="w-full flex justify-between items-center">
          {
            name === "OGY" ?
              <p className="text-2xs font-normal sm:text-sm text-black text-justify w-[60%]">
                The Gold DAO has received a donation of 500 million OGY tokens from the ORIGYN Foundation.
                These tokens are locked in a 5 year neuron and its rewards
                are distributed to active Gold DAO neuron owners.
              </p> :
              <p className="text-2xs font-normal sm:text-sm text-black text-justify w-[60%]">
                The Gold DAO has {neurons.length} neurons of {name} with different times of Dissolve
                Delay. Each Neuron has voting power in the
                {name} governance and earns rewards by voting on
                proposals.
              </p>
          }
          <div className="bg-white size-25 rounded-full flex justify-center items-center">
            {image && (<Image src={image} width={62} height={30} alt="icp" className="w-12 h-12 object-contain" />)}
          </div>
        </div>
      </div>
      <div className="flex justify-between w-full pt-2 mb-2">
        <div className="w-full flex justify-between items-center">
          <a href={name === 'OGY' ? 'https://docs.gold-dao.org/resources/gold-dao-owned-neurons/ogy-neuron' : 'https://docs.gold-dao.org/resources/gold-dao-owned-neurons/nns-neurons'} target="_blank">
            <h2 className="w-full flex gap-2">
              <b>Learn more</b>
              <Image src="svg/about.svg" alt="About Icon" width={20} height={20} />
            </h2>
          </a>
        </div>
      </div>

      {/* Neurons */}
      <div className='grid grid-cols-2 gap-4 mt-[17%]'>
        {neurons[name]?.map((neuron, index) => (
          <div key={neuron.id} className="bg-white rounded-lg">
            <div className="flex justify-start items-center border-b-2 border-gray-200 p-2 text-xs gap-2">
              <span className="truncate text-2xs">
                <a href={`${name === 'OGY' ? 'https://dashboard.internetcomputer.org/sns/leu43-oiaaa-aaaaq-aadgq-cai/neuron/' : 'https://dashboard.internetcomputer.org/neuron/'}${neuron.id?.toString()}`} target='_blank'>{neuron.id?.toString()}</a>
              </span>
              <Image
                src="svg/copy-button.svg"
                height={12}
                width={12}
                onClick={() => copyContent(neuron.id?.toString?.(), setCopyState)}
                className="cursor-pointer size-3"
                alt="copy icon"
              />
              {copyState.data === index && copyState.state === true && (
                <span className="text-green-500">copied</span>
              )}
            </div>

            {/* Amount */}
            <div className="flex justify-start p-2 items-center">
              <div className="text-2xl font-bold mr-2">{parseNumbers(neuron.stakedAmount)}</div>
              <div>
                {image && (<Image src={image} height={24} width={24} alt="icp" className="size-6" />)}
              </div>
            </div>

            {/* Dissolve Time */}
            <div className="p-2 text-2xs">
              <h4>Dissolving in {neuron?.dissolveDelay} {!isNaN(neuron?.dissolveDelay) && (neuron?.dissolveDelay > 1 ? 'years' : 'year')}</h4>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ModalNeuron;
