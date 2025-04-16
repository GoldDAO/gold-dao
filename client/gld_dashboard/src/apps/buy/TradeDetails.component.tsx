import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useAtom } from "jotai";

import { KONGSWAP_CANISTER_ID_IC } from "@constants";

import { useAuth } from "@auth/index";

import { Button } from "@components/index";

import { BuyGLDTStateReducerAtom } from "./atoms";

// import { Logo } from "@components/index";

import useApprove from "@services/ledger/hooks/useApprove";
import useSwap from "@services/kongswap/hooks/useSwap";

const TradeDetails = () => {
  const { authenticatedAgent, principalId } = useAuth();
  const queryClient = useQueryClient();
  const [buyAtomState, dispatch] = useAtom(BuyGLDTStateReducerAtom);
  const { pay_token, receive_token, max_slippage } = buyAtomState;

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
    <div className="grid grid-cols-1 gap-8 mt-4 lg:mt-6">
      {(approve.isIdle ||
        swap.isIdle ||
        approve.isPending ||
        swap.isPending) && (
        <div className="flex justify-center items-center px-4 py-8 lg:py-16">
          <div className="flex flex-col gap-4 text-center">
            <div>Loading...</div>
            {approve.isPending && <div className="mt-2">Approving...</div>}
            {swap.isPending && <div className="mt-2">Buying GLDT...</div>}
          </div>
        </div>
      )}
      {(approve.isError || swap.isError) && (
        <div className="flex flex-col items-center gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-amber-700">Buy GLDT error</div>
            <div>Something went wrong, please retry.</div>
          </div>
          <div className="flex justify-center items-center gap-2">
            <Button
              onClick={handleRetry}
              className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
            >
              Retry
            </Button>
            <Button
              onClick={() => dispatch({ type: "RESET" })}
              className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
            >
              Close
            </Button>
          </div>
        </div>
      )}
      {approve.isSuccess && swap.isSuccess && (
        <div className="flex flex-col items-center gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-green-700">Buy GLDT success</div>
            <div>You successfully bought GLDT.</div>
          </div>

          <Button
            onClick={() => dispatch({ type: "RESET" })}
            className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
          >
            Close
          </Button>
        </div>
      )}
    </div>
  );
};

export default TradeDetails;
