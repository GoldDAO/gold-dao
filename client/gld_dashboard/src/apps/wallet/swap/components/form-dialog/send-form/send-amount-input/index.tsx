import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import useFetchSwapAmount from "@shared/hooks/useFetchSwapAmount";
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
import isReceiveAmountGreaterThanZero from "@shared/utils/validators/isReceiveAmountGreaterThanZero";
import { isNumeric } from "@shared/utils/numbers";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";

const SendAmountInput = () => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);
  const {
    register,
    control,
    setValue,
    formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    reValidateMode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,

    defaultValues: {
      amount: swapState.send_amount_input || "",
    },
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

  const swapAmount = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: swapState.token_from.token.name,
      from_canister_id: swapState.token_from.token.canister_id,
      to: swapState.token_to.token.name,
      amount: Number(amount),
      enabled: !!unauthenticatedAgent,
    }
  );

  useEffect(() => {
    dispatchSwapState({
      type: "SET_SEND_AMOUNT",
      value: isNumeric(amount) ? amount : "",
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [amount]);

  useEffect(() => {
    if (swapState.send_amount_input === "") return;
    setValue("amount", swapState.send_amount_input, {
      shouldValidate: true,
      shouldDirty: true,
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swapState.send_amount_input, swapState.token_from.token]);

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

  return (
    <input
      id="amount"
      type="number"
      autoComplete="off"
      placeholder="0.00"
      min="0"
      readOnly={!balance.isSuccess}
      className={clsx(
        {
          "animate-pulse cursor-not-allowed": !balance.isSuccess,
        },
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
            if (!balance.isSuccess) return true;
            return (
              isInsufficientFunds(
                Number(v),
                balance.data.balance_e8s,
                balance.data.fee_e8s,
                balance.data.decimals
              ) || "Amount must not exceed your balance minus network fees"
            );
          },
          isAmountGreaterThanFee: (v: string) => {
            if (!balance.isSuccess) return true;
            return (
              isAmountGreaterThanFee(
                Number(v),
                balance.data.fee_e8s,
                balance.data.decimals
              ) || "Amount must not be less or equal than transaction fee"
            );
          },
          isReceiveAmountGreaterThanZero: () => {
            if (!swapAmount.isSuccess) return true;
            return (
              isReceiveAmountGreaterThanZero(swapAmount.data.receive_amount) ||
              "Receive amount is zero, please increase the send amount."
            );
          },
        },
      })}
    />
  );
};

export default SendAmountInput;
