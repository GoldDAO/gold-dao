import { ReactNode, ButtonHTMLAttributes } from "react";
import clsx from "clsx";
import {
  Dialog as HeadlessUIDialog,
  DialogPanel,
  DialogBackdrop,
} from "@headlessui/react";
import Icon from "@shared/ui/icons";

type DialogSize = "xs" | "sm" | "md" | "xl" | "xxl" | "auto" | "full";

const getDialogSize = (size: DialogSize): string => {
  const sizeMap = {
    xs: "w-xs",
    sm: "w-sm",
    md: "w-md",
    xl: "w-2xl",
    xxl: "w-4xl",
    auto: "w-fit",
    full: "w-full",
  };
  return sizeMap[size];
};

const BackBtn = ({ ...restProps }: ButtonHTMLAttributes<HTMLButtonElement>) => {
  return (
    <button
      className={clsx(
        "p-1 rounded-full cursor-pointer",
        "hover:bg-primary hover:text-white"
      )}
      {...restProps}
    >
      <Icon.Chevron width={18} className="rotate-90" />
    </button>
  );
};

const CloseBtn = ({
  ...restProps
}: ButtonHTMLAttributes<HTMLButtonElement>) => {
  return (
    <button
      className={clsx(
        "p-1 rounded-full cursor-pointer",
        "hover:bg-primary hover:text-white"
      )}
      {...restProps}
    >
      <Icon.Close width={18} />
    </button>
  );
};

const Dialog = ({
  open = false,
  onClose = () => null,
  children,
  size = "xl",
}: {
  open: boolean;
  onClose?: () => void;
  children?: ReactNode;
  size?: DialogSize;
}) => {
  return (
    <HeadlessUIDialog
      open={open}
      onClose={onClose}
      transition
      className={clsx(
        "relative z-50",
        "transition duration-300 ease-in-out data-[closed]:opacity-0"
      )}
    >
      <DialogBackdrop className="fixed inset-0 bg-black/60" />
      <div
        className={clsx(
          "fixed inset-0",
          "flex w-screen items-center justify-center p-4"
        )}
      >
        <DialogPanel
          className={clsx(
            getDialogSize(size),
            "bg-surface-primary rounded-xl",
            "p-4 xl:p-6",
            "max-h-[70vh] md:max-h-[90vh] overflow-y-auto"
          )}
        >
          {children}
        </DialogPanel>
      </div>
    </HeadlessUIDialog>
  );
};

Dialog.BackBtn = BackBtn;
Dialog.CloseBtn = CloseBtn;

export default Dialog;
