import './modal.css';
import Image from 'next/image';
import { useEffect } from 'react';

export default function ModalCollapse({
  title, idModal = 'id_modal_1', children,
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
          className={`${title?.substring(0, 5) === 'chart' ? 'chart-modal-collapse' : 'box-modal-collapse'} rounded-t-3xl bg-SoftGrey border-b-[0.5px] border-DarkGrey`}
        >
          <div className='title w-full flex justify-between items-center px-5 py-4'>
            <h2 className="font-bold text-dark text-[28px]">{title}</h2>
            <form method="dialog">
              <button className="bg-DarkGrey size-[26px] rounded-full flex justify-center items-center outline-none">
                <Image
                  src={'svg/exit.svg'}
                  alt='exit'
                  height={8}
                  width={8}
                  className="size-3 outline-none"
                />
              </button>
            </form>
          </div>
          <div className="modal-content max-h-[calc(100vh-100px)] overflow-y-auto overflow-x-auto">
            {children}
          </div>
        </div>
      </dialog>
    </>
  );
}
