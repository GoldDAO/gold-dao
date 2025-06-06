import { useEffect, useState } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useForm, useWatch } from "react-hook-form";
import { InfoCircle } from "iconsax-react";
import {
  KONGSWAP_CANISTER_ID_IC,
  GLDT_LEDGER_CANISTER_ID,
  GLDT_VALUE_1G_NFT,
} from "@constants";
import { BuyGLDTStateReducerAtom } from "@buy/atoms/BuyGLDTAtom";
import { useAuth } from "@auth/index";
import ImgBuyGold from "@assets/img-buy-gold-section.svg";
import {
  onKeyDownPreventNoDigits,
  onPastePreventNoDigits,
} from "@shared/utils/form/input";
import { Button, Logo } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import { Token } from "./utils";
import SelectToken from "./components/select-token/SelectToken";
import BuyConfirm from "./components/buy-dialog/Confirm";
import BuyDetails from "./components/buy-dialog/Details";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchSwapAmount from "@services/kongswap/hooks/useFetchSwapAmount";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import GradientCard from "@shared/components/ui/card/GradientCard";

const Buy = () => {
  const { principalId, unauthenticatedAgent, isConnected, connect } = useAuth();
  const [buyAtomState, dispatch] = useAtom(BuyGLDTStateReducerAtom);
  const {
    pay_token,
    receive_token,
    is_open_confirm_dialog,
    is_open_details_dialog,
  } = buyAtomState;
  const [isOpenInfoUnlockDelayDialog, setIsOpenInfoUnlockDelayDialog] =
    useState(false);

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
  }) as number;

  const balance = useFetchLedgerBalance(
    pay_token.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: pay_token.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const payTokenDecimals = useFetchDecimals(
    pay_token.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: pay_token.token.id,
      enabled: !!unauthenticatedAgent,
    }
  );

  const price = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: pay_token.token.name,
      to: "GLDT",
      amount: amount
        ? BigInt(Math.round(amount * 10 ** (payTokenDecimals?.data ?? 0)))
        : 0n,
      enabled: !!unauthenticatedAgent,
    }
  );

  const priceExchangeRate = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: pay_token.token.name,
      to: "GLDT",
      amount: BigInt(10 ** (payTokenDecimals?.data ?? 0)),
      enabled: !!unauthenticatedAgent,
    }
  );

  const payTokenPrice = useFetchTokenPrice(unauthenticatedAgent, {
    from: pay_token.token.name,
    from_canister_id: pay_token.token.canisterId,
    amount: price.data?.pay_amount ?? 0n,
    enabled: !!unauthenticatedAgent && price.isSuccess,
  });

  const payTokenPriceExchangeRate = useFetchTokenPrice(unauthenticatedAgent, {
    from: "GLDT",
    from_canister_id: GLDT_LEDGER_CANISTER_ID,
    amount: priceExchangeRate.data?.receive_amount ?? 0n,
    enabled: !!unauthenticatedAgent && priceExchangeRate.isSuccess,
  });

  const receiveTokenPrice = useFetchTokenPrice(unauthenticatedAgent, {
    from: "GLDT",
    from_canister_id: GLDT_LEDGER_CANISTER_ID,
    amount: price.data?.receive_amount ?? 0n,
    enabled: !!unauthenticatedAgent && price.isSuccess,
  });

  useEffect(() => {
    if (price.isSuccess && receiveTokenPrice.isSuccess && amount > 0) {
      dispatch({
        type: "SET_PRICE_DATA",
        value: {
          slippage: price.data.slippage,
          txs: price.data.txs,
          receive_token_amount: price.data.receive_amount,
        },
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [price.isSuccess, price.data, receiveTokenPrice.isSuccess, isConnected]);

  useEffect(
    () => {
      if (receiveTokenPrice.isSuccess) {
        dispatch({
          type: "SET_RECEIVE_TOKEN_DATA",
          value: {
            amount: receiveTokenPrice.data.amount,
            amount_usd: receiveTokenPrice.data.amount_usd,
            amount_gold:
              receiveTokenPrice.data.amount / BigInt(GLDT_VALUE_1G_NFT),
            decimals: receiveTokenPrice.data.decimals,
            fee: receiveTokenPrice.data.fee,
          },
        });
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [receiveTokenPrice.isSuccess, receiveTokenPrice.data, isConnected]
  );

  useEffect(
    () => {
      if (balance.isSuccess && payTokenPrice.isSuccess) {
        dispatch({
          type: "SET_PAY_TOKEN_DATA",
          value: {
            amount: payTokenPrice.data.amount,
            amount_usd: payTokenPrice.data.amount_usd,
            decimals: payTokenPrice.data.decimals,
            user_balance: balance.data.balance_e8s,
            fee: payTokenPrice.data.fee,
          },
        });
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [
      balance.isSuccess,
      payTokenPrice.isSuccess,
      payTokenPrice.data,
      isConnected,
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

  const isInsufficientFunds = (
    value: number,
    balance: bigint,
    fee: bigint,
    decimals: number
  ) => {
    if (value === 0) return true;
    return BigInt(Math.round(value * 10 ** decimals)) + fee <= balance;
  };

  const isAmountGreaterThanFee = (
    value: number,
    fee: bigint,
    decimals: number
  ) => {
    if (value === 0) return true;
    return BigInt(Math.round(value * 10 ** decimals)) >= fee;
  };

  const isAmountGreaterThanZero = (value: number) => value > 0;

  const isDataFetched =
    balance.isSuccess &&
    price.isSuccess &&
    receiveTokenPrice.isSuccess &&
    payTokenPrice.isSuccess;

  const isReceiveTokenPriceIsFetched =
    receiveTokenPrice.isSuccess && !receiveTokenPrice.isFetching;

  const isDisabledBuyButton =
    !isValid ||
    !receiveTokenPrice.isSuccess ||
    !isDataFetched ||
    receiveTokenPrice.isFetching ||
    price.data.receive_amount <= 0n;

  const errorReceiveAmountLowerThanZero =
    price.isSuccess &&
    payTokenPrice.isSuccess &&
    payTokenPrice.data.amount > 0n &&
    price.data.receive_amount <= 0n;

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col items-center justify-between text-center xl:text-left xl:items-start h-full px-4 xl:px-8">
          <div className="text-5xl xl:text-6xl flex flex-col">
            <div className="font-semibold text-primary/90">Buy</div>
            <div className="flex xl:flex-col gap-2 xl:gap-0 font-light">
              <div>Tokenized</div>
              <div>Gold</div>
            </div>
          </div>
          <div className="hidden xl:flex xl:justify-center w-full my-4">
            <img className="max-w-58" src={ImgBuyGold} alt="Buy Gold" />
          </div>
          <div className="mt-3">
            <div className="font-semibold text-content/70">
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
        <div className="p-4 xl:p-8 my-auto">
          <div
            className={clsx(
              "max-w-3xl mx-auto bg-surface-primary border border-border rounded-xl",
              "flex flex-col items-center text-center"
            )}
          >
            <div className="w-full px-4 xl:px-8 pt-8 xl:pt-12 pb-8 xl:pb-12">
              <div className="mb-4 text-xl xl:text-4xl">
                Buy GLDT <span className="text-primary">Gold Tokens</span>
              </div>
              <div className="inline-flex items-center text-sm text-content/60 border border-border rounded-full px-4 py-2">
                <Logo name="gldt" className="h-5 w-5 mr-1" />
                100 GLDT = 1 gram of physical gold
              </div>
              <div className="flex flex-col xl:flex-row gap-4 mt-8">
                <div className="flex items-center border border-border rounded-md grow bg-surface-secondary">
                  <div className="p-4 border-r border-border text-primary">
                    Pay with
                  </div>
                  <div className="p-4">
                    {!isConnected && (
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
                          onPaste={onPastePreventNoDigits}
                          onKeyDown={(e) => {
                            onKeyDownPreventNoDigits(e);
                          }}
                          {...register("amount", {
                            pattern: /[0-9.]/,
                            required: "",
                            validate: {
                              isAmountGreaterThanZero: (v: string) =>
                                isAmountGreaterThanZero(Number(v)) || "",
                            },
                          })}
                        />
                        <div>{pay_token.token.name}</div>
                        <div className="flex items-center justify-center rounded-full h-6 w-6 shrink-0 aspect-square">
                          <Logo name={pay_token.token.id} className="p-1" />
                        </div>
                      </form>
                    )}
                    {isConnected &&
                      (isDataFetched ? (
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
                            onPaste={onPastePreventNoDigits}
                            onKeyDown={(e) => {
                              onKeyDownPreventNoDigits(e);
                            }}
                            {...register("amount", {
                              pattern: /[0-9.]/,
                              required: "",
                              validate: {
                                isInsufficientFunds: (v: string) => {
                                  return (
                                    isInsufficientFunds(
                                      Number(v),
                                      balance.data.balance_e8s,
                                      payTokenPrice.data.fee,
                                      payTokenPrice.data.decimals
                                    ) ||
                                    "Amount must not exceed your balance minus network fees"
                                  );
                                },
                                isAmountGreaterThanFee: (v: string) =>
                                  isAmountGreaterThanFee(
                                    Number(v),
                                    payTokenPrice.data.fee,
                                    payTokenPrice.data.decimals
                                  ) ||
                                  "Amount must not be less or equal than transaction fee",
                                isAmountGreaterThanZero: (v: string) =>
                                  isAmountGreaterThanZero(Number(v)) || "",
                              },
                            })}
                          />
                          <div>{pay_token.token.name}</div>
                          <div className="flex items-center justify-center rounded-full h-6 w-6 shrink-0 aspect-square">
                            <Logo name={pay_token.token.id} className="p-1" />
                          </div>
                        </form>
                      ) : (
                        <div>Loading...</div>
                      ))}
                  </div>
                </div>

                <SelectToken
                  value={pay_token.token}
                  handleOnChange={handleOnChangePayToken}
                />
              </div>
            </div>

            <GradientCard
              className={clsx(
                "w-full px-4 xl:px-8 pt-8 xl:pt-12 pb-4 xl:pb-8",
                "rounded-b-[inherit]"
              )}
            >
              <div className="text-primary">You will receive</div>
              <div className="mt-4">
                <div className="text-2xl xl:text-4xl">
                  {isReceiveTokenPriceIsFetched ? (
                    <div className="inline-flex items-center gap-2">
                      <TokenValueToLocaleString
                        value={receiveTokenPrice.data.amount}
                        tokenDecimals={receiveTokenPrice.data.decimals}
                        decimals={5}
                      />{" "}
                      GLDT
                      <InfoCircle
                        size={16}
                        className="cursor-pointer"
                        onClick={() => setIsOpenInfoUnlockDelayDialog(true)}
                      />
                    </div>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
                <div className="font-semibold text-lg xl:text-xl mt-1">
                  {isReceiveTokenPriceIsFetched ? (
                    <>
                      ≈{" "}
                      <TokenValueToLocaleString
                        value={
                          receiveTokenPrice.data.amount /
                          BigInt(GLDT_VALUE_1G_NFT)
                        }
                        tokenDecimals={receiveTokenPrice.data.decimals}
                        decimals={5}
                      />
                      g of gold{" "}
                      <span className="text-content/60 font-normal">
                        ($
                        <NumberToLocaleString
                          value={receiveTokenPrice.data.amount_usd}
                        />
                        )
                      </span>
                    </>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
                <div className="bg-surface-secondary mt-8 inline-flex flex-col xl:flex-row gap-1 xl:gap-2 text-sm text-content/60 border border-border rounded-xl xl:rounded-full px-6 py-2">
                  <div>Current exchange rate:</div>

                  {payTokenPriceExchangeRate.isSuccess ? (
                    <div className="flex items-center gap-1">
                      <Logo name={pay_token.token.id} className="h-4 w-4" />
                      <div>1</div>
                      <div>{pay_token.token.name}</div>
                      <div>=</div>
                      <TokenValueToLocaleString
                        value={payTokenPriceExchangeRate.data.amount}
                        tokenDecimals={payTokenPriceExchangeRate.data.decimals}
                        decimals={2}
                      />
                      <div>{receive_token.token.name}</div>
                      <Logo name={receive_token.token.id} className="h-4 w-4" />
                    </div>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
              </div>

              <div className="mt-8 xl:mt-12">
                {isConnected ? (
                  <>
                    <Button
                      className="w-full px-4 py-3 bg-secondary text-white xl:text-lg font-medium rounded-md"
                      onClick={() => dispatch({ type: "OPEN_DIALOG_CONFIRM" })}
                      disabled={isDisabledBuyButton}
                    >
                      {isReceiveTokenPriceIsFetched ? (
                        <>
                          Buy for ≈{" "}
                          <TokenValueToLocaleString
                            value={receiveTokenPrice.data.amount}
                            tokenDecimals={receiveTokenPrice.data.decimals}
                            decimals={5}
                          />{" "}
                          GLDT
                        </>
                      ) : (
                        <div>Loading...</div>
                      )}
                    </Button>
                    {errors.amount && errors.amount?.message !== "" && (
                      <div className="mt-2 text-red-500">
                        {errors?.amount?.message as string}
                      </div>
                    )}
                    {errorReceiveAmountLowerThanZero && (
                      <div className="mt-2 text-red-500">
                        Receive amount is too low. Please increase it a bit
                      </div>
                    )}
                  </>
                ) : (
                  <Button
                    className="w-full px-4 py-3 bg-secondary text-white xl:text-lg font-medium rounded-md"
                    onClick={connect}
                  >
                    Connect Wallet
                  </Button>
                )}
              </div>
            </GradientCard>
            <>
              <Dialog
                open={is_open_confirm_dialog}
                handleOnClose={() => dispatch({ type: "CANCEL" })}
                title="Confirm Purchase"
              >
                <BuyConfirm />
              </Dialog>
              <Dialog
                open={is_open_details_dialog}
                handleOnClose={() => dispatch({ type: "OPEN_DIALOG_DETAILS" })}
              >
                <BuyDetails />
              </Dialog>

              <Dialog
                open={isOpenInfoUnlockDelayDialog}
                handleOnClose={() => setIsOpenInfoUnlockDelayDialog(false)}
              >
                <div className="p-4 text-center">
                  <div className="font-semibold text-lg mb-4">
                    Receive amount
                  </div>
                  <div className="text-content/60 mb-8">
                    The exact amount of GLDT received will vary due to market
                    fluctuations and slippage.
                  </div>
                  <div className="flex justify-end">
                    <Button
                      className="px-6 py-2 bg-secondary text-white rounded-full"
                      onClick={() => setIsOpenInfoUnlockDelayDialog(false)}
                    >
                      Close
                    </Button>
                  </div>
                </div>
              </Dialog>
            </>
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Buy;
