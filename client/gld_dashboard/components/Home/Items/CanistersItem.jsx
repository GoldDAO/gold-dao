'use client';

import { Bounce, toast } from 'react-toastify';
import { useEffect, useState } from 'react';
import { copyContent, elapsedTime } from '../../../utils/functions';

import { CopyIcon } from '../../../utils/svgs';

const CanistersItem = ({
  id,
  type,
  cycles,
  freezingCycles,
  idleCycles,
  memory,
  status,
  ModuleHash,
  // eslint-disable-next-line camelcase
  freezing_threshold,
  controllers,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const array = new Uint8Array(ModuleHash);
  // eslint-disable-next-line no-bitwise
  const hexString = Array.from(array, (byte) => (`0${(byte & 0xff).toString(16)}`).slice(-2)).join(
    '',
  );
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
    <div className="border-t-[0.5px] border-[#C6C6C6]">
      <div
        className="flex items-center px-4 cursor-pointer text-[16px]"
        onClick={() => setIsOpen(!isOpen)}
      >
        <div
          className={`w-10 h-10 flex justify-center items-center  ${isOpen ? ' rotate-180' : 'rotate-0'}`}
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
              stroke-width="2"
              strokeLinecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </div>

        <div className="w-[1200px]  sm:w-[100%] pl-3 grid grid-cols-7 gap-5 text-center py-6  border-[#C6C6C6]">
          <div
            className="flex items-center"
            onClick={(e) => {
              e.stopPropagation();
              copyContent(id, setCopyState);
            }}
          >
            <p className="text-[#C6C6C6] text-start text-[16px] truncate underline">{id}</p>
            {CopyIcon}
          </div>

          <div className="w-full flex flex-row items-center  justify-start min-w-40">
            <span className="flex items-center px-4 font-bold text-white text-xs  rounded-[60px]  h-[32px] bg-[#C6C6C6]">
              {type.charAt(0).toUpperCase() + type.slice(1)}
            </span>
          </div>

          <div className="">
            {' '}
            <p className="w-full flex flex-row items-center justify-start min-w-40 gap-1">
              <span className="font-bold">{cycles}</span> T
            </p>
          </div>
          <div className="w-full  flex flex-row items-center  justify-start min-w-40">
            <p className="  flex justify-center gap-1">
              <span className="font-bold">{freezingCycles}</span> T
            </p>
          </div>
          <div className="w-full  flex flex-row items-center  justify-start min-w-40">
            {' '}
            <p className="font-bold flex justify-center">{idleCycles}</p>
          </div>
          <div className="w-full  flex flex-row items-center  justify-start min-w-40">
            <p className="flex justify-center gap-1">
              <span className="font-bold">{memory}</span> MiB
            </p>
          </div>
          <div className="w-full  flex flex-row items-center  justify-start min-w-40">
            <div className="flex justify-center">
              <span className="py-2 px-4 font-bold text-white text-xs  rounded-[60px]  h-[32px] bg-[#C6C6C6]">
                {status.charAt(0).toUpperCase() + status.slice(1)}
              </span>
            </div>
          </div>
        </div>
      </div>

      {isOpen && (
        <section
          className="flex start w-[100%] gap-5 px-17 py-5"
          style={{ transition: 'height 0.3s ease' }}
        >
          <div>
            <p className="text-[12px] font-bold text-[#D3B871]">Freezing Threshold Time</p>
            <p className="text-[12px] font-bold mt-2">
              {elapsedTime(freezing_threshold) === 'a month'
                ? '30 days'
                : elapsedTime(freezing_threshold)}
            </p>
          </div>
          <div>
            <p className="text-[12px] font-bold text-[#D3B871]">Controllers</p>
            <p className="text-[12px] font-bold mt-2">{controllers}</p>
          </div>
          <div>
            <p className="text-[12px] font-bold text-[#D3B871]">Module Hash</p>
            <p className="text-[12px] font-bold mt-2"> {hexString}</p>
          </div>
        </section>
      )}
    </div>
  );
};

export default CanistersItem;
