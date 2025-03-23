import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";

import { KONGSWAP_CANISTER_ID_IC, GLDT_VALUE_1G_NFT } from "@constants";

import BuyGLDTStateAtom from "./atoms";

import { useAuth } from "@auth/index";

import ImgBuyGold from "@assets/img-buy-gold-section.svg";

import { Button } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

import InnerAppLayout from "@components/outlets/InnerAppLayout";

import SelectBuyMethod from "./SelectToken.component";
import { Token } from "./tokensList.utils";
import InputGLDTAmount from "./InputGLDTAmount.component";
import TradeConfirm from "./TradeConfirm.component";
import TradeDetails from "./TradeDetails.component";

import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTokenPrice from "@services/kongswap/hooks/useFetchTokenPrice";

const BuyGLDT = () => {
  const { principalId, authenticatedAgent, isConnected } = useAuth();
  const [buyAtomState, setBuyAtomstate] = useAtom(BuyGLDTStateAtom);
  const {
    pay_token,
    pay_amount,
    receive_amount,
    pay_token_decimals,
    is_open_confirm_dialog,
    is_open_details_dialog,
  } = buyAtomState;

  const balance = useFetchUserBalance(
    pay_token.canisterId,
    authenticatedAgent,
    {
      ledger: pay_token.id,
      owner: principalId,
      enabled: !!authenticatedAgent && !!isConnected,
    }
  );

  const midPrice = useFetchTokenPrice(
    KONGSWAP_CANISTER_ID_IC,
    authenticatedAgent,
    {
      from: "GLDT",
      to: pay_token.name,
      amount: receive_amount,
      enabled: !!authenticatedAgent && !!isConnected,
    }
  );

  const price = useFetchTokenPrice(
    KONGSWAP_CANISTER_ID_IC,
    authenticatedAgent,
    {
      from: pay_token.name,
      to: "GLDT",
      amount: (midPrice.data?.mid_price ?? 0) * receive_amount,
      enabled: !!authenticatedAgent && !!isConnected && !!midPrice.isSuccess,
    }
  );

  const decimals = useFetchDecimals(pay_token.canisterId, authenticatedAgent, {
    ledger: pay_token.id,
    enabled: !!authenticatedAgent && !!isConnected,
  });

  const handleOnChangeReceiveAmount = (receive_amount: number) => {
    setBuyAtomstate((state) => ({
      ...state,
      receive_amount,
    }));
  };

  const handleOnChangePayToken = (pay_token: Token) => {
    setBuyAtomstate((state) => ({
      ...state,
      pay_token,
    }));
  };

  const handleOpenConfirmDialog = () => {
    setBuyAtomstate((state) => ({
      ...state,
      is_open_confirm_dialog: true,
    }));
  };

  const handleCloseConfirmDialog = () => {
    setBuyAtomstate((state) => ({
      ...state,
      is_open_confirm_dialog: false,
    }));
  };

  useEffect(
    () => {
      if (midPrice.isSuccess && price.isSuccess && decimals.isSuccess) {
        setBuyAtomstate((state) => ({
          ...state,
          pay_amount: BigInt(
            BigInt(
              Math.round(
                midPrice.data?.mid_price * receive_amount * 1.01 * 10e7
              )
            ) +
              BigInt(
                Math.round(
                  midPrice.data?.mid_price *
                    receive_amount *
                    (price.data?.slippage / 100) *
                    10e7
                )
              )
          ),
          pay_token_decimals: decimals.data,
        }));
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [
      midPrice.isSuccess,
      midPrice.data,
      price.isSuccess,
      price.data,
      decimals.isSuccess,
      decimals.data,
    ]
  );

  useEffect(
    () => {
      if (balance.isSuccess) {
        setBuyAtomstate((state) => ({
          ...state,
          pay_token_user_balance: balance.data,
        }));
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [balance.isSuccess, balance.data]
  );

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col h-full">
          <div className="text-left text-2xl lg:text-4xl flex flex-row gap-2 lg:flex-col lg:gap-0 font-semibold">
            <div>Buy Gold</div>
            <div className="text-primary">with no cost</div>
          </div>
          <div className="hidden lg:flex lg:justify-center w-full my-4 lg:my-12">
            <img className="max-w-48" src={ImgBuyGold} alt="Buy Gold" />
          </div>
          <div className="hidden lg:block text-left text-sm lg:text-base text-content/60 mt-2">
            Purchase GLDT, tokenised gold, with each token fully backed by
            physical gold reserves securely stored in Swiss vaults at a ratio of
            100 GLDT to 1 gram. Everything fully transparent and accessible to
            anyone, anywhere.
          </div>
        </div>
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div className="px-4 lg:px-8 pt-4 lg:pt-16 pb-4 lg:pb-8">
          <div
            className={clsx(
              "max-w-2xl mx-auto p-4 lg:p-8 bg-surface-primary border border-border rounded-xl",
              "flex flex-col gap-12 lg:gap-24 items-center text-center"
            )}
          >
            <div>
              <div className="mb-8">Buy GLDT</div>
              <InputGLDTAmount handleOnChange={handleOnChangeReceiveAmount} />
              <div className="font-semibold text-lg lg:text-2xl mt-1">
                {receive_amount / GLDT_VALUE_1G_NFT}g of gold
              </div>

              <div
                className={clsx(
                  "flex flex-col lg:flex-row items-center gap-2 lg:gap-4 mt-8"
                )}
              >
                <div className="shrink-0">Purchase with:</div>
                <SelectBuyMethod
                  className="w-80"
                  value={pay_token}
                  handleOnChange={handleOnChangePayToken}
                />
              </div>
            </div>

            <div className="w-full">
              {pay_amount && pay_token_decimals && receive_amount > 0 ? (
                <>
                  <Button
                    className="w-full px-4 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
                    onClick={handleOpenConfirmDialog}
                  >
                    Buy for ≈{" "}
                    <TokenValueToLocaleString
                      value={pay_amount}
                      tokenDecimals={pay_token_decimals}
                      decimals={5}
                    />{" "}
                    {pay_token.name}
                  </Button>
                </>
              ) : (
                <Button
                  disabled={true}
                  className="w-full px-4 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
                >
                  Buy for ≈ 0 {pay_token.name}
                </Button>
              )}

              <div className="mt-4 text-sm lg:text-base text-content/40">
                Estimated price 1 GLDT ≈ (todo)$
              </div>
            </div>
            {/* Dialogs */}
            <>
              <Dialog
                open={is_open_confirm_dialog}
                handleOnClose={handleCloseConfirmDialog}
                title="Confirm Trade"
              >
                <TradeConfirm />
              </Dialog>
              <Dialog
                open={is_open_details_dialog}
                handleOnClose={() =>
                  setBuyAtomstate((state) => ({
                    ...state,
                    is_open_details_dialog: false,
                  }))
                }
                title="Trade Details"
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
