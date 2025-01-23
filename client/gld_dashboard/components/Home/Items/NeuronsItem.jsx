'use client';

import { useState } from 'react';
import { arrayToHex, elapsedTime, formatDateFromSeconds } from '../../../utils/functions';

import { parseNumbers } from '../../../utils/parsers';

const NeuronsItem = ({ neuron }) => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="border-t border-[#C6C6C6]">
      <div
        className="flex items-center px-5 cursor-pointer text-[16px]"
        onClick={() => setIsOpen(!isOpen)}
      >
        <div
          className={`w-10 h-10 flex justify-center items-center  ${isOpen && 'rotate-180'}`}
        >
          <svg
            width="20"
            height="10"
            viewBox="0 0 14 8"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M1 1L7 7L13 1"
              stroke="black"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </div>

        <div className="w-[1200px]  sm:w-[100%] pl-3 grid grid-cols-7 gap-5 text-center py-6  border-[#C6C6C6]">
          <div className="">
            {' '}
            <p className="text-[#C6C6C6] text-[16px] truncate underline">{arrayToHex(neuron.id)}</p>
          </div>

          <div className="flex justify-start ">
            <span className="px-4 text-white text-xs font-bold rounded-[60px]  h-[32px] bg-[#C6C6C6] flex items-center">
              {neuron.dissolving}
            </span>
          </div>

          <div className=" flex justify-start">
            {' '}
            <p className="">
              <span className="font-bold whitespace-nowrap truncate">
                {parseNumbers(neuron.stakedAmount, 10)}
              </span>{' '}
              GOLDAO
            </p>
          </div>
          <div className=" flex justify-start">
            {' '}
            <p className="font-bold">{(neuron.maturity / 1e8).toFixed(2) || 0}</p>
          </div>
          <div className=" flex justify-start">
            <p className="font-bold ">{elapsedTime(neuron.dissolveDelay)}</p>
          </div>
          <div className=" flex justify-start">
            {' '}
            <p className="font-bold  ">{elapsedTime(neuron.age)}</p>
          </div>
          <div className=" flex justify-start ">
            {' '}
            <p className="font-bold ">
              {neuron.votingPower === 0 ? '-' : neuron.votingPower}
            </p>
          </div>
        </div>
      </div>

      {isOpen && (
        <section
          className="flex justify-start w-full gap-10 px-18 py-5"
          style={{ transition: 'height 0.3s ease' }}
        >
          <div>
            <p className="text-xs font-bold text-[#D3B871]">Date Created</p>
            <p className="text-xs font-bold mt-2">
              {formatDateFromSeconds(neuron.dateCreated)}, {' '}
              {elapsedTime(Math.round(new Date().getTime() / 1000) - neuron.dateCreated)}
            </p>
          </div>
          <div>
            <p className="text-xs font-bold text-[#D3B871]">Auto-Stake Maturity</p>
            <p className="text-xs font-bold mt-2">
              {neuron.autoStakeMaturity ? 'Yes' : 'No'}
            </p>
          </div>
          <div>
            <p className="text-xs font-bold text-[#D3B871]">Dissolve Delay Bonus</p>
            <p className="text-xs font-bold mt-2">{(neuron.dissolveDelayBonus * 100).toFixed(2)} %</p>
          </div>
          <div>
            <p className="text-xs font-bold text-[#D3B871]">Age Bonus</p>
            <p className="text-xs font-bold mt-2">{(neuron.ageBonus * 100).toFixed(2)} %</p>
          </div>
          <div>
            <p className="text-xs font-bold text-[#D3B871]">Total Bonus</p>
            <p className="text-xs font-bold mt-2">{((neuron.totalBonus - 1) * 100).toFixed(2)} %</p>
          </div>
        </section>
      )}
    </div>
  );
};

export default NeuronsItem;
