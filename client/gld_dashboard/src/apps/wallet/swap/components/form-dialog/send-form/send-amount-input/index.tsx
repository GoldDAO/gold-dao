import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import clsx from "clsx";
import { useForm, useWatch } from "react-hook-form";
import {
  onKeyDownPreventNoDigits,
  onPastePreventNoDigits,
} from "@shared/utils/form/input";
import isInsufficientFunds from "@shared/utils/validators/isInsufficientFunds";
import isAmountGreaterThanZero from "@shared/utils/validators/isAmountGreaterThanZero";
import isAmountGreaterThanFee from "@shared/utils/validators/isAmountGreaterThanFee";
import { isNumeric } from "@shared/utils/numbers";

const SendAmountInput = ({ initialValue = "" }: { initialValue: string }) => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);
  const {
    register,
    control,
    formState: { errors, isValid },
    setValue,
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });

  const balance = useFetchLedgerBalance(
    swapState.token_from.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.token_from.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const amount = useWatch({
    control,
    name: "amount",
  }) as string;

  useEffect(() => {
    setValue("amount", initialValue, {
      shouldValidate: true,
      shouldDirty: true,
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    dispatchSwapState({
      type: "SET_SEND_AMOUNT",
      value: isNumeric(amount) ? amount : "0",
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [amount]);

  useEffect(() => {
    dispatchSwapState({
      type: "SET_FORM_STATE",
      value: {
        errors,
        isValid,
      },
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isValid, errors]);

  if (!balance.isSuccess) {
    return (
      <input
        className="animate-pulse cursor-not-allowed"
        value="0.00"
        readOnly
      />
    );
  }

  return (
    <input
      id="amount"
      type="number"
      autoComplete="off"
      placeholder="0.00"
      min="0"
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
          isNumericAmount: (v: string) => isNumeric(v),
          isAmountGreaterThanZero: (v: string) =>
            isAmountGreaterThanZero(Number(v)),
          isInsufficientFunds: (v: string) => {
            return (
              isInsufficientFunds(
                Number(v),
                balance.data.balance_e8s,
                balance.data.fee_e8s,
                balance.data.decimals
              ) || "Amount must not exceed your balance minus network fees"
            );
          },
          isAmountGreaterThanFee: (v: string) =>
            isAmountGreaterThanFee(
              Number(v),
              balance.data.fee_e8s,
              balance.data.decimals
            ) || "Amount must not be less or equal than transaction fee",
        },
      })}
    />
  );
};

export default SendAmountInput;
