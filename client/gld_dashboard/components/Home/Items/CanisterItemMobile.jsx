'use client';

import { Bounce, toast } from 'react-toastify';
import { useEffect, useState } from 'react';
import { copyContent, elapsedTime } from '../../../utils/functions';

import { CopyIcon } from '../../../utils/svgs';

const CanistersItemMobile = ({
  id,
  type,
  cycles,
  freezingCycles,
  idleCycles,
  memory,
  status,
  ModuleHash,
  freezingThreshold,
  controllers,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const array = new Uint8Array(ModuleHash);
  // eslint-disable-next-line no-bitwise
  const hexString = Array.from(array, (byte) => (`0${(byte & 0xff).toString(16)}`).slice(-2)).join('');

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
    <div className="border-t border-[#C6C6C6]">
      <div className="flex items-center cursor-pointer" onClick={() => setIsOpen(!isOpen)}>
        <div
          className={`w-10 h-10 flex justify-center items-center  ${isOpen ? ' rotate-180' : 'rotate-0'}`}
        >
          <svg
            width="10"
            height="5"
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

        <div className="grid grid-cols-7  w-full gap-5 text-center py-5 items-center ">
          <div
            className="flex items-center"
            onClick={(e) => {
              e.stopPropagation();
              copyContent(id, setCopyState);
            }}
          >
            <p className="text-[#C6C6C6] text-xs text-start truncate underline">{id}</p>
            {CopyIcon}
          </div>

          <div className=" flex flex-grow items-center justify-center">
            <span className="flex items-center justify-center px-4 font-bold text-white text-[10px] w-full text-center rounded-[60px]  h-[28px] bg-[#C6C6C6]">
              {type.charAt(0).toUpperCase() + type.slice(1)}
            </span>
          </div>

          <div className="">
            {' '}
            <p className="w-full flex flex-grow items-center  text-[12px] justify-start">
              <span className="font-bold">{cycles}</span> T
            </p>
          </div>
          <div className="w-full  flex flex-grow items-center  justify-start text-[12px]">
            <p className="  flex justify-center">
              <span className="font-bold">{freezingCycles}</span> T
            </p>
          </div>
          <div className="w-full  flex flex-grow items-center  justify-start text-[12px]">
            {' '}
            <p className="font-bold flex justify-center">{idleCycles}</p>
          </div>
          <div className="w-full  flex flex-grow items-center  justify-start text-[10px]">
            <p className="flex justify-center">
              <span className="font-bold">{memory}</span> MiB
            </p>
          </div>
          <div className="w-full  flex flex-grow items-center  justify-start text-[10px]">
            <div className="flex justify-center">
              <span className="py-2 px-4 font-bold text-white text-[10px]  rounded-[60px]  h-[30px] bg-[#C6C6C6]">
                {status.charAt(0).toUpperCase() + status.slice(1)}
              </span>
            </div>
          </div>
        </div>
      </div>

      {isOpen && (
        <section
          className="flex start w-[100%] gap-5 px-12 py-5"
          style={{ transition: 'height 0.3s ease' }}
        >
          <div>
            <p className="text-[12px] font-bold text-[#D3B871]">Freezing Threshold Time</p>
            <p className="text-[12px] font-bold mt-2">
              {elapsedTime(freezingThreshold) === 'a month'
                ? '30 days'
                : elapsedTime(freezingThreshold)}
            </p>
          </div>
          <div>
            <p className="text-[12px] font-bold text-[#D3B871]">Controllers</p>
            <p className="text-[12px] font-bold mt-2">{`${controllers.substring(0, 5)}...`}</p>
          </div>
          <div>
            <p className="text-[12px] font-bold text-[#D3B871]">Module Hash</p>
            <p className="text-[12px] font-bold mt-2"> {`${hexString.substring(0, 20)}...`}</p>
          </div>
        </section>
      )}
    </div>
  );
};

export default CanistersItemMobile;
