import { ReactNode, useEffect } from "react";
import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import { FieldValues, useForm, useWatch } from "react-hook-form";
import { ErrorMessage } from "@hookform/error-message";
import { useAuth } from "@auth/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import { Logo, Button } from "@components/index";
import { TokenSelectedAtom } from "@wallet/atoms/WalletAtom";
import { SendTokenStateAtom } from "@wallet/atoms/TransferTokenAtom";
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchTokenData from "@hooks/useFetchTokenData";
import ICRCAccount from "./components/form/input/ICRCAccount";
import PrincipalAndSubaccount from "./components/form/input/PrincipalAndSubaccount";
import ICRCAccountOrAccountId from "./components/form/input/ICRCAccountOrAccountId";

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

  const balance = useFetchUserBalance(token.canisterId, unauthenticatedAgent, {
    ledger: token.id,
    owner: principalId,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const tokenData = useFetchTokenData(unauthenticatedAgent, {
    token: token.id,
    token_canister_id: token.canisterId,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  useEffect(() => {
    if (amount_input !== "") {
      setValue("amount", amount_input, {
        shouldValidate: true,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

<<<<<<< HEAD
  // const isValidSendAddress = (principal: string, subaccount?: string) => {
  //   try {
  //     if (!is_use_icrc_account) {
  //       const encoded = encodeIcrcAccount({
  //         owner: Principal.fromText(principal),
  //         subaccount: subaccount ? Buffer.from(subaccount, "hex") : [],
  //       });
  //       decodeIcrcAccount(encoded);
  //       setIsValidAddress(null);
  //     } else {
  //       if (
  //         !isValidAccount(principal) &&
  //         !isValidPrincipalOrICRCAccount(principal)
  //       ) {
  //         setIsValidAddress(
  //           "Invalid ICRC account or account ID or principal ID"
  //         );
  //         return;
  //       }
  //       // decodeIcrcAccount(principal);
  //     }
  //     setIsValidAddress(null);
  //   } catch (err: unknown) {
  //     setIsValidAddress(err instanceof Error ? err.message : String(err));
  //   }
  // };

=======
>>>>>>> transfer-send_dfx
  if (!balance.isSuccess || !tokenData.isSuccess) {
    return (
      <div className="flex justify-center items-center px-4 py-16 xl:py-32">
        Loading...
      </div>
    );
  }

  const getAmount = () => {
    const value = Number(watchedAmount);
    if (isNaN(value) || value === 0 || errors?.amount) {
      return 0n;
    } else {
      const amountValue = BigInt(
        Math.round(value * 10 ** tokenData.data.decimals)
      );
      if (amountValue + tokenData.data.fee_e8s !== balance.data)
        return amountValue - tokenData.data.fee_e8s;
      else return amountValue;
    }
  };

  const isAmountBelowBalance = (value: string) => {
    const amount =
      BigInt(Math.round(parseFloat(value) * 10 ** tokenData.data.decimals)) +
      tokenData.data.fee_e8s;
    if (amount > balance.data) return false;
    return true;
  };

  const isAmountAboveFee = (value: string) => {
    const amount = BigInt(
      Math.round(parseFloat(value) * 10 ** tokenData.data.decimals)
    );
    if (amount <= tokenData.data.fee_e8s) return false;
    return true;
  };

  const handleOnClickMaxBalance = () => {
    const amount = balance.data - tokenData.data.fee_e8s;
    setValue(
      "amount",
      amount > 0 ? Number(amount) / 10 ** tokenData.data.decimals : 0,
      {
        shouldValidate: true,
      }
    );
    setFocus("principal");
  };

  const handleOnSubmit = (data: FieldValues) => {
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
          Send <span className="font-semibold text-primary">{token.name}</span>
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
              <Button
                onClick={handleSwitchAddress}
                className="w-full px-4 py-4 border border-primary bg-primary/10 text-primary font-medium rounded-md"
              >
                {!is_use_icrc_account
                  ? "Use ICRC Account"
                  : "Use Principal & Subaccount"}
              </Button>
            </div>
          </div>
          <ErrorMessage
            errors={error_message_receive_address}
            name="principal"
            as="div"
            className="text-red-600 text-sm mt-1"
          />
        </div>
        <div>
          <div className="text-primary text-sm mb-2">Amount</div>
          <InputCard>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="flex items-center justify-center rounded-full bg-surface-secondary h-6 w-6 shrink-0 aspect-square">
                  <Logo name={token.id} />
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
              <button
                onClick={handleOnClickMaxBalance}
                type="button"
                className="rounded-md py-1 px-2 bg-surface-primary text-content/60 border border-border text-xs disabled:cursor-not-allowed cursor-pointer"
                data-tooltip-id="tooltip"
                data-tooltip-html="Max selects your balance minus network fees,<br>ensuring your transaction completes successfully."
              >
                Max
              </button>
            </div>
            <div className="text-content/40 text-sm mt-2 ml-1">
              $
              <NumberToLocaleString
                value={Number(watchedAmount * tokenData.data.price_usd)}
              />
            </div>
          </InputCard>
          {errors && (
            <p className="text-red-600 text-sm mt-1">
              {typeof errors?.amount?.message === "string" &&
                errors.amount.message}
            </p>
          )}
        </div>
        <div className="flex items-center justify-between">
          <div className="inline-flex text-content/60 items-center gap-1 text-sm bg-surface-secondary rounded-md px-2 py-1">
            <div className="text-content/40">Available:</div>
            <TokenValueToLocaleString
              value={balance.data}
              tokenDecimals={tokenData.data.decimals}
            />
            <div>{token.name}</div>
          </div>
          <div className="inline-flex text-content/60 items-center gap-1 text-sm">
            <div className="text-content/40">Fee:</div>
            <NumberToLocaleString value={tokenData.data.fee} />
            <div>{token.name}</div>
            <div className="text-content/40">
              ≈ $
              <NumberToLocaleString value={tokenData.data.fee_usd} />
            </div>
          </div>
        </div>
      </div>

      <div className="mt-8">
        <Button
          type="submit"
          disabled={!isValid || !is_valid_receive_address}
          className="w-full px-6 py-3 bg-secondary text-white font-medium rounded-md"
        >
          Transfer
        </Button>
      </div>
    </form>
  );
};

export default Form;
