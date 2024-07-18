import { Fragment, ReactNode } from "react";
import {
  Transition,
  TransitionChild,
  Dialog,
  DialogPanel,
  DialogTitle,
} from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";

const DialogComponent = ({
  show = true,
  handleClose,
  enableClose = true,
  children,
}: {
  show: boolean;
  handleClose: () => void;
  children?: ReactNode;
  enableClose?: boolean;
}) => {
  return (
    <Transition show={show} as={Fragment}>
      <div className="fixed z-50 inset-0 overflow-hidden">
        <Dialog
          static
          as={Fragment}
          onClose={enableClose ? handleClose : () => null}
        >
          <div className="absolute z-50 inset-0 overflow-hidden">
            <TransitionChild
              as={Fragment}
              enter="ease-in-out duration-500"
              enterFrom="opacity-0"
              enterTo="opacity-100"
              leave="ease-in-out duration-500"
              leaveFrom="opacity-100"
              leaveTo="opacity-0"
            >
              <div className="fixed inset-0 bg-black bg-opacity-50 transition-opacity" />
            </TransitionChild>
            <div className="fixed flex justify-center h-screen w-full p-4">
              <TransitionChild
                as={Fragment}
                enter="ease-out duration-300"
                enterFrom="opacity-0 scale-95"
                enterTo="opacity-100 scale-100"
                leave="ease-in duration-200"
                leaveFrom="opacity-100 scale-100"
                leaveTo="opacity-0 scale-95"
              >
                <DialogPanel
                  className={`bg-surface rounded-xl w-full max-w-xl transform transition-all my-auto`}
                >
                  <DialogTitle className="flex justify-end px-6 pt-6">
                    {enableClose && (
                      <button onClick={handleClose}>
                        <div className="hover:bg-accent hover:text-white p-1 rounded-full">
                          <XMarkIcon className="h-8 w-8" />
                        </div>
                      </button>
                    )}
                  </DialogTitle>
                  {children}
                </DialogPanel>
              </TransitionChild>
            </div>
          </div>
        </Dialog>
      </div>
    </Transition>
  );
};

export default DialogComponent;
