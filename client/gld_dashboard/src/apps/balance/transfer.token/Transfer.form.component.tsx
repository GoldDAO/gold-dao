import { useEffect } from "react";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import { FieldValues, useForm, useWatch } from "react-hook-form";

import { useAuth } from "@auth/index";

import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { Logo, Button } from "@components/index";

import { TokenSelectedAtom } from "../balance.atoms";
import { SendTokenStateAtom } from "./atoms";

import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTransferFee from "@services/ledger/hooks/useFetchTransferFee";

const TransferToken = ({ className }: { className?: string }) => {
  const { authenticatedAgent, principalId, isConnected } = useAuth();

  const token = useAtomValue(TokenSelectedAtom);
  const [sendState, setSendState] = useAtom(SendTokenStateAtom);
  const { amount_input } = sendState;

  const {
    register,
    handleSubmit,
    control,
    setFocus,
    setValue,
    formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });

  const watchedAmount = useWatch({
    control,
    name: "amount",
    defaultValue: "",
  });

  const balance = useFetchUserBalance(token.canisterId, authenticatedAgent, {
    ledger: token.id,
    owner: principalId,
    enabled: !!authenticatedAgent && !!isConnected,
  });

  const fee = useFetchTransferFee(token.canisterId, authenticatedAgent, {
    ledger: token.id,
    enabled: !!authenticatedAgent,
  });

  const decimals = useFetchDecimals(token.canisterId, authenticatedAgent, {
    ledger: token.id,
    enabled: !!authenticatedAgent && !!isConnected,
  });

  useEffect(() => {
    if (fee.isSuccess && decimals.isSuccess) {
      setSendState((state) => ({
        ...state,
        fee: fee.data,
        decimals: decimals.data,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fee.isSuccess, fee.data, decimals.isSuccess, decimals.data]);

  useEffect(() => {
    if (amount_input !== "") {
      setValue("amount", amount_input, {
        shouldValidate: true,
      });
    }
    setValue(
      "recipient_address",
      "oxh25-vm4xh-tmsig-jsjms-3ra3g-jyyqy-nqb2k-swild-u5hfd-qvmmf-qqe",
      { shouldValidate: true }
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  if (!decimals.isSuccess || !balance.isSuccess || !fee.isSuccess) {
    return (
      <div className="flex justify-center items-center px-4 py-16 lg:py-32">
        Loading...
      </div>
    );
  }

  const getAmount = () => {
    const value = Number(watchedAmount);
    if (isNaN(value) || value === 0 || errors?.amount) {
      return 0n;
    } else {
      const amountValue = BigInt(Math.round(value * 10 ** decimals.data));
      if (amountValue + fee.data !== balance.data)
        return amountValue - fee.data;
      else return amountValue;
    }
  };

  const isAmountBelowBalance = (value: string) => {
    const amount =
      BigInt(Math.round(parseFloat(value) * 10 ** decimals.data)) + fee.data;
    if (amount > balance.data) return false;
    return true;
  };

  const isAmountAboveFee = (value: string) => {
    const amount = BigInt(Math.round(parseFloat(value) * 10 ** decimals.data));
    if (amount <= fee.data) return false;
    return true;
  };

  const isValidRecipientAddress = (value: string) => {
    try {
      decodeIcrcAccount(value);
      return true;
    } catch (err) {
      console.error(err);
      return false;
    }
  };

  const handleOnClickMaxBalance = () => {
    const amount = balance.data - fee.data;
    setValue(
      "amount",
      amount > 0 ? Number(amount) / 10 ** decimals.data : 0.0,
      {
        shouldValidate: true,
      }
    );
    setFocus("recipient_address");
  };

  const handleOnSubmit = (data: FieldValues) => {
    setSendState((state) => ({
      ...state,
      amount_input: data.amount,
      amount: getAmount(),
      receive_address: data.recipient_address,
      is_step_send_form: false,
      is_step_send_confirm: true,
    }));
  };

  return (
    <form onSubmit={handleSubmit(handleOnSubmit)} className={className}>
      <div className="flex justify-end items-start mb-8">
        <div className="flex items-center gap-2">
          <div className="flex items-center gap-1 text-content/60 font-semibold text-sm">
            <div>Balance:</div>
            <TokenValueToLocaleString
              value={balance.data}
              tokenDecimals={decimals.data}
            />
            <div>{token.name}</div>
          </div>
          <button
            onClick={handleOnClickMaxBalance}
            type="button"
            className="rounded-md py-1 px-2 bg-secondary text-white text-xs font-semibold hover:bg-secondary/90 disabled:cursor-not-allowed cursor-pointer"
            data-tooltip-id="tooltip"
            data-tooltip-html="Max selects your balance minus network fees,<br>ensuring your transaction completes successfully."
          >
            MAX
          </button>
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex items-center justify-center rounded-full bg-surface-secondary h-16 w-16 shrink-0 aspect-square">
          <Logo name={token.id} className="p-1" />
        </div>
        <input
          id="amount"
          type="text"
          autoComplete="off"
          placeholder="0.00"
          className={clsx(
            "w-full outline-none focus:outline-none focus:border-none focus:ring-0 bg-surface-primary",
            "text-6xl font-semibold",
            "placeholder:text-content/60 placeholder:text-6xl placeholder:font-semibold"
          )}
          {...register("amount", {
            pattern: /[0-9.]/,
            required: "Amount is required",
            validate: {
              isAmountBelowBalance: (v: string) =>
                isAmountBelowBalance(v) ||
                "Amount must not exceed your balance minus network fees",
              isAmountAboveFee: (v: string) =>
                isAmountAboveFee(v) ||
                "Amount must not be less or equal than transaction fee",
            },
          })}
        />
      </div>
      <div className="mt-1 h-8">
        {errors && (
          <p className="text-red-600 text-sm font-semibold">
            {typeof errors?.amount?.message === "string" &&
              errors.amount.message}
          </p>
        )}
      </div>

      <input
        id="recipient_address"
        type="text"
        autoComplete="off"
        placeholder="Principal ID"
        className={clsx(
          "mt-4 w-full border border-border outline-none focus:outline-none focus:ring-0 p-4 rounded-md bg-surface-primary",
          "text-sm font-semibold",
          "placeholder:text-content/60 placeholder:text-sm placeholder:font-semibold"
        )}
        {...register("recipient_address", {
          pattern: /[0-9.]/,
          required: "Recipient address is required",
          validate: {
            isValidRecipientAddress: (v) =>
              isValidRecipientAddress(v) || "Invalid recipient address",
          },
        })}
      />
      <div className="mt-1 h-8 ml-2">
        {errors && (
          <p className="text-red-600 text-sm font-semibold">
            {typeof errors?.recipient_address?.message === "string" &&
              errors.recipient_address.message}
          </p>
        )}
      </div>

      <div className="mt-4">
        <div className="flex items-center gap-1 text-sm text-content/60">
          <div>Transfer fee:</div>
          <TokenValueToLocaleString value={fee.data} decimals={4} />
          <div>{token.name}</div>
        </div>
        <div className="flex items-center gap-1 text-sm text-content/60 font-semibold">
          <div>Amount received:</div>
          <TokenValueToLocaleString
            value={getAmount()}
            tokenDecimals={decimals.data}
            decimals={4}
          />
          <div>{token.name}</div>
        </div>
      </div>

      <div className="mt-8">
        <Button
          type="submit"
          disabled={!isValid}
          className="w-full px-6 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
        >
          Transfer
        </Button>
      </div>
    </form>
  );
};

export default TransferToken;
