import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useAtom } from "jotai";

import { KONGSWAP_CANISTER_ID_IC } from "@constants";

import { useAuth } from "@auth/index";

import { Button } from "@components/index";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";

import BuyGLDTStateAtom from "./atoms";

// import { Logo } from "@components/index";
// import E8sToLocaleString from "@components/numbers/E8sToLocaleString";

import useApprove from "@services/ledger/hooks/useApprove";
import useFetchTransferFee from "@services/ledger/hooks/useFetchTransferFee";
import useSwap from "@services/kongswap/hooks/useSwap";
import { RESET } from "jotai/utils";

const TradeDetails = ({ className }: { className?: string }) => {
  const { authenticatedAgent, principalId, isConnected } = useAuth();
  const queryClient = useQueryClient();
  const [buyAtomState, setBuyAtomstate] = useAtom(BuyGLDTStateAtom);
  const { pay_token, pay_amount, receive_token, is_open_details_dialog } =
    buyAtomState;

  const transferFee = useFetchTransferFee(
    pay_token.canisterId,
    authenticatedAgent,
    {
      ledger: pay_token.id,
      enabled: !!authenticatedAgent && !!isConnected,
    }
  );

  const approve = useApprove(pay_token.canisterId, authenticatedAgent);
  const swap = useSwap(KONGSWAP_CANISTER_ID_IC, authenticatedAgent);

  const handleRetry = () => {
    if (approve.isError) approve.reset();
    if (swap.isError && pay_amount !== null) {
      swap.reset();
      swap.mutate(
        {
          receive_token: receive_token.name,
          pay_token: pay_token.name,
          pay_amount,
          receive_address: principalId,
        },
        {
          onSuccess: (res) => {
            console.log("swapped");
            console.log(res);
          },
        }
      );
    }
  };

  const handleClose = () => {
    setBuyAtomstate(RESET);
    approve.reset();
    swap.reset();
  };

  useEffect(() => {
    if (
      approve.isIdle &&
      is_open_details_dialog &&
      transferFee.isSuccess &&
      pay_amount
    ) {
      approve.mutate(
        {
          amount: pay_amount + transferFee.data,
          spender: { owner: KONGSWAP_CANISTER_ID_IC },
        },
        {
          onSuccess: (res) => {
            console.log("approved");
            console.log(res);
            swap.mutate(
              {
                receive_token: receive_token.name,
                pay_token: pay_token.name,
                pay_amount,
                receive_address: principalId,
              },
              {
                onSuccess: (res) => {
                  console.log("swapped");
                  console.log(res);
                  queryClient.invalidateQueries({
                    queryKey: [`USER_FETCH_LEDGER_BALANCE_${pay_token.name}`],
                  });
                },
              }
            );
          },
        }
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    is_open_details_dialog,
    transferFee.isSuccess,
    pay_amount,
    transferFee.data,
    principalId,
    pay_token.name,
    approve.isIdle,
  ]);

  return (
    <div className={className}>
      <div className="flex flex-col gap-4 mt-4 lg:mt-8">
        <div className="flex items-center gap-4">
          <MutationStatusIcons status={approve.status} />
          <div>1. Approve</div>
        </div>
        <div className="flex items-center gap-4">
          <MutationStatusIcons status={swap.status} />
          <div>2. Swap</div>
        </div>
      </div>

      <div className="flex justify-center items-center gap-2 mt-4 lg:mt-8">
        {(approve.isError || swap.isError) && (
          <Button
            onClick={handleRetry}
            className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
          >
            Retry
          </Button>
        )}
        {!(approve.isPending || swap.isPending) && (
          <Button
            onClick={handleClose}
            className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
          >
            Close
          </Button>
        )}
      </div>
    </div>
  );
};

export default TradeDetails;
