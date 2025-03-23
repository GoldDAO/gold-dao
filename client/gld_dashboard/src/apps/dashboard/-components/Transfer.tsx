import { useEffect, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { FieldValues, useForm, useWatch } from "react-hook-form";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { CheckCircleIcon, XCircleIcon } from "@heroicons/react/24/outline";
import clsx from "clsx";

import { useAuth } from "@auth/index";

import E8sToLocaleString from "@components/numbers/E8sToLocaleString";
import { LoaderSpin } from "@components/loaders";
import { Logo } from "@components/index";
import { divideBy1e8 } from "@utils/numbers";

import { Ledger } from "@services/ledger/utils/interfaces";
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchTransferFee from "@services/ledger/hooks/useFetchTransferFee";
import useTransfer from "@services/ledger/hooks/useTransfer";

const Form = ({
  form,
  transfer,
  token,
  transferFee,
  balance,
}: {
  form: ReturnType<typeof useForm>;
  transfer: ReturnType<typeof useTransfer>;
  token: Ledger;
  transferFee: bigint;
  balance: bigint;
}) => {
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    control,
    reset: resetFrom,
    setFocus,
    setValue,
    formState: { errors, isValid },
  } = form;
  const [isSubmitting, setIsSubmitting] = useState(false);
  const queryClient = useQueryClient();

  const isAmountBelowBalance = (value: string) => {
    const amount = BigInt(Math.round(parseFloat(value) * 1e8)) + transferFee;
    if (amount > balance) return false;
    return true;
  };

  const isAmountAboveFee = (value: string) => {
    const amount = BigInt(Math.round(parseFloat(value) * 1e8));
    if (amount <= transferFee) return false;
    return true;
  };

  const [amount, setAmount] = useState<bigint>(0n);
  const watchedAmount = useWatch({
    control,
    name: "amount",
    defaultValue: "",
  });
  const watchedAddress = useWatch({
    control,
    name: "recipient_address",
    defaultValue: "",
  });

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
    const amount = balance - transferFee;
    setValue("amount", amount > 0 ? divideBy1e8(amount) : 0.0, {
      shouldValidate: true,
    });
    setFocus("recipient_address");
  };

  const handleOnSubmit = (data: FieldValues) => {
    transfer.mutate(
      { amount, to: data.recipient_address },
      {
        onSuccess: () => {
          queryClient.invalidateQueries({
            queryKey: [
              `USER_FETCH_LEDGER_BALANCE_${token.toLocaleUpperCase()}`,
            ],
          });
        },
      }
    );
  };

  const handleConfirmTransfer = () => {
    setIsSubmitting(true);
  };

  const handleCancelTransfer = () => {
    setIsSubmitting(false);
  };

  const resetTransfer = () => {
    resetFrom();
    transfer.reset();
    setIsSubmitting(false);
  };

  const handleBackToAccount = () => {
    resetTransfer();
    navigate("/dashboard/account");
  };

  useEffect(() => {
    const value = Number(watchedAmount);
    if (isNaN(value) || value === 0 || errors?.amount) {
      setAmount(0n);
    } else {
      const amountValue = BigInt(Math.round(value * 1e8));
      if (amountValue + transferFee !== balance)
        setAmount(amountValue - transferFee);
      else setAmount(amountValue);
    }
  }, [watchedAmount, errors?.amount, transferFee, balance]);

  return (
    <>
      {transfer.isIdle && (
        <form onSubmit={handleSubmit(handleOnSubmit)}>
          <div>
            <div>
              <div className="flex justify-end items-start mb-8">
                <div className="flex items-center gap-2">
                  <div className="flex items-center gap-1 text-content/60 font-semibold text-sm">
                    <div>Balance:</div>
                    <E8sToLocaleString value={balance} />
                    <div>{token.toLocaleUpperCase()}</div>
                  </div>
                  <button
                    onClick={handleOnClickMaxBalance}
                    disabled={isSubmitting}
                    type="button"
                    className="rounded-md py-1 px-2 bg-accent-2 text-white text-xs font-semibold hover:bg-accent-2/90 disabled:cursor-not-allowed"
                    data-tooltip-id="tooltip"
                    data-tooltip-html="Max selects your balance minus network fees,<br>ensuring your transaction completes successfully."
                  >
                    MAX
                  </button>
                </div>
              </div>

              <div className="flex items-center gap-4">
                <div className="flex items-center justify-center rounded-full bg-surface-secondary h-16 w-16 shrink-0 aspect-square">
                  <Logo name={token} className="p-1" />
                </div>
                <input
                  id="amount"
                  type="text"
                  disabled={isSubmitting}
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
            </div>

            <div>
              <input
                id="recipient_address"
                type="text"
                autoComplete="off"
                disabled={isSubmitting}
                placeholder="Principal ID"
                className={clsx(
                  "mt-6 w-full border border-border outline-none focus:outline-none focus:ring-0 p-4 rounded-md bg-surface-primary",
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
                {watchedAddress && errors && (
                  <p className="text-red-600 text-sm font-semibold">
                    {typeof errors?.recipient_address?.message === "string" &&
                      errors.recipient_address.message}
                  </p>
                )}
              </div>
            </div>

            <div className="mt-4">
              <div className="flex items-center gap-1 text-sm text-content/60">
                <div>Transfer fee:</div>
                <E8sToLocaleString value={transferFee} decimals={4} />
                <div>{token.toLocaleUpperCase()}</div>
              </div>
              <div className="flex items-center gap-1 text-sm text-content/60 font-semibold">
                <div>Amount received:</div>
                <E8sToLocaleString value={amount} decimals={4} />
                <div>{token.toLocaleUpperCase()}</div>
              </div>
            </div>

            {isSubmitting ? (
              <div className="grid grid-cols-2 mt-8 gap-1">
                <button
                  type="button"
                  onClick={handleCancelTransfer}
                  className="px-6 py-3 rounded-md bg-surface-secondary"
                >
                  Cancel
                </button>

                <button
                  type="submit"
                  disabled={!isValid}
                  className="px-6 py-3 bg-accent-2 text-white rounded-md"
                >
                  Confirm
                </button>
              </div>
            ) : (
              <div className="grid grid-cols-1 mt-8">
                <button
                  type="button"
                  onClick={handleConfirmTransfer}
                  disabled={!isValid}
                  className={clsx(
                    "w-full py-3 rounded-md",
                    "bg-accent-2 text-white",
                    "disabled:opacity-60 cursor-pointer disabled:cursor-not-allowed"
                  )}
                >
                  Transfer
                </button>
              </div>
            )}
          </div>
        </form>
      )}
      {transfer.isPending && (
        <div className="px-4 py-16 flex flex-col justify-center items-center">
          <LoaderSpin />
          <div className="font-semibold text-xl mt-8">
            Transfer is being processed
          </div>
          <div className="text-content/60">This can take a few seconds</div>
        </div>
      )}
      {transfer.isSuccess && (
        <div className="px-4">
          <div className="flex flex-col justify-center items-center mb-4">
            <CheckCircleIcon className="h-24 w-24 text-jade mb-4" />
            <div className="font-semibold text-xl mb-8">
              Transfer was successful !
            </div>
          </div>
          <div className="grid grid-cols-2 gap-1">
            <button
              type="button"
              onClick={handleBackToAccount}
              className="px-6 py-3 rounded-md bg-surface-secondary"
            >
              Back to Account
            </button>

            <button
              type="button"
              onClick={resetTransfer}
              className="px-6 py-3 rounded-md bg-accent-2 text-white"
            >
              New Transfer
            </button>
          </div>
        </div>
      )}
      {transfer.isError && (
        <div className="px-4">
          <div className="flex flex-col justify-center items-center mb-12">
            <XCircleIcon className="h-24 w-24 text-red-600 mb-4" />
            <div className="font-semibold text-xl">Transfer error !</div>
          </div>
          <div className="grid grid-cols-2 gap-1">
            <button
              type="button"
              onClick={handleBackToAccount}
              className="px-6 py-3 rounded-md bg-surface-secondary"
            >
              Back to Account
            </button>

            <button
              type="button"
              onClick={resetTransfer}
              className="px-6 py-3 rounded-md bg-accent-2 text-white"
            >
              Retry Transfer
            </button>
          </div>
        </div>
      )}
    </>
  );
};

const Transfer = ({
  token,
  canisterId,
}: {
  token: Ledger;
  canisterId: string;
}) => {
  const { unauthenticatedAgent, authenticatedAgent, principalId, isConnected } =
    useAuth();

  const form = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });

  const balance = useFetchUserBalance(canisterId, unauthenticatedAgent, {
    ledger: token,
    owner: principalId,
    enabled: !!unauthenticatedAgent && !!isConnected,
  });

  const transferFee = useFetchTransferFee(canisterId, unauthenticatedAgent, {
    ledger: token,
    enabled: !!unauthenticatedAgent,
  });

  const transfer = useTransfer(canisterId, authenticatedAgent);

  useEffect(() => {
    return () => {
      transfer.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <>
      {(balance.isLoading ||
        balance.isError ||
        transferFee.isLoading ||
        transferFee.isError) && (
        <div className="flex items-center justify-center my-8">
          <LoaderSpin size="sm" />
        </div>
      )}
      {balance.isSuccess && transferFee.isSuccess && (
        <Form
          form={form}
          transfer={transfer}
          token={token}
          transferFee={transferFee.data}
          balance={balance.data}
        />
      )}
    </>
  );
};

export default Transfer;
