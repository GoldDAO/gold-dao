import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";

import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import { UnlockStateReducerAtom } from "./atoms";
import useDissolveStake from "@services/gldt_stake/hooks/useDissolveStake";

const DetailsDissolve = () => {
  const { authenticatedAgent } = useAuth();
  const queryClient = useQueryClient();
  const [unlockState, dispatch] = useAtom(UnlockStateReducerAtom);
  const dissolve = useDissolveStake(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  const handleDissolve = () => {
    dissolve.mutate(
      {
        id: unlockState.stake_id as bigint,
      },
      {
        onSuccess: (res) => {
          console.log("dissolved");
          console.log(res);
          queryClient.invalidateQueries({
            queryKey: ["USER_STAKE_FETCH_ALL"],
          });
        },
      }
    );
  };

  useEffect(() => {
    if (dissolve.isIdle) handleDissolve();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [dissolve.isIdle]);

  const handleRetry = () => {
    dissolve.reset();
    handleDissolve();
  };

  return (
    <div className="grid grid-cols-1 gap-8 mt-4 lg:mt-6">
      {(dissolve.isIdle || dissolve.isPending) && (
        <div className="flex justify-center items-center px-4 py-8 lg:py-16">
          <div className="flex flex-col gap-4 text-center">
            <div>Loading...</div>
            <div className="mt-2">Dissolving...</div>
          </div>
        </div>
      )}
      {dissolve.isError && (
        <div className="flex flex-col items-center gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-amber-700">Unlock stake error</div>
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
      {dissolve.isSuccess && (
        <div className="grid grid-cols-1 gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-green-700">Unlock stake success</div>
            <div>You successfully unlocked your stake.</div>
          </div>
          <div className="grid grid-cols-1 gap-4 text-center">
            <div className="p-4 border border-border bg-surface-secondary rounded-md text-sm">
              You will receive your GLDT liquid in your wallet in one week.
              <br />
              During this time, you are not receiving any new rewards.
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

export default DetailsDissolve;
