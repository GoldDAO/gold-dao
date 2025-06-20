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
import { BuyGLDTStateReducerAtom } from "@buy/shared/atoms/BuyGLDTAtom";
import { useAuth } from "@auth/index";
import ImgBuyGold from "@assets/img-buy-gold-section.svg";
import {
  onKeyDownPreventNoDigits,
  onPastePreventNoDigits,
} from "@shared/utils/form/input";
import { Logo } from "@components/index";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchSwapAmount from "@services/kongswap/hooks/useFetchSwapAmount";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import GradientCard from "@shared/components/ui/card/GradientCard";
import { Token, TOKEN_LIST_AVAILABLE } from "@buy/shared/utils";
import SelectToken from "@buy/select-token";
import ConfirmDialog from "@buy/confirm-dialog";
import DetailsDialog from "@buy/details-dialog";
import DisclaimerAmountReceivedDialog from "@buy/disclaimer-amount-received-dialog";
import DisclaimerConfirmHighSlippageDialog from "./disclaimer-confirm-high-slippage-dialog";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const Buy = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [buyAtomState, dispatch] = useAtom(BuyGLDTStateReducerAtom);
  const {
    pay_token,
    receive_token,
    slippage,
    network_fee,
    max_slippage,
    lp_fee,
    is_open_confirm_dialog,
    is_open_details_dialog,
    is_open_disclaimer_confirm_high_slippage_dialog,
  } = buyAtomState;
  const [
    openDisclaimerAmountReceivedDialog,
    setOpenDisclaimerAmountReceivedDialog,
  ] = useState(false);

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

  const isBuyEnabled =
    isValid &&
    receiveTokenPrice.isSuccess &&
    !receiveTokenPrice.isFetching &&
    isDataFetched &&
    price.data.receive_amount > 0n;

  const errorReceiveAmountLowerThanZero =
    price.isSuccess &&
    payTokenPrice.isSuccess &&
    payTokenPrice.data.amount > 0n &&
    price.data.receive_amount <= 0n;

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col items-center gap-4 xl:gap-8 text-center xl:text-left xl:items-start">
          <div className="text-4xl xl:text-6xl flex flex-col justify-center items-center xl:items-start">
            <div className="font-semibold text-gold">Buy</div>
            <div>Tokenized Gold</div>
          </div>

          <div className="hidden xl:flex xl:justify-center w-full">
            <img className="max-w-48" src={ImgBuyGold} alt="Buy Gold" />
          </div>

          <div>
            <div className="font-semibold text-content/70">
              The Simplest Way to Own Physical Gold.
            </div>

            <div className="mt-2 flex flex-col gap-3 text-content/60">
              <div>
                GLDT removes the complexity of owning gold. Each gold token
                (GLDT) represents real, physical gold secured in a Swiss vault.
              </div>
              <div className="flex items-center justify-center xl:justify-start">
                <Logo name="gldt" className="h-6 w-6 mr-2" />
                <div className="font-semibold">100 GLDT = 1 gram of Gold</div>
              </div>
              <div>
                Buy, sell, or hold a timeless store of value with the ease of a
                digital asset, unlocking its full potential.
              </div>
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
            <div className="w-full px-4 xl:px-8 pt-8 xl:pt-12 pb-8 xl:pb-12 border-b border-border">
              <div className="mb-4 text-xl xl:text-4xl">
                Buy GLDT{" "}
                <span className="text-gold font-semibold">Gold Tokens</span>
              </div>
              <div className="inline-flex items-center text-sm text-content/60 border border-border rounded-full px-4 py-2">
                <Logo name="gldt" className="h-5 w-5 mr-1" />
                100 GLDT = 1 gram of physical gold
              </div>
              <div className="grid grid-cols-1 xl:grid-cols-7 gap-4 mt-8">
                <div className="xl:col-span-4 flex items-center border border-border rounded-md grow bg-surface-secondary">
                  <div className="p-4 border-r border-border text-copper font-semibold">
                    Pay with
                  </div>
                  <div className="p-4">
                    {!isConnected && (
                      <form
                        className="flex justify-center items-center gap-2"
                        onSubmit={(e) => e.preventDefault()}
                      >
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
                        <form
                          className="flex justify-center items-center gap-2"
                          onSubmit={(e) => e.preventDefault()}
                        >
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

                <div className="xl:col-span-3">
                  <SelectToken
                    tokens={TOKEN_LIST_AVAILABLE}
                    value={pay_token.token}
                    handleOnChange={handleOnChangePayToken}
                  />
                </div>
              </div>
            </div>

            <GradientCard
              className={clsx(
                "w-full px-4 xl:px-8 pt-8 xl:pt-12 pb-4 xl:pb-8",
                "rounded-b-[inherit]"
              )}
            >
              <div className="text-copper font-semibold">
                You will receive approximately
              </div>
              <div className="mt-4">
                <div className="text-2xl xl:text-4xl">
                  {isReceiveTokenPriceIsFetched ? (
                    <div className="inline-flex items-center gap-2">
                      <E8sToLocaleString
                        value={receiveTokenPrice.data.amount}
                        tokenDecimals={receiveTokenPrice.data.decimals}
                        decimals={5}
                      />{" "}
                      GLDT
                      <InfoCircle
                        size={16}
                        className="cursor-pointer"
                        onClick={() =>
                          setOpenDisclaimerAmountReceivedDialog(true)
                        }
                      />
                    </div>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
                <div className="font-semibold text-lg xl:text-xl mt-1">
                  {isReceiveTokenPriceIsFetched ? (
                    <>
                      <E8sToLocaleString
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
                      <E8sToLocaleString
                        value={payTokenPriceExchangeRate.data.amount}
                        tokenDecimals={payTokenPriceExchangeRate.data.decimals}
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
                    <BtnPrimary
                      className="w-full"
                      size="lg"
                      onClick={() => dispatch({ type: "OPEN_DIALOG_CONFIRM" })}
                      disabled={!isBuyEnabled}
                    >
                      {isReceiveTokenPriceIsFetched ? (
                        <>
                          Buy ≈{" "}
                          <E8sToLocaleString
                            value={receiveTokenPrice.data.amount}
                            tokenDecimals={receiveTokenPrice.data.decimals}
                            decimals={5}
                          />{" "}
                          GLDT
                        </>
                      ) : (
                        <div>Loading...</div>
                      )}
                    </BtnPrimary>
                    {errors.amount && errors.amount?.message !== "" && (
                      <div className="mt-2 text-danger">
                        {errors?.amount?.message as string}
                      </div>
                    )}
                    {errorReceiveAmountLowerThanZero && (
                      <div className="mt-2 text-danger">
                        Receive amount is too low. Please increase it a bit
                      </div>
                    )}
                  </>
                ) : (
                  <BtnConnectWallet className="w-full" size="lg" />
                )}
              </div>
            </GradientCard>
            <>
              <DisclaimerAmountReceivedDialog
                open={openDisclaimerAmountReceivedDialog}
                handleClose={() => setOpenDisclaimerAmountReceivedDialog(false)}
              />

              {isBuyEnabled && (
                <>
                  <ConfirmDialog
                    open={is_open_confirm_dialog}
                    handleClose={() => dispatch({ type: "CANCEL" })}
                    handleConfirm={
                      slippage <= max_slippage
                        ? () => dispatch({ type: "CONFIRM" })
                        : () =>
                            dispatch({
                              type: "OPEN_CONFIRM_HIGH_SLIPPAGE",
                            })
                    }
                    payToken={pay_token}
                    receiveToken={receive_token}
                    slippage={slippage}
                    maxSlippage={max_slippage}
                    networkFee={network_fee}
                    lpFee={lp_fee}
                  />

                  <DisclaimerConfirmHighSlippageDialog
                    open={is_open_disclaimer_confirm_high_slippage_dialog}
                    handleClose={() => dispatch({ type: "CANCEL" })}
                    handleConfirm={() =>
                      dispatch({
                        type: "CONFIRM_HIGH_SLIPPAGE",
                        value: { slippage },
                      })
                    }
                    slippage={slippage}
                    maxSlippage={max_slippage}
                  />

                  <DetailsDialog
                    open={is_open_details_dialog}
                    handleClose={() => dispatch({ type: "RESET" })}
                    payToken={pay_token}
                    receiveToken={receive_token}
                    maxSlippage={max_slippage}
                  />
                </>
              )}
            </>
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Buy;
