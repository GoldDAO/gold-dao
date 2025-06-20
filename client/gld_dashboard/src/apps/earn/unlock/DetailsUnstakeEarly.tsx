import { useEffect } from "react";
import { useAtom } from "jotai";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { UnlockStateReducerAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useUnstakeEarly from "@services/gldt_stake/hooks/useUnstakeEarly";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const DetailsUnstakeEarly = () => {
  const { authenticatedAgent } = useAuth();
  const [unlockState, dispatch] = useAtom(UnlockStateReducerAtom);
  const unstakeEarly = useUnstakeEarly(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent
  );

  const handleUnstake = () => {
    unstakeEarly.mutate({
      id: unlockState.stake_id as bigint,
    });
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
            <div className="text-xl text-warning">Unstake error</div>
            <div>Something went wrong, please retry.</div>
          </div>
          <div className="flex justify-center items-center gap-2">
            <BtnPrimary onClick={handleRetry} variant="outlined">
              Retry
            </BtnPrimary>
            <BtnPrimary onClick={() => dispatch({ type: "RESET" })}>
              Close
            </BtnPrimary>
          </div>
        </div>
      )}
      {unstakeEarly.isSuccess && (
        <div className="grid grid-cols-1 gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-success">Unstake success</div>
            <div>You successfully unstaked your stake.</div>
          </div>
          <div className="grid grid-cols-1 gap-4 text-center">
            <div className="p-4 border border-border bg-surface-secondary rounded-md text-sm">
              You will be charged of (todo) GLDT.
            </div>
          </div>
          <BtnPrimary
            className="w-full"
            onClick={() => dispatch({ type: "RESET" })}
          >
            Close
          </BtnPrimary>
        </div>
      )}
    </div>
  );
};

export default DetailsUnstakeEarly;
