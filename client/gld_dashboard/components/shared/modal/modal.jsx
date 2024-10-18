import './modal.css';

import Image from 'next/image';
import { useEffect } from 'react';
import { parseNumbers } from '../../../utils/parsers';

export default function Modal({
  title, idModal = 'id_modal_1', amount, children,
}) {
  useEffect(() => {
    const dialog = document.getElementById(idModal);
    const handleOutsideClick = (event) => {
      if (event.target === dialog) {
        dialog.close();
      }
    };

    dialog.addEventListener('click', handleOutsideClick);

    return () => {
      dialog.removeEventListener('click', handleOutsideClick);
    };
  }, [idModal]);
  return (
    <>
      <dialog id={idModal} className="modal overflow-hidden z-0">
        <div
          className={`${title?.substring(0, 5) === 'chart' ? 'chart-modal' : `${title.toLowerCase().includes('add neuron') ? 'h-[90vh] mb-60 w-full p-5 overflow-y-auto sm:h-[545px] sm:w-[694px] sm:mb-0 sm:p-[36px]' : 'box-modal'}`}  rounded-t-[2rem] sm:rounded-[2rem] bg-[#F3F3F3] border-[.5px] border-[#C6C6C6] shadow-[0_0_24px_0_rgba(0,0,0,0.5)]`}
        >
          <div
            className={`title w-[100%] ${title?.substring(0, 5) === 'chart' ? '' : 'border-b border-[#C6C6C6]'} flex justify-between items-center pb-4`}
          >
            <h2 className="font-bold text-dark text-[28px]">
              {title?.substring(0, 5) === 'chart' ? title?.substring(5) : title}
            </h2>
            <form method="dialog">
              <button className="bg-DarkGrey size-[26px] rounded-full flex justify-center items-center outline-none">
                <Image
                  src={'svg/exit.svg'}
                  alt="exit"
                  height={8}
                  width={8}
                  className="size-3 outline-none"
                />
              </button>
            </form>
          </div>
          {title?.substring(0, 5) === 'chart' ? (
            <h1 className="flex flex-row absolute mb-5 text-3xl font-bold text-black">
              {title?.substring(5) === 'Treasury'
                || title?.substring(5) === ' Total GLDGov Supply'
                || title?.substring(5) === ' GLDGov Price'
                || title?.substring(5) === ' GLDGov Marketcap'
                ? parseNumbers(amount)
                : parseNumbers(amount)}
              <Image width={25} height={25} src="svg/g-logo.svg" className="ml-2" alt="gold dao" />
            </h1>
          ) : (
            ''
          )}

          {children}
          <h3 className="text-base font-normal leading-5 text-DarkGrey mt-4">
            {title?.toLowerCase().includes('supply') && 'Total amount of GLDGov tokens minted.'}
            {title?.toLowerCase().includes('price') && 'Average price of GLDGov on the market.'}
            {title?.toLowerCase().includes('marketcap')
              && 'Total amount of GLDGov multiplied by the average price.'}
          </h3>
        </div>
      </dialog>
    </>
  );
}
