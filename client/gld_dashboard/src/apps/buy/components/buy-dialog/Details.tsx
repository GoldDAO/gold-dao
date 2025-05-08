import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useAtom } from "jotai";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";
import { useAuth } from "@auth/index";
import { Button, LoaderSpin, Logo } from "@components/index";
import { BuyGLDTStateReducerAtom } from "../../atoms/BuyGLDT";
import useApprove from "@services/ledger/hooks/useApprove";
import useSwap from "@services/kongswap/hooks/useSwap";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";

const Details = () => {
  const { authenticatedAgent, principalId, unauthenticatedAgent, isConnected } =
    useAuth();
  const queryClient = useQueryClient();
  const [buyAtomState, dispatch] = useAtom(BuyGLDTStateReducerAtom);
  const { pay_token, receive_token, max_slippage } = buyAtomState;

  const balance = useFetchUserBalance(
    pay_token.token.canisterId,
    unauthenticatedAgent,
    {
      ledger: pay_token.token.id,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const approve = useApprove(pay_token.token.canisterId, authenticatedAgent);
  const swap = useSwap(KONGSWAP_CANISTER_ID_IC, authenticatedAgent);

  const handleSwap = () => {
    swap.mutate(
      {
        receive_token: "GLDT",
        pay_token: pay_token.token.name,
        pay_amount: pay_token.amount as bigint,
        receive_address: principalId,
        max_slippage: max_slippage as number,
      },
      {
        onSuccess: (res) => {
          console.log("swapped");
          console.log(res);
          queryClient.invalidateQueries({
            queryKey: [`USER_FETCH_LEDGER_BALANCE_${pay_token.token.name}`],
          });
          queryClient.invalidateQueries({
            queryKey: [`USER_FETCH_LEDGER_BALANCE_${receive_token.token.name}`],
          });
        },
      }
    );
  };

  useEffect(() => {
    if (approve.isIdle) {
      approve.mutate(
        {
          amount: (pay_token.amount as bigint) + (pay_token.fee as bigint),
          spender: { owner: KONGSWAP_CANISTER_ID_IC },
        },
        {
          onSuccess: (res) => {
            console.log("approved");
            console.log(res);
            handleSwap();
          },
        }
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [approve.isIdle]);

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
              <div className="mt-2 text-lg">Buying GLDT...</div>
            )}
          </div>
        </div>
      )}
      {(approve.isError || swap.isError) && (
        <div className="flex flex-col items-center gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-amber-600">Buy GLDT error</div>
            <div>Something went wrong, please retry.</div>
          </div>
          <div className="flex justify-center items-center gap-2">
            <Button
              onClick={handleRetry}
              className="px-6 py-2 bg-secondary text-white xl:text-lg font-medium rounded-md"
            >
              Retry
            </Button>
            <Button
              onClick={() => dispatch({ type: "RESET" })}
              className="px-6 py-2 bg-secondary text-white xl:text-lg font-medium rounded-md"
            >
              Close
            </Button>
          </div>
        </div>
      )}
      {approve.isSuccess && swap.isSuccess && (
        <div className="flex flex-col items-center gap-8">
          <div className="flex flex-col items-center gap-2 text-4xl">
            <div>You successfully bought</div>
            <div className="flex items-center gap-2 text-primary font-semibold">
              <Logo name="gldt" className="h-12 w-12" />
              <TokenValueToLocaleString
                value={receive_token.amount}
                decimals={2}
                tokenDecimals={receive_token.decimals as number}
              />
              <div>{receive_token.token.name}</div>
            </div>
          </div>

          <div className="bg-surface-secondary border border-border rounded-md p-4 w-full">
            <div className="text-content/60">Transaction details</div>
            <div className="mt-6 grid grid-cols-1 gap-3">
              <div className="flex justify-between items-start">
                <div className="text-content/60">
                  Amount deducted from wallet
                </div>
                <div className="flex flex-col items-end">
                  <div className="flex items-center gap-1">
                    <Logo name={pay_token.token.id} className="h-4 w-4" />
                    <TokenValueToLocaleString
                      value={pay_token.amount as bigint}
                      decimals={2}
                      tokenDecimals={pay_token.decimals as number}
                    />
                    <div>{pay_token.token.name}</div>
                  </div>
                  <div className="text-content/60 text-sm">
                    ≈${pay_token.amount_usd}
                  </div>
                </div>
              </div>

              <div className="flex justify-between items-start">
                <div className="text-content/60">Amount received on wallet</div>
                <div className="flex flex-col items-end">
                  <div className="flex items-center gap-1">
                    <Logo name={receive_token.token.id} className="h-4 w-4" />
                    <TokenValueToLocaleString
                      value={receive_token.amount as bigint}
                      decimals={2}
                      tokenDecimals={receive_token.decimals as number}
                    />
                    <div>{receive_token.token.name}</div>
                  </div>
                  <div className="text-content/60 text-sm">
                    ≈${receive_token.amount_usd}
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div className="flex flex-col items-center w-full">
            <Button
              onClick={() => dispatch({ type: "RESET" })}
              className="w-full px-6 py-2 bg-secondary text-white xl:text-lg font-medium rounded-md"
            >
              Go to Balance
            </Button>
            <div className="flex items-center gap-1 px-2 py-1 bg-surface-secondary text-content/60 rounded-md text-sm mt-4">
              <div>Your balance:</div>
              {balance.isSuccess ? (
                <TokenValueToLocaleString
                  value={balance.data}
                  decimals={2}
                  tokenDecimals={receive_token.decimals as number}
                />
              ) : (
                <div>Loading...</div>
              )}
              <div>GLDT</div>
              <Logo name="gldt" className="h-4 w-4" />
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Details;
