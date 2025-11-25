import { useEffect } from "react";
import { useAtom } from "jotai";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";
import { useAuth } from "@auth/index";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import Dialog from "@shared/ui/dialog/DialogV2";
import { LoaderSpin, Logo } from "@components/index";
import useApprove from "@shared/hooks/useApproveLedger";
import useSwap from "@services/kongswap/hooks/useSwap";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import useFetchSwapAmount from "@shared/hooks/useFetchSwapAmount";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const DetailsDialog = () => {
  const { authenticatedAgent, principalId, unauthenticatedAgent } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const balanceTokenFrom = useFetchLedgerBalance(
    swapState.send_token.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.send_token.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const balanceTokenTo = useFetchLedgerBalance(
    swapState.receive_token.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.receive_token.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const swapAmount = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: swapState.send_token.token.name,
      from_canister_id: swapState.send_token.token.canister_id,
      to: swapState.receive_token.token.name,
      amount: Number(swapState.send_amount_input),
      enabled: !!unauthenticatedAgent,
    }
  );

  const approve = useApprove(
    swapState.send_token.token.canister_id,
    authenticatedAgent
  );
  const swap = useSwap(KONGSWAP_CANISTER_ID_IC, authenticatedAgent, {
    pay_token: swapState.send_token.token.name,
    receive_token: swapState.receive_token.token.name,
  });

  const handleSwap = (pay_amount: bigint) => {
    swap.mutate({
      pay_amount: pay_amount,
      receive_address: principalId,
      max_slippage: swapState.max_slippage,
    });
  };

  useEffect(() => {
    if (
      swapState.is_open_details_dialog &&
      approve.isIdle &&
      swapAmount.isSuccess &&
      balanceTokenFrom.isSuccess
    ) {
      // console.log(swapAmount.data);
      // console.log("pay_amount", swapAmount.data.pay_amount);
      // console.log("balance_e8s", balanceTokenFrom.data.balance_e8s);
      // console.log("fee_e8s", balanceTokenFrom.data.fee_e8s);
      // console.log(
      //   "amount",
      //   swapAmount.data.pay_amount + balanceTokenFrom.data.fee_e8s
      // );
      approve.mutate(
        {
          amount: swapAmount.data.pay_amount + balanceTokenFrom.data.fee_e8s,
          spender: { owner: KONGSWAP_CANISTER_ID_IC },
        },
        {
          onSuccess: () => {
            handleSwap(swapAmount.data.pay_amount);
          },
        }
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    swapState.is_open_details_dialog,
    approve.isIdle,
    swapAmount.isSuccess,
    swapAmount.data,
    balanceTokenFrom.isSuccess,
  ]);

  const handleRetry = (pay_amount: bigint) => {
    if (approve.isError) approve.reset();
    if (swap.isError) {
      swap.reset();
      handleSwap(pay_amount);
    }
  };

  const onClose = () => {
    dispatchSwapState({
      type: "RESET",
    });
    approve.reset();
    swap.reset();
  };

  return (
    <Dialog open={swapState.is_open_details_dialog} onClose={onClose}>
      <div className="flex items-center justify-end mb-4">
        <Dialog.CloseBtn onClick={onClose} />
      </div>
      {balanceTokenFrom.isSuccess &&
      balanceTokenTo.isSuccess &&
      swapAmount.isSuccess ? (
        <div className="grid grid-cols-1 gap-4 mt-4">
          {(approve.isIdle ||
            swap.isIdle ||
            approve.isPending ||
            swap.isPending) && (
            <div className="flex justify-center items-center px-4 py-8">
              <div className="flex flex-col gap-4 items-center">
                <div>
                  <LoaderSpin size="md" />
                </div>
                {approve.isPending && (
                  <div className="mt-2 text-lg">Approving...</div>
                )}
                {swap.isPending && (
                  <div className="mt-2 text-lg">
                    Buying {swapState.receive_token.token.name}...
                  </div>
                )}
              </div>
            </div>
          )}
          {(approve.isError || swap.isError) && (
            <div className="flex flex-col items-center gap-8">
              <div className="grid grid-cols-1 gap-2 text-center">
                <div className="text-xl text-warning">
                  Buy {swapState.receive_token.token.name} error
                </div>
                <div>Something went wrong, please retry.</div>
              </div>
              <div className="flex justify-center items-center gap-2">
                <BtnPrimary
                  onClick={() => handleRetry(swapAmount.data.pay_amount)}
                  variant="outlined"
                >
                  Retry
                </BtnPrimary>
                <BtnPrimary onClick={onClose}>Close</BtnPrimary>
              </div>
            </div>
          )}
          {approve.isSuccess && swap.isSuccess && (
            <div className="flex flex-col items-center gap-8">
              <div className="flex flex-col items-center gap-2 text-4xl">
                <div>You successfully bought</div>
                <div className="flex items-center gap-2 font-semibold">
                  <Logo
                    name={swapState.receive_token.token.id}
                    className="h-12 w-12"
                  />
                  <E8sToLocaleString
                    value={swapAmount.data.receive_amount}
                    tokenDecimals={balanceTokenTo.data.decimals as number}
                  />
                  <div>{swapState.receive_token.token.name}</div>
                </div>
              </div>

              <div className="bg-surface-secondary border border-border rounded-md p-4 w-full">
                <div className="font-semibold">Transaction details</div>
                <div className="mt-6 grid grid-cols-1 gap-3">
                  <div className="flex justify-between items-start">
                    <div className="text-content/60">
                      Amount deducted from wallet
                    </div>
                    <div className="flex flex-col items-end">
                      <div className="flex items-center gap-1">
                        <Logo
                          name={swapState.send_token.token.id}
                          className="h-4 w-4"
                        />
                        <E8sToLocaleString
                          value={swapAmount.data.pay_amount}
                          tokenDecimals={balanceTokenFrom.data.decimals}
                        />
                        <div>{swapState.send_token.token.name}</div>
                      </div>
                      <div className="text-content/60 text-sm">
                        ≈$
                        <NumberToLocaleString
                          value={
                            (Number(swapAmount.data.receive_amount) /
                              10 ** balanceTokenTo.data.decimals) *
                            balanceTokenTo.data.price_usd
                          }
                        />
                      </div>
                    </div>
                  </div>

                  <div className="flex justify-between items-start">
                    <div className="text-content/60">
                      Amount received on wallet
                    </div>
                    <div className="flex flex-col items-end">
                      <div className="flex items-center gap-1">
                        <Logo
                          name={swapState.receive_token.token.id}
                          className="h-4 w-4"
                        />
                        <E8sToLocaleString
                          value={swapAmount.data.receive_amount}
                          tokenDecimals={balanceTokenTo.data.decimals}
                        />
                        <div>{swapState.receive_token.token.name}</div>
                      </div>
                      <div className="text-content/60 text-sm">
                        ≈$
                        <NumberToLocaleString
                          value={
                            (Number(swapAmount.data.receive_amount) /
                              10 ** balanceTokenTo.data.decimals) *
                            balanceTokenTo.data.price_usd
                          }
                        />
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* <div className="flex flex-col items-center w-full">
                <BtnPrimary onClick={onClose} className="w-full">
                  Go to Wallet
                </BtnPrimary>
                <div className="flex items-center gap-1 px-2 py-1 bg-surface-secondary text-content/60 rounded-md text-sm mt-4">
                  <div>Your balance:</div>
                  {balanceTokenTo.isSuccess ? (
                    <NumberToLocaleString value={balanceTokenTo.data.balance} />
                  ) : (
                    <div>Loading...</div>
                  )}
                  <div>{swapState.receive_token.token.name}</div>
                  <Logo
                    name={swapState.receive_token.token.id}
                    className="h-4 w-4"
                  />
                </div>
              </div> */}
            </div>
          )}
        </div>
      ) : (
        <div className="flex items-center justify-center my-8">
          <LoaderSpin />
        </div>
      )}
    </Dialog>
  );
};

export default DetailsDialog;
