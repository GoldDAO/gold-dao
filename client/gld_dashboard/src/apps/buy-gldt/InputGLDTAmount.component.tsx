import { useEffect, useRef } from "react";
import { useAtom } from "jotai";
import clsx from "clsx";

import { Logo } from "@components/index";

import BuyGLDTStateAtom from "./atoms";

const InputGLDTAmount = ({
  className,
  handleOnChange,
}: {
  className?: string;
  handleOnChange: (value: number) => void;
}) => {
  const formRef = useRef<HTMLFormElement>(null);
  const [buyAtomState, setBuyAtomstate] = useAtom(BuyGLDTStateAtom);
  const { is_new_swap } = buyAtomState;

  const onChange = (value: string) => {
    handleOnChange(value !== "" ? Number(value) : 0);
  };

  const preventMinus = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.code === "Minus") e.preventDefault();
  };

  const preventPasteNegative = (e: React.ClipboardEvent<HTMLInputElement>) => {
    const clipboardData = e.clipboardData;
    const pastedData = parseFloat(clipboardData.getData("text"));
    if (pastedData < 0) e.preventDefault();
  };

  useEffect(() => {
    if (is_new_swap && formRef.current) {
      setBuyAtomstate((state) => ({
        ...state,
        is_new_swap: false,
      }));
      formRef.current.reset();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [is_new_swap]);

  return (
    <div className={className}>
      <form
        onChange={(e) => onChange(e.currentTarget.amount.value)}
        className="flex justify-center items-center gap-4"
        ref={formRef}
      >
        <input
          id="amount"
          type="number"
          min={0}
          autoComplete="off"
          placeholder="0.00"
          className={clsx(
            "field-sizing-content max-w-56 text-right outline-none focus:outline-none focus:border-none focus:ring-0 bg-surface-primary",
            "text-3xl lg:text-6xl font-semibold",
            "placeholder:text-content/40 placeholder:text-3xl lg:placeholder:text-6xl placeholder:font-semibold",
            "[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
          )}
          onPaste={preventPasteNegative}
          onKeyDown={preventMinus}
        />

        <div className={clsx("text-2xl lg:text-6xl font-semibold text-accent")}>
          GLDT
        </div>

        <div className="flex items-center justify-center rounded-full bg-surface-secondary h-10 w-10 lg:h-16 lg:w-16 shrink-0 aspect-square">
          <Logo name="gldt" className="p-1" />
        </div>
      </form>
    </div>
  );
};

export default InputGLDTAmount;
