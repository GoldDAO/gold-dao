import { ReactNode } from "react";
import clsx from "clsx";
import {
  Dialog as HUIDialog,
  DialogPanel,
  DialogTitle,
  DialogBackdrop,
} from "@headlessui/react";
import { ChevronLeftIcon } from "@heroicons/react/20/solid";
import { XMarkIcon } from "@heroicons/react/24/outline";

const SIZES = {
  xs: "max-w-xs",
  sm: "max-w-sm",
  md: "max-w-md",
  xl: "max-w-2xl",
  xxl: "max-w-4xl",
  auto: "max-w-auto",
};

const BackIcon = ({ handleOnClick }: { handleOnClick: () => void }) => {
  return (
    <div
      className="p-1 rounded-full cursor-pointer hover:bg-primary hover:text-white"
      onClick={handleOnClick}
    >
      <ChevronLeftIcon className="h-6 w-6" />
    </div>
  );
};

const Dialog = ({
  open = false,
  handleOnClose = () => null,
  closeEnabled = true,
  title = undefined,
  children,
  size = "xl",
  handlePreviousStep = undefined,
}: {
  open: boolean;
  title?: ReactNode;
  handleOnClose?: () => void;
  children?: ReactNode;
  closeEnabled?: boolean;
  size?: keyof typeof SIZES;
  handlePreviousStep?: () => void;
}) => {
  return open ? (
    <HUIDialog
      open={open}
      onClose={closeEnabled ? handleOnClose : () => null}
      transition
      className={clsx(
        "relative z-50",
        "transition duration-1000 ease-in-out data-[closed]:opacity-0"
      )}
    >
      <DialogBackdrop className="fixed inset-0 bg-black/60 backdrop-blur-[2px]" />
      <div
        className={clsx(
          "fixed inset-0 flex w-screen items-center justify-center p-4"
        )}
      >
        <DialogPanel
          className={clsx(
            `container ${SIZES[size]}`,
            "bg-surface-primary rounded-xl p-4"
          )}
        >
          <DialogTitle className={"flex items-center"}>
            {(title || handlePreviousStep) && (
              <div className="flex items-center gap-2">
                {handlePreviousStep && (
                  <BackIcon handleOnClick={handlePreviousStep} />
                )}
                {title && (
                  <h2 className="font-semibold text-content/80 shrink-0">
                    {title}
                  </h2>
                )}
              </div>
            )}
            {closeEnabled && (
              <div className="w-full text-end">
                <button onClick={handleOnClose}>
                  <div
                    className={clsx(
                      "p-1 rounded-full cursor-pointer",
                      "hover:bg-primary hover:text-white"
                    )}
                  >
                    <XMarkIcon className="h-6 w-6" />
                  </div>
                </button>
              </div>
            )}
          </DialogTitle>
          <div className="max-h-[70vh] overflow-y-auto pr-2 xl:pr-0">
            {children}
          </div>
        </DialogPanel>
      </div>
    </HUIDialog>
  ) : null;
};

export default Dialog;
