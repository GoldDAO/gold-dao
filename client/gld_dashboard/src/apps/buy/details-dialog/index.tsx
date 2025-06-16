import { useEffect } from "react";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";
import { useAuth } from "@auth/index";
import { LoaderSpin, Logo } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import useApprove from "@services/ledger/hooks/useApprove";
import useSwap from "@services/kongswap/hooks/useSwap";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import { PayToken, ReceiveToken } from "@buy/shared/utils";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const DetailsDialog = ({
  open,
  handleClose,
  payToken,
  receiveToken,
  maxSlippage,
}: {
  open: boolean;
  handleClose: () => void;
  payToken: PayToken;
  receiveToken: ReceiveToken;
  maxSlippage: number;
}) => {
  const { authenticatedAgent, principalId, unauthenticatedAgent, isConnected } =
    useAuth();

  const balance = useFetchUserBalance(
    receiveToken.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: receiveToken.token.id,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const approve = useApprove(payToken.token.canisterId, authenticatedAgent);
  const swap = useSwap(KONGSWAP_CANISTER_ID_IC, authenticatedAgent, {
    pay_token: payToken.token.id,
    receive_token: receiveToken.token.id,
  });

  const handleSwap = () => {
    swap.mutate({
      pay_amount: payToken.amount as bigint,
      receive_address: principalId,
      max_slippage: maxSlippage,
    });
  };

  useEffect(() => {
    if (open && approve.isIdle) {
      approve.mutate(
        {
          amount: (payToken.amount as bigint) + (payToken.fee as bigint),
          spender: { owner: KONGSWAP_CANISTER_ID_IC },
        },
        {
          onSuccess: () => handleSwap(),
        }
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open, approve.isIdle]);

  useEffect(() => {
    return () => {
      approve.reset();
      swap.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleRetry = () => {
    if (approve.isError) approve.reset();
    if (swap.isError) {
      swap.reset();
      handleSwap();
    }
  };

  return (
    <Dialog open={open} handleOnClose={handleClose}>
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
                  Buying {receiveToken.token.name}...
                </div>
              )}
            </div>
          </div>
        )}
        {(approve.isError || swap.isError) && (
          <div className="flex flex-col items-center gap-8">
            <div className="grid grid-cols-1 gap-2 text-center">
              <div className="text-xl text-amber-600">
                Buy {receiveToken.token.name} error
              </div>
              <div>Something went wrong, please retry.</div>
            </div>
            <div className="flex justify-center items-center gap-2">
              <BtnPrimary onClick={handleRetry} variant="outlined">
                Retry
              </BtnPrimary>
              <BtnPrimary onClick={handleClose}>Close</BtnPrimary>
            </div>
          </div>
        )}
        {approve.isSuccess && swap.isSuccess && (
          <div className="flex flex-col items-center gap-8">
            <div className="flex flex-col items-center gap-2 text-4xl">
              <div>You successfully bought</div>
              <div className="flex items-center gap-2 text-gold font-semibold">
                <Logo name={receiveToken.token.id} className="h-12 w-12" />
                <E8sToLocaleString
                  value={receiveToken.amount}
                  tokenDecimals={receiveToken.decimals as number}
                />
                <div>{receiveToken.token.name}</div>
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
                      <Logo name={payToken.token.id} className="h-4 w-4" />
                      <E8sToLocaleString
                        value={payToken.amount as bigint}
                        tokenDecimals={payToken.decimals as number}
                      />
                      <div>{payToken.token.name}</div>
                    </div>
                    <div className="text-content/60 text-sm">
                      ≈$
                      <NumberToLocaleString
                        value={receiveToken.amount_usd as number}
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
                      <Logo name={receiveToken.token.id} className="h-4 w-4" />
                      <E8sToLocaleString
                        value={receiveToken.amount as bigint}
                        tokenDecimals={receiveToken.decimals as number}
                      />
                      <div>{receiveToken.token.name}</div>
                    </div>
                    <div className="text-content/60 text-sm">
                      ≈$
                      <NumberToLocaleString
                        value={receiveToken.amount_usd as number}
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div className="flex flex-col items-center w-full">
              <BtnPrimary onClick={handleClose} className="w-full">
                Go to Balance
              </BtnPrimary>
              <div className="flex items-center gap-1 px-2 py-1 bg-surface-secondary text-content/60 rounded-md text-sm mt-4">
                <div>Your balance:</div>
                {balance.isSuccess ? (
                  <E8sToLocaleString
                    value={balance.data}
                    tokenDecimals={receiveToken.decimals as number}
                  />
                ) : (
                  <div>Loading...</div>
                )}
                <div>{receiveToken.token.name}</div>
                <Logo name={receiveToken.token.id} className="h-4 w-4" />
              </div>
            </div>
          </div>
        )}
      </div>
    </Dialog>
  );
};

export default DetailsDialog;
