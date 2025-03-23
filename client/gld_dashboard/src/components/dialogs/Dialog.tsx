import { ReactNode } from "react";
import clsx from "clsx";
import {
  Dialog as HUIDialog,
  DialogPanel,
  DialogTitle,
  DialogBackdrop,
} from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";

const SIZES = {
  xs: "max-w-xs",
  sm: "max-w-sm",
  md: "max-w-md",
  lg: "max-w-lg",
  xl: "max-w-2xl",
  xxl: "max-w-7xl",
  auto: "max-w-auto",
};

const Dialog = ({
  open = false,
  handleOnClose = () => null,
  disableClose = false,
  title = undefined,
  children,
  size = "xl",
}: {
  open: boolean;
  title?: ReactNode;
  handleOnClose?: () => void;
  children?: ReactNode;
  disableClose?: boolean;
  size?: keyof typeof SIZES;
}) => {
  return open ? (
    <HUIDialog
      open={open}
      onClose={!disableClose ? handleOnClose : () => null}
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
            "bg-surface-primary rounded-xl p-4 lg:p-8"
          )}
        >
          <DialogTitle className={"flex items-center"}>
            {title && (
              <div className="font-semibold text-content/80 shrink-0">
                {title}
              </div>
            )}
            {!disableClose && (
              <div className="w-full text-end">
                <button onClick={handleOnClose}>
                  <div
                    className={clsx(
                      "p-1 rounded-full cursor-pointer",
                      "hover:bg-secondary hover:text-white"
                    )}
                  >
                    <XMarkIcon className="h-6 w-6" />
                  </div>
                </button>
              </div>
            )}
          </DialogTitle>
          {children}
        </DialogPanel>
      </div>
    </HUIDialog>
  ) : null;
};

export default Dialog;
