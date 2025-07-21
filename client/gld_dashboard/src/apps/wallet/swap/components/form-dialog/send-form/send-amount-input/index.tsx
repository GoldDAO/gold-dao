import { useEffect } from "react";
import { useAtom } from "jotai";
import { useDebounceValue } from "usehooks-ts";
import { useAuth } from "@auth/index";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import useFetchSwapAmount from "@shared/hooks/useFetchSwapAmount";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import clsx from "clsx";
import { useForm } from "react-hook-form";
import {
  onKeyDownPreventNoDigits,
  onPastePreventNoDigits,
} from "@shared/utils/form/input";
import isInsufficientFunds from "@shared/utils/validators/isInsufficientFunds";
import isAmountGreaterThanZero from "@shared/utils/validators/isAmountGreaterThanZero";
import isAmountGreaterThanFee from "@shared/utils/validators/isAmountGreaterThanFee";
import { isNumeric } from "@shared/utils/numbers";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";

const SendAmountInput = () => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);
  const {
    register,
    setValue,
    formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
    defaultValues: {
      amount: swapState.send_amount_input,
    },
  });
  const [debouncedAmount, setDebouncedAmount] = useDebounceValue("", 500);

  const balance = useFetchLedgerBalance(
    swapState.send_token.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.send_token.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const swapAmount = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: swapState.send_token.token.name,
      from_canister_id: swapState.send_token.token.canister_id,
      to: swapState.receive_token.token.name,
      amount: Number(swapState.send_amount_input),
      key: "swap",
      enabled: !!unauthenticatedAgent,
    }
  );

  useEffect(() => {
    if (swapState.send_amount_input === "") return;
    setValue("amount", swapState.send_amount_input, {
      shouldValidate: true,
      shouldDirty: true,
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swapState.send_amount_input]);

  useEffect(() => {
    dispatchSwapState({
      type: "SET_FORM_STATE",
      value: {
        errors,
        isValid,
      },
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isValid, errors, swapState.send_amount_input]);

  useEffect(() => {
    if (balance.isSuccess) {
      register("amount", {
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
          isAmountGreaterThanFee: (v: string) => {
            return (
              isAmountGreaterThanFee(
                Number(v),
                balance.data.fee_e8s,
                balance.data.decimals
              ) || "Amount must not be less or equal than transaction fee"
            );
          },
        },
      });
    }
  }, [balance.isSuccess, balance.data, register]);

  useEffect(() => {
    if (swapAmount.isSuccess) {
      const {
        receive_amount,
        slippage_without_tx_fee,
        slippage_with_tx_fee,
        network_fee,
        lp_fee,
      } = swapAmount.data;
      dispatchSwapState({
        type: "SET_TX_DATA",
        value: {
          receive_amount,
          slippage_without_tx_fee,
          slippage_with_tx_fee,
          network_fee,
          lp_fee,
        },
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swapAmount.isSuccess, swapAmount.data]);

  useEffect(() => {
    dispatchSwapState({
      type: "SET_SEND_AMOUNT",
      value: debouncedAmount,
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [debouncedAmount]);

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
        required: true,
        onChange: (e) => {
          setDebouncedAmount(e.target.value);
        },
      })}
    />
  );
};

export default SendAmountInput;
