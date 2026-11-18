import { ReactNode, useEffect } from "react";
import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import { FieldValues, useForm, useWatch } from "react-hook-form";
import { ErrorMessage } from "@hookform/error-message";
import { useAuth } from "@auth/index";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import TokenLogo from "@shared/components/token-logo";
import { formatUnits, parseUnits } from "@shared/utils/numbers/units";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import { SendTokenStateAtom } from "@wallet/shared/atoms/TransferTokenAtom";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import ICRCAccount from "./form-input/ICRCAccount";
import PrincipalAndSubaccount from "./form-input/PrincipalAndSubaccount";
import ICRCAccountOrAccountId from "./form-input/ICRCAccountOrAccountId";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import MaxButton from "@shared/components/MaxButton";
import BalanceAvailable from "@shared/components/BalanceAvailable";

const InputCard = ({ children }: { children: ReactNode }) => {
  return (
    <div className="p-4 border border-border rounded-lg bg-surface-secondary">
      {children}
    </div>
  );
};

const Form = ({ className }: { className?: string }) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const token = useAtomValue(TokenSelectedAtom);
  const [sendState, setSendState] = useAtom(SendTokenStateAtom);
  const {
    amount_input,
    is_use_icrc_account,
    is_valid_receive_address,
    error_message_receive_address,
  } = sendState;

  const {
    register,
    handleSubmit,
    control,
    setFocus,
    setValue,
    // reset,
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

  const balance = useFetchLedgerBalance(
    token.canister_id,
    unauthenticatedAgent,
    {
      ledger: token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  useEffect(() => {
    if (amount_input !== "") {
      setValue("amount", amount_input, {
        shouldValidate: true,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  if (!balance.isSuccess) {
    return (
      <div className="flex justify-center items-center px-4 py-16 xl:py-32">
        Loading...
      </div>
    );
  }

  // The typed amount in the token's smallest unit, or null when the text is
  // not a number. Converted on the digits, so it is exact for any number of
  // decimals.
  const toUnits = (value: string | number) =>
    parseUnits(String(value), balance.data.decimals);

  const getAmount = () => {
    const amountValue = toUnits(watchedAmount);
    if (amountValue === null || amountValue === 0n || errors?.amount) {
      return 0n;
    } else {
      if (amountValue + balance.data.fee_e8s !== balance.data.balance_e8s)
        return amountValue - balance.data.fee_e8s;
      else return amountValue;
    }
  };

  const isAmountBelowBalance = (value: string) => {
    const amount = toUnits(value);
    if (amount === null) return false;
    if (amount + balance.data.fee_e8s > balance.data.balance_e8s) return false;
    return true;
  };

  const isAmountAboveFee = (value: string) => {
    const amount = toUnits(value);
    if (amount === null) return false;
    if (amount <= balance.data.fee_e8s) return false;
    return true;
  };

  const onClickMaxBalance = () => {
    const amount = formatUnits(
      balance.data.balance_e8s - balance.data.fee_e8s,
      balance.data.decimals
    );
    setValue("amount", amount, {
      shouldValidate: true,
    });
    setFocus("principal");
  };

  const handleOnSubmit = (data: FieldValues) => {
    // The Transfer button is disabled without a valid recipient, but a form
    // can be submitted in other ways.
    if (!is_valid_receive_address) return;
    setSendState((state) => ({
      ...state,
      amount_input: data.amount,
      amount: getAmount(),
      is_step_send_form: false,
      is_step_send_confirm: true,
      is_send_confirm: true,
    }));
  };

  const handleSwitchAddress = () => {
    setSendState((state) => ({
      ...state,
      is_use_icrc_account: !is_use_icrc_account,
    }));
  };

  return (
    <form onSubmit={handleSubmit(handleOnSubmit)} className={className}>
      <div className="flex justify-center mb-8">
        <div className="text-4xl">
          Send <span className="font-semibold text-gold">{token.name}</span>
        </div>
      </div>
      <div className="flex flex-col gap-4 mt-4">
        <div className="w-full">
          <div className="flex flex-col md:flex-row md:items-end gap-4">
            {is_use_icrc_account ? (
              <div className="w-full">
                {token.id === "icp" ? (
                  <ICRCAccountOrAccountId />
                ) : (
                  <ICRCAccount />
                )}
              </div>
            ) : (
              <div className="w-full">
                <PrincipalAndSubaccount />
              </div>
            )}
            <div className="shrink-0">
              {/* type="button": without it this is the form's submit button,
                  and switching the input also sent the form. */}
              <button
                type="button"
                onClick={handleSwitchAddress}
                className="w-full px-6 py-3 border border-gold bg-gold/10 font-semibold text-gold text-base rounded-xl"
              >
                {!is_use_icrc_account
                  ? "Use ICRC Account"
                  : "Use Principal & Subaccount"}
              </button>
            </div>
          </div>
          <ErrorMessage
            errors={error_message_receive_address}
            name="principal"
            as="div"
            className="text-danger text-sm mt-1"
          />
        </div>
        <div>
          <div className="text-copper text-sm font-semibold mb-2">Amount</div>
          <InputCard>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="flex items-center justify-center rounded-full bg-surface-secondary h-6 w-6 shrink-0 aspect-square">
                  <TokenLogo token={token} className="h-6 w-6" />
                </div>
                <input
                  id="amount"
                  type="text"
                  autoComplete="off"
                  placeholder={`0 ${token.name}`}
                  className={clsx(
                    "w-full outline-none focus:outline-none focus:border-none focus:ring-0 bg-surface-secondary",
                    "placeholder:text-content/40"
                  )}
                  {...register("amount", {
                    pattern: /[0-9.]/,
                    required: "Amount is required",
                    validate: {
                      isValidAmount: (v: string) =>
                        toUnits(v) !== null || "Enter a valid amount",
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
              <MaxButton
                disabled={balance.data.balance_e8s < balance.data.fee_e8s}
                handleOnClick={onClickMaxBalance}
                data-tooltip-id="tooltip"
                data-tooltip-html="Max selects your balance minus network fees,<br>ensuring your transaction completes successfully."
              />
            </div>
            {balance.data.has_price && (
              <div className="text-content/40 text-sm mt-2 ml-1">
                $
                <NumberToLocaleString
                  value={Number(watchedAmount * balance.data.price_usd)}
                />
              </div>
            )}
          </InputCard>
          {errors && (
            <p className="text-danger text-sm mt-1">
              {typeof errors?.amount?.message === "string" &&
                errors.amount.message}
            </p>
          )}
        </div>
        <div className="flex flex-col xl:flex-row items-center justify-between gap-2 xl:gap-0">
          <div className="text-content/60 text-sm bg-surface-secondary rounded-md px-4 py-1">
            <BalanceAvailable
              token={token.name}
              balance={balance.data?.balance}
            />
          </div>
          <div className="inline-flex text-content/60 items-center gap-1 text-sm">
            <div className="text-content/40">Fee:</div>
            <NumberToLocaleString value={balance.data.fee} decimals={8} />
            <div>{token.name}</div>
            {balance.data.has_price && (
              <div className="text-content/40">
                ≈ $
                <NumberToLocaleString
                  value={balance.data.fee_usd}
                  decimals={8}
                />
              </div>
            )}
          </div>
        </div>
      </div>

      <div className="mt-8">
        <BtnPrimary
          type="submit"
          disabled={!isValid || !is_valid_receive_address}
          className="w-full"
        >
          Transfer
        </BtnPrimary>
      </div>
    </form>
  );
};

export default Form;
