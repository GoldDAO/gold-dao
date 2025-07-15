import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import useFetchSwapAmount from "@shared/hooks/useFetchSwapAmount";
// import BalanceAvailable from "../balance-available";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import Dialog from "@shared/ui/dialog/DialogV2";
import Icon from "@shared/ui/icons";
import { LoaderSpin, Logo } from "@components/index";
import BtnPrimary from "@shared/ui/button/BtnPrimary";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";

const AmountUSD = ({
  className,
  amount,
}: {
  className?: string;
  amount: number;
}) => {
  return (
    <div className={className}>
      <div className="text-content/60 text-sm xl:text-base">
        ≈ $
        <NumberToLocaleString value={amount} decimals={5} />
      </div>
    </div>
  );
};

const ConfirmDialog = () => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const balanceTokenFrom = useFetchLedgerBalance(
    swapState.token_from.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.token_from.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const balanceTokenTo = useFetchLedgerBalance(
    swapState.token_to.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.token_to.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const swapAmount = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: swapState.token_from.token.name,
      from_canister_id: swapState.token_from.token.canister_id,
      to: swapState.token_to.token.name,
      amount: Number(swapState.send_amount_input),
      enabled: !!unauthenticatedAgent,
    }
  );

  const onClose = () => {
    dispatchSwapState({
      type: "RESET",
    });
  };

  const onConfirm = ({
    slippage_with_tx_fee,
    max_slippage,
  }: {
    slippage_with_tx_fee: number;
    max_slippage: number;
  }) => {
    if (slippage_with_tx_fee <= max_slippage) {
      dispatchSwapState({ type: "CONFIRM" });
    } else {
      dispatchSwapState({ type: "OPEN_DIALOG_CONFIRM_HIGH_SLIPPAGE" });
    }
  };

  return (
    <Dialog open={swapState.is_open_confirm_dialog} onClose={onClose}>
      {balanceTokenFrom.isSuccess &&
      swapAmount.isSuccess &&
      balanceTokenTo.isSuccess ? (
        <>
          <div className="flex items-center justify-between mb-4">
            <Icon.Chevron
              width={16}
              className="rotate-90"
              onClick={() => dispatchSwapState({ type: "BACK_DIALOG_CONFIRM" })}
            />
            <Dialog.CloseBtn onClick={onClose} />
          </div>
          <div className="flex flex-col gap-4 mt-4">
            <div className="rounded-xl bg-surface-secondary border border-border">
              <div className="p-4 xl:p-6 border-b border-border">
                <div className="text-sm mb-4 text-content/60">You pay</div>
                <div className="flex flex-row justify-between items-center xl:items-end">
                  <div className="flex items-center gap-2">
                    <Logo
                      name={swapState.token_from.token.id}
                      className="h-10 w-10 shrink-0 aspect-square"
                    />
                    <div>
                      <div className="flex items-center gap-2 text-2xl xl:text-4xl">
                        <E8sToLocaleString
                          value={swapAmount.data.pay_amount}
                          tokenDecimals={
                            balanceTokenFrom.data.decimals as number
                          }
                          decimals={5}
                        />
                        <div>{swapState.token_from.token.name}</div>
                      </div>
                      <AmountUSD
                        amount={
                          Number(swapAmount.data.pay_amount) /
                          10 ** balanceTokenFrom.data.decimals
                        }
                        className="block xl:hidden"
                      />
                    </div>
                  </div>
                  <AmountUSD
                    amount={
                      (Number(swapAmount.data.pay_amount) /
                        10 ** balanceTokenFrom.data.decimals) *
                      balanceTokenFrom.data.price_usd
                    }
                    className="hidden xl:block"
                  />
                </div>
              </div>
              <div className="p-4 xl:p-6">
                <div className="flex items-center gap-2 mb-4">
                  <div className="text-sm text-content/60">
                    You receive approximately
                  </div>

                  <Icon.InfoCircle
                    width={16}
                    data-tooltip-id="tooltip"
                    data-tooltip-content={`The exact amount of ${swapState.token_to.token.name} received will vary due to market
                    fluctuations and slippage.`}
                  />
                </div>

                <div className="flex flex-row justify-between items-center xl:items-end">
                  <div className="flex items-center gap-2">
                    <Logo
                      name={swapState.token_to.token.id}
                      className="h-10 w-10 shrink-0 aspect-square"
                    />
                    <div>
                      <div className="flex items-center gap-2 text-2xl xl:text-4xl">
                        <E8sToLocaleString
                          value={swapAmount.data.receive_amount}
                          tokenDecimals={balanceTokenTo.data.decimals as number}
                          decimals={5}
                        />
                        <div>{swapState.token_to.token.name}</div>
                      </div>
                      <AmountUSD
                        amount={
                          (Number(swapAmount.data.receive_amount) /
                            10 ** balanceTokenTo.data.decimals) *
                          balanceTokenTo.data.price_usd
                        }
                        className="block xl:hidden"
                      />
                    </div>
                  </div>
                  <AmountUSD
                    amount={
                      (Number(swapAmount.data.receive_amount) /
                        10 ** balanceTokenTo.data.decimals) *
                      balanceTokenTo.data.price_usd
                    }
                    className="hidden xl:block"
                  />
                </div>
              </div>
            </div>

            <div className="rounded-xl border border-border p-4 xl:p-6">
              <div className="mb-4">Transaction details</div>
              <div className="flex flex-col gap-4">
                <div className="flex justify-between items-center px-2">
                  <div className="text-content/60">Slippage</div>
                  <div className="text-content/60">
                    <NumberToLocaleString
                      value={swapAmount.data.slippage_without_tx_fee}
                    />
                    %
                  </div>
                </div>
                <div className="flex justify-between items-center px-2">
                  <div className="text-content/60">Slippage incl. TX fee</div>
                  <div className="flex items-center gap-1">
                    {swapAmount.data.slippage_with_tx_fee >
                      swapState.max_slippage && (
                      <Icon.Warning
                        width={20}
                        className="text-warning"
                        data-tooltip-id="tooltip"
                        data-tooltip-html={
                          "Warning: Current slippage is large than the recommended limit of 5%. The price impact of your purchase is quite significant."
                        }
                      />
                    )}
                    <div className="text-content/60">
                      <NumberToLocaleString
                        value={swapAmount.data.slippage_with_tx_fee}
                      />
                      %
                    </div>
                  </div>
                </div>
                <div className="flex justify-between items-center px-2">
                  <div className="flex items-center gap-1">
                    <div className="text-content/60">Max slippage</div>
                    <Icon.InfoCircle
                      width={16}
                      data-tooltip-id="tooltip"
                      data-tooltip-html={
                        "Slippage is the difference between the expected price of a trade and the price at which it's executed.<br />The system will allow slippages up to 5% and will ask you for confirmation if the slippage is higher."
                      }
                    />
                  </div>

                  <div className="text-content/60">
                    {swapState.max_slippage}%
                  </div>
                </div>
                <div>
                  <div className="flex justify-between items-center px-2">
                    <div className="text-content/60">Fees</div>
                    {balanceTokenTo.data.decimals &&
                    swapAmount.data.network_fee &&
                    swapAmount.data.lp_fee ? (
                      <>
                        <E8sToLocaleString
                          value={
                            swapAmount.data.network_fee + swapAmount.data.lp_fee
                          }
                          tokenDecimals={balanceTokenTo.data.decimals}
                          decimals={5}
                        />{" "}
                        {swapState.token_to.token.name}
                      </>
                    ) : (
                      <div>Loading...</div>
                    )}
                  </div>
                  <div className="mt-4 text-content/60 text-sm flex flex-col gap-4 border border-border rounded-md bg-surface-secondary p-4">
                    <div className="flex justify-between items-center">
                      <div>Network fee</div>
                      {balanceTokenTo.data.decimals &&
                      swapAmount.data.network_fee ? (
                        <>
                          <E8sToLocaleString
                            value={swapAmount.data.network_fee}
                            tokenDecimals={balanceTokenTo.data.decimals}
                            decimals={5}
                          />{" "}
                          {swapState.token_to.token.name}
                        </>
                      ) : (
                        <div>Loading...</div>
                      )}
                    </div>
                    <div className="border-t border-border"></div>
                    <div className="flex justify-between items-center">
                      <div>LP fee</div>
                      {balanceTokenTo.data.decimals &&
                      swapAmount.data.lp_fee ? (
                        <>
                          <E8sToLocaleString
                            value={swapAmount.data.lp_fee}
                            tokenDecimals={balanceTokenTo.data.decimals}
                            decimals={5}
                          />{" "}
                          {swapState.token_to.token.name}
                        </>
                      ) : (
                        <div>Loading...</div>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <BtnPrimary
              onClick={() =>
                onConfirm({
                  slippage_with_tx_fee: swapAmount.data.slippage_with_tx_fee,
                  max_slippage: swapState.max_slippage,
                })
              }
              className="w-full"
              size="lg"
            >
              Confirm {swapState.token_to.token.name} purchase
            </BtnPrimary>
            <div className="flex justify-center">
              <div className="flex items-center gap-1 text-content/60 text-sm">
                In partnership with
                <a
                  href="https://www.kongswap.io/"
                  target="_blank"
                  rel="noopener noreferrer"
                  className={`flex items-center text-content hover:text-gold`}
                >
                  <div>KongSwap</div>
                  <Icon.ExternalLink width={16} className="ml-2" />
                </a>
              </div>
            </div>
          </div>
        </>
      ) : (
        <div className="flex items-center justify-center my-8">
          <LoaderSpin />
        </div>
      )}
    </Dialog>
  );
};

export default ConfirmDialog;
