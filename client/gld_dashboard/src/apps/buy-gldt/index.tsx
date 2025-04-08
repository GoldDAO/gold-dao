import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useForm, useWatch } from "react-hook-form";

import { KONGSWAP_CANISTER_ID_IC } from "@constants";

import { BuyGLDTStateReducerAtom } from "./atoms";

import { useAuth } from "@auth/index";

import ImgBuyGold from "@assets/img-buy-gold-section.svg";

import { Button, Logo } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

import InnerAppLayout from "@components/outlets/InnerAppLayout";

import SelectToken from "./SelectToken.component";
import { Token } from "./tokensList.utils";
import TradeConfirm from "./TradeConfirm.component";
import TradeDetails from "./TradeDetails.component";

import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchTransferFee from "@services/ledger/hooks/useFetchTransferFee";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTokenPrice from "@services/kongswap/hooks/useFetchTokenPrice";

const BuyGLDT = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [buyAtomState, dispatch] = useAtom(BuyGLDTStateReducerAtom);
  const {
    pay_token,
    receive_token,
    is_open_confirm_dialog,
    is_open_details_dialog,
  } = buyAtomState;

  const {
    register,
    reset,
    control,
    formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });

  const amount = useWatch({
    control,
    name: "amount",
  });

  const balance = useFetchUserBalance(
    pay_token.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: pay_token.token.id,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const price = useFetchTokenPrice(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: pay_token.token.name,
      to: "GLDT",
      amount: amount ?? 0,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const payTokenFee = useFetchTransferFee(
    pay_token.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: pay_token.token.id,
      enabled: !!unauthenticatedAgent,
    }
  );

  const receiveTokenFee = useFetchTransferFee(
    receive_token.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: receive_token.token.id,
      enabled: !!unauthenticatedAgent,
    }
  );

  const payTokenDecimals = useFetchDecimals(
    pay_token.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: pay_token.token.id,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const receiveTokenDecimals = useFetchDecimals(
    receive_token.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: receive_token.token.id,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  useEffect(
    () => {
      if (
        price.isSuccess &&
        receiveTokenDecimals.isSuccess &&
        receiveTokenFee.isSuccess
      ) {
        dispatch({
          type: "SET_RECEIVE_TOKEN_DATA",
          value: {
            amount: price.data.receive_amount,
            decimals: receiveTokenDecimals.data,
            fee: receiveTokenFee.data,
          },
        });
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [
      price.isSuccess,
      price.data,
      receiveTokenDecimals.isSuccess,
      receiveTokenFee.isSuccess,
    ]
  );

  useEffect(
    () => {
      if (
        amount &&
        balance.isSuccess &&
        payTokenDecimals.isSuccess &&
        payTokenFee.isSuccess
      ) {
        dispatch({
          type: "SET_PAY_TOKEN_DATA",
          value: {
            decimals: payTokenDecimals.data,
            user_balance: balance.data,
            fee: payTokenFee.data,
            amount,
          },
        });
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [
      balance.isSuccess,
      payTokenDecimals.isSuccess,
      payTokenFee.isSuccess,
      amount,
    ]
  );

  useEffect(() => {
    if (pay_token.amount === null) {
      reset({
        amount: "",
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [pay_token.amount]);

  const handleOnChangePayToken = (token: Token) => {
    dispatch({ type: "RESET" });
    dispatch({ type: "SET_PAY_TOKEN", value: token });
  };

  const isInsufficientFunds = (value: number) => {
    return (
      BigInt(Math.round(value * 10 ** (payTokenDecimals.data as number))) +
        (payTokenFee.data as bigint) <=
      (balance.data as bigint)
    );
  };

  const isAmountGreaterThanFee = (value: number) => {
    return value === 0
      ? true
      : BigInt(Math.round(value * 10 ** (payTokenDecimals.data as number))) >=
          (payTokenFee.data as bigint);
  };

  const isAmountGreaterThanZero = (value: number) => value > 0;

  const preventNoDigits = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "e" || e.key === "-" || e.key === "+") e.preventDefault();
  };

  const preventPasteNoDigits = (e: React.ClipboardEvent<HTMLInputElement>) => {
    const clipboardData = e.clipboardData;
    const text = clipboardData.getData("text");
    const number = parseFloat(text);
    if (number < 0) e.preventDefault();
    if (text === "e" || text === "+" || text === "-") e.preventDefault();
  };

  const isDataFetched =
    price.isSuccess &&
    balance.isSuccess &&
    payTokenDecimals.isSuccess &&
    payTokenFee.isSuccess;

  const isReceiveTokenDataFetched =
    price.isSuccess && receiveTokenDecimals.isSuccess;

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col items-center text-center lg:text-left lg:items-start h-full">
          <div className="text-5xl lg:text-6xl flex flex-row gap-2 lg:flex-col lg:gap-0 font-semibold mt-4">
            <div>Buy</div>
            <div className="text-primary font-light">tokenized Gold</div>
          </div>
          <div className="hidden lg:flex lg:justify-center w-full my-4 lg:my-12">
            <img className="max-w-48" src={ImgBuyGold} alt="Buy Gold" />
          </div>
          <div className="mt-3">
            <div className="font-semibold">
              Unlock Gold's Potential. Digitally.
            </div>
            <div className="text-content/60 mt-2">
              GLDT revolutionizes gold ownership. Each Gold token (GLDT)
              represents a tangible claim to securely vaulted Swiss gold, 100
              GLDT per gram. Own your future, with complete transparency,
              anywhere.
            </div>
          </div>
        </div>
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div className="p-4 lg:p-8">
          <div
            className={clsx(
              "max-w-3xl mx-auto bg-surface-primary border border-border rounded-xl",
              "flex flex-col items-center text-center"
            )}
          >
            <div className="w-full px-4 lg:px-8 pt-8 lg:pt-16 pb-8 lg:pb-16">
              <div className="mb-4 text-xl lg:text-4xl">
                Buy GLDT <span className="text-primary">Gold Tokens</span>
              </div>
              <div className="mb-2 text-sm text-content/60">
                GLDT is a{" "}
                <span className="text-content">gold-backed token</span> where{" "}
                <span className="text-content">
                  100 GLDT = 1 gram of physical gold
                </span>
              </div>
              <div className="flex flex-col lg:flex-row gap-4 mt-8 lg:mt-12">
                <div className="flex items-center border border-border rounded-md grow bg-surface-secondary">
                  <div className="p-4 border-r border-border text-primary">
                    Pay with
                  </div>
                  <div className="p-4">
                    {isDataFetched ? (
                      <form className="flex justify-center items-center gap-2">
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
                          onPaste={preventPasteNoDigits}
                          onKeyDown={preventNoDigits}
                          {...register("amount", {
                            pattern: /[0-9.]/,
                            required: "",
                            // onChange: (e) =>
                            //   handleOnChangePayTokenAmount(
                            //     Number(e.target.value)
                            //   ),
                            validate: {
                              isInsufficientFunds: (v: string) =>
                                isInsufficientFunds(Number(v)) ||
                                "Amount must not exceed your balance minus network fees",
                              isAmountGreaterThanFee: (v: string) =>
                                isAmountGreaterThanFee(Number(v)) ||
                                "Amount must not be less or equal than transaction fee",
                              isAmountGreaterThanZero: (v: string) =>
                                isAmountGreaterThanZero(Number(v)) || "",
                            },
                          })}
                        />
                        <div className={clsx("")}>{pay_token.token.name}</div>

                        <div className="flex items-center justify-center rounded-full h-6 w-6 shrink-0 aspect-square">
                          <Logo name={pay_token.token.id} className="p-1" />
                        </div>
                      </form>
                    ) : (
                      <div>Loading...</div>
                    )}
                  </div>
                </div>

                <SelectToken
                  value={pay_token.token}
                  handleOnChange={handleOnChangePayToken}
                />
              </div>
            </div>

            <div
              className={clsx(
                "w-full px-4 lg:px-8 pt-8 lg:pt-16 pb-4 lg:pb-8",
                "bg-linear-to-t from-neutral-100 to-background dark:from-neutral-900 dark:to-neutral-800 rounded-tr-[inherit]"
              )}
            >
              <div className="text-primary">You will receive</div>
              <div className="mt-4">
                <div className="text-2xl lg:text-4xl">
                  {isReceiveTokenDataFetched ? (
                    <>
                      <TokenValueToLocaleString
                        value={receive_token.amount}
                        tokenDecimals={receive_token.decimals}
                        decimals={5}
                      />{" "}
                      GLDT
                    </>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
                <div className="font-semibold text-lg lg:text-xl mt-1">
                  {isReceiveTokenDataFetched ? (
                    <>
                      ≈{" "}
                      <TokenValueToLocaleString
                        value={receive_token.amount_in_gold}
                        tokenDecimals={receive_token.decimals}
                        decimals={5}
                      />
                      g of gold
                    </>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
              </div>

              <div className="mt-8 lg:mt-16">
                <Button
                  className="w-full px-4 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
                  onClick={() => dispatch({ type: "OPEN_DIALOG_CONFIRM" })}
                  disabled={!isValid || !isDataFetched}
                >
                  {isReceiveTokenDataFetched ? (
                    <>
                      Buy for ≈{" "}
                      <TokenValueToLocaleString
                        value={receive_token.amount}
                        tokenDecimals={receive_token.decimals}
                        decimals={5}
                      />{" "}
                      GLDT
                    </>
                  ) : (
                    <div>Loading...</div>
                  )}
                </Button>
                {errors.amount && errors.amount?.message !== "" && (
                  <div className="mt-2">
                    {errors?.amount?.message as string}
                  </div>
                )}
              </div>
            </div>
            <>
              <Dialog
                open={is_open_confirm_dialog}
                handleOnClose={() => dispatch({ type: "CANCEL" })}
                title="Confirm Purchase"
              >
                <TradeConfirm />
              </Dialog>
              <Dialog
                open={is_open_details_dialog}
                handleOnClose={() => dispatch({ type: "OPEN_DIALOG_DETAILS" })}
                title="Purchase Details"
              >
                <TradeDetails />
              </Dialog>
            </>
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default BuyGLDT;
