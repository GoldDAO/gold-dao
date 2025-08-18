import { useEffect } from "react";
import { useAtom } from "jotai";
import { useDebounceValue } from "usehooks-ts";
import { MIN_STAKE_AMOUNT, MAX_STAKE_AMOUNT } from "@constants";
import { IncreaseStakeStateReducerAtom } from "@earn/components/increase-stake/atoms";
import clsx from "clsx";
import { useForm } from "react-hook-form";
import {
  onKeyDownPreventNoDigits,
  onPastePreventNoDigits,
} from "@shared/utils/form/input";
import { isNumeric } from "@shared/utils/numbers";
import isInsufficientFunds from "@shared/utils/validators/isInsufficientFunds";
import isAmountGreaterThanFee from "@shared/utils/validators/isAmountGreaterThanFee";
import isAmountGreaterThanZero from "@shared/utils/validators/isAmountGreaterThanZero";

const StakeAmountInput = () => {
  const [stakeState, dispatchStakeState] = useAtom(
    IncreaseStakeStateReducerAtom
  );
  const { user_balance_gldt, user_staked_data } = stakeState;

  const {
    register,
    setValue,
    formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
    defaultValues: { amount: stakeState.stake_amount },
  });
  const [debouncedAmount, setDebouncedAmount] = useDebounceValue(
    stakeState.stake_amount,
    500
  );

  useEffect(() => {
    setValue("amount", stakeState.stake_amount, {
      shouldValidate: true,
      shouldDirty: true,
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stakeState.stake_amount]);

  useEffect(() => {
    dispatchStakeState({
      type: "SET_FORM_STATE",
      value: {
        errors,
        isValid,
      },
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isValid, errors]);

  useEffect(() => {
    dispatchStakeState({
      type: "SET_STAKE_AMOUNT",
      value: debouncedAmount,
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [debouncedAmount]);

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setDebouncedAmount(e.target.value);
  };

  const isAmountGreaterThanMinStakeAmount = (value: string) => {
    if (user_staked_data.staked_amount > 0) {
      return true; // If already staked, no minimum amount required
    }
    return (
      Number(value) >= MIN_STAKE_AMOUNT ||
      `Minimum stake amount is ${MIN_STAKE_AMOUNT} GLDT`
    );
  };

  const isAmountLessThanMaxStakeAmount = (value: string) => {
    const max_amount = MAX_STAKE_AMOUNT - user_staked_data.staked_amount;
    return (
      Number(value) <= max_amount ||
      `Maximum stake amount is ${max_amount} GLDT`
    );
  };

  return (
    <input
      id="amount"
      type="number"
      autoComplete="off"
      placeholder="10.00"
      min="10.00"
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
        required: true,
        onChange,
        validate: {
          isNumericAmount: (v: string) => isNumeric(v),
          isAmountGreaterThanZero: (v: string) =>
            isAmountGreaterThanZero(Number(v)),
          isAmountGreaterThanMinStakeAmount: (v: string) =>
            isAmountGreaterThanMinStakeAmount(v),
          isAmountLessThanMaxStakeAmount: (v: string) =>
            isAmountLessThanMaxStakeAmount(v),
          isInsufficientFunds: (v: string) => {
            return (
              isInsufficientFunds(
                Number(v),
                user_balance_gldt.balance_e8s,
                user_balance_gldt.fee_e8s,
                user_balance_gldt.decimals
              ) || "Amount must not exceed your balance minus network fees"
            );
          },
          isAmountGreaterThanFee: (v: string) =>
            isAmountGreaterThanFee(
              Number(v),
              user_balance_gldt.fee_e8s,
              user_balance_gldt.decimals
            ) || "Amount must be greater than network fees",
        },
      })}
    />
  );
};

export default StakeAmountInput;
