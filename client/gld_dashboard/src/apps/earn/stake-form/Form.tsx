import { useEffect, useState } from "react";
import { InfoCircle } from "iconsax-react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useForm } from "react-hook-form";
import { StakeStateReducerAtom } from "./atoms";
import { Logo } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import { MIN_STAKE_AMOUNT } from "./utils";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const Form = ({
  balance,
  fee,
  decimals,
  className,
}: {
  balance: bigint;
  fee: bigint;
  decimals: number;
  className?: string;
}) => {
  const [stakeState, dispatch] = useAtom(StakeStateReducerAtom);
  const [isOpenInfoUnlockDelayDialog, setIsOpenInfoUnlockDelayDialog] =
    useState(false);

  const {
    register,
    handleSubmit,
    setValue,
    formState: { isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
    defaultValues: {
      amount: MIN_STAKE_AMOUNT,
    },
  });

  useEffect(() => {
    if (stakeState.amount === undefined) {
      setValue("amount", MIN_STAKE_AMOUNT, { shouldValidate: true });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stakeState.amount]);

  const isAmountBelowBalance = (value: number) => {
    const amount = BigInt(Math.round(value * 10 ** decimals));
    if (amount > balance) return false;
    return true;
  };

  // const handleOnClickMaxBalance = () => {
  //   setValue("amount", Number(balance) / 10 ** decimals, {
  //     shouldValidate: true,
  //   });
  // };

  const handleOnSubmit = (data: { amount: number }) => {
    dispatch({
      type: "SUBMIT",
      value: {
        amount: BigInt(Math.round(Number(data.amount) * 10 ** decimals)),
        fee,
      },
    });
  };

  return (
    <>
      <form onSubmit={handleSubmit(handleOnSubmit)} className={className}>
        <div className="text-gold font-semibold">Stake GLDT</div>
        <div className="p-4 mt-4 xl:mt-6 flex justify-center items-center gap-2 bg-surface-secondary border border-border rounded-md">
          <input
            id="amount"
            type="number"
            autoComplete="off"
            placeholder="0.00"
            step="any"
            autoFocus={true}
            className={clsx(
              "field-sizing-content max-w-56 text-right outline-none focus:outline-none focus:border-none focus:ring-0 bg-surface-secondary",
              "xl:text-xl font-semibold",
              "placeholder:text-content/60 placeholder:xl:text-xl placeholder:font-semibold",
              "[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            )}
            {...register("amount", {
              pattern: /[0-9.]/,
              setValueAs: (v) => Number(v),
              required: "Amount is required",
              min: { value: 10, message: "Amount must be greater than 10" },
              validate: {
                isAmountBelowBalance: (v: number) =>
                  isAmountBelowBalance(v) ||
                  "Amount must not exceed your balance",
              },
            })}
            onKeyDown={(e: React.KeyboardEvent<HTMLInputElement>) => {
              if (["-", "+", "e", "Enter"].includes(e.key)) {
                e.preventDefault();
              }
            }}
            onPaste={(e: React.ClipboardEvent<HTMLInputElement>) => {
              const clipboardData = e.clipboardData;
              const pastedData = clipboardData.getData("text");
              if (/[+\-e]/.test(pastedData)) e.preventDefault();
            }}
          />

          <div className={clsx("xl:text-xl font-semibold text-accent")}>
            GLDT
          </div>

          <div className="flex items-center justify-center rounded-full bg-surface-secondary h-10 w-10 shrink-0 aspect-square">
            <Logo name="gldt" className="p-1" />
          </div>
        </div>

        <div className="my-4 inline-flex flex-col gap-2">
          <div className="flex items-center gap-2 px-2 py-1 bg-surface-secondary rounded-md">
            <div className="text-content/60 text-sm">
              Min Stake: {MIN_STAKE_AMOUNT} GLDT
            </div>
            <Logo name="gldt" className="w-4 h-4" />
          </div>
          <div className="flex items-center gap-2 px-2 py-1 bg-surface-secondary rounded-md">
            <div className="text-content/60 text-sm">Unlock Delay: 1 week</div>
            <InfoCircle
              size={16}
              className="cursor-pointer"
              onClick={() => setIsOpenInfoUnlockDelayDialog(true)}
            />
          </div>
        </div>
        <BtnPrimary className="w-full" type="submit" disabled={!isValid}>
          Stake GLDT
        </BtnPrimary>
      </form>
      <Dialog
        open={isOpenInfoUnlockDelayDialog}
        handleOnClose={() => setIsOpenInfoUnlockDelayDialog(false)}
      >
        <div className="p-4 text-center">
          <div className="font-semibold text-lg mb-4">Unlock delay</div>
          <div className="text-content/60 mb-8">
            When unlocking GLDT, the tokens are locked for 1 week without
            rewards before they can be withdrawn.
          </div>
          <div className="flex justify-end">
            <BtnPrimary onClick={() => setIsOpenInfoUnlockDelayDialog(false)}>
              Close
            </BtnPrimary>
          </div>
        </div>
      </Dialog>
    </>
  );
};

export default Form;
