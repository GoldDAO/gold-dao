import { useEffect } from "react";
import { useAtom } from "jotai";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import clsx from "clsx";
import { useForm, useWatch } from "react-hook-form";
import {
  onKeyDownPreventNoDigits,
  onPastePreventNoDigits,
} from "@shared/utils/form/input";
import useSendAmountInputValidation from "@wallet/swap/components/form-dialog/send-form/send-amount-input/hooks/useSendAmountInputValidation";
import { isNumeric } from "@shared/utils/numbers";

const SendAmountInput = ({
  initialValue = "",
  balance,
  fee,
  decimals,
}: {
  balance: bigint;
  fee: bigint;
  decimals: number;
  initialValue: string;
}) => {
  const [, dispatchSwapState] = useAtom(SwapStateReducerAtom);
  const {
    register,
    reset,
    control,
    // formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });

  const {
    isInsufficientFunds,
    isAmountGreaterThanFee,
    isAmountGreaterThanZero,
  } = useSendAmountInputValidation(balance, fee, decimals);

  const amount = useWatch({
    control,
    name: "amount",
  }) as string;

  useEffect(() => {
    reset({
      amount: initialValue,
    });
  }, [reset, initialValue]);

  useEffect(() => {
    dispatchSwapState({
      type: "SET_SEND_AMOUNT",
      value: isNumeric(amount) ? amount : "0",
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [amount]);

  return (
    <input
      id="amount"
      type="number"
      autoComplete="off"
      placeholder="0.00"
      className={clsx(
        "field-sizing-content max-w-42 text-left outline-none focus:outline-none focus:border-none focus:ring-0 bg-surface-secondary",
        "placeholder:text-content/40",
        "[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
      )}
      onPaste={onPastePreventNoDigits}
      onKeyDown={(e) => {
        onKeyDownPreventNoDigits(e);
      }}
      {...register("amount", {
        pattern: /[0-9.]/,
        required: "",
        validate: {
          isInsufficientFunds: (v: string) => {
            return (
              isInsufficientFunds(Number(v)) ||
              "Amount must not exceed your balance minus network fees"
            );
          },
          isAmountGreaterThanFee: (v: string) =>
            isAmountGreaterThanFee(Number(v)) ||
            "Amount must not be less or equal than transaction fee",
          isAmountGreaterThanZero: (v: string) =>
            isAmountGreaterThanZero(Number(v)) || "",
        },
      })}
    />
  );
};

export default SendAmountInput;
