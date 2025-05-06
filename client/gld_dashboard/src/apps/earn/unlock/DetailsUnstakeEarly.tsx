import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";

import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { UnlockStateReducerAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useUnstakeEarly from "@services/gldt_stake/hooks/useUnstakeEarly";

const DetailsUnstakeEarly = () => {
  const { authenticatedAgent } = useAuth();
  const queryClient = useQueryClient();
  const [unlockState, dispatch] = useAtom(UnlockStateReducerAtom);
  const unstakeEarly = useUnstakeEarly(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent
  );

  const handleUnstake = () => {
    unstakeEarly.mutate(
      {
        id: unlockState.stake_id as bigint,
      },
      {
        onSuccess: (res) => {
          console.log("unstaked early");
          console.log(res);
          queryClient.invalidateQueries({
            queryKey: ["USER_STAKE_FETCH_ALL"],
          });
          queryClient.invalidateQueries({
            queryKey: ["USER_STAKE_FETCH_TOTAL_STAKED"],
          });
          queryClient.invalidateQueries({
            queryKey: ["USER_FETCH_LEDGER_BALANCE_GLDT"],
          });
        },
      }
    );
  };

  useEffect(() => {
    if (unstakeEarly.isIdle) handleUnstake();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [unstakeEarly.isIdle]);

  const handleRetry = () => {
    unstakeEarly.reset();
    handleUnstake();
  };

  return (
    <div className="grid grid-cols-1 gap-8 mt-4 xl:mt-6">
      {(unstakeEarly.isIdle || unstakeEarly.isPending) && (
        <div className="flex justify-center items-center px-4 py-8 xl:py-16">
          <div className="flex flex-col gap-4 text-center">
            <div>Loading...</div>
            <div className="mt-2">Unstaking...</div>
          </div>
        </div>
      )}
      {unstakeEarly.isError && (
        <div className="flex flex-col items-center gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-amber-700">Unstake error</div>
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
      {unstakeEarly.isSuccess && (
        <div className="grid grid-cols-1 gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-green-700">Unstake success</div>
            <div>You successfully unstaked your stake.</div>
          </div>
          <div className="grid grid-cols-1 gap-4 text-center">
            <div className="p-4 border border-border bg-surface-secondary rounded-md text-sm">
              You will be charged of (todo) GLDT.
            </div>
          </div>
          <Button
            className={clsx(
              "px-4 py-3 rounded-md",
              "bg-secondary text-white w-full"
            )}
            onClick={() => dispatch({ type: "RESET" })}
          >
            Close
          </Button>
        </div>
      )}
    </div>
  );
};

export default DetailsUnstakeEarly;
