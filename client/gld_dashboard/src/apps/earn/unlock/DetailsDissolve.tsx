import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";

import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { UnlockStateReducerAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useDissolveStake from "@services/gldt_stake/hooks/useDissolveStake";

const DetailsDissolve = () => {
  const { authenticatedAgent } = useAuth();
  const queryClient = useQueryClient();
  const [unlockState, dispatch] = useAtom(UnlockStateReducerAtom);
  const dissolve = useDissolveStake(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  const handleOnDissolve = () => {
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
    if (dissolve.isIdle) {
      handleOnDissolve();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [dissolve.isIdle]);

  const handleOnRetry = () => {
    dissolve.reset();
    handleOnDissolve();
  };

  return (
    <>
      <div className="grid grid-cols-1 gap-4 my-8">
        <div className="p-4 border border-border rounded-md">
          <div className="flex items-center gap-4">
            <MutationStatusIcons status={dissolve.status} />
            <div>Dissolve stake</div>
          </div>
        </div>
      </div>
      {dissolve.isError && (
        <div className="flex justify-center items-center gap-4">
          <Button
            className={clsx("px-4 py-3 rounded-md", "bg-secondary text-white")}
            onClick={handleOnRetry}
          >
            Retry
          </Button>
          <Button
            className={clsx("px-4 py-3 rounded-md", "bg-secondary text-white")}
            onClick={() => dispatch({ type: "RESET" })}
          >
            Close
          </Button>
        </div>
      )}
      {dissolve.isSuccess && (
        <Button
          className={clsx(
            "px-4 py-3 rounded-md",
            "bg-secondary text-white w-full"
          )}
          onClick={() => dispatch({ type: "RESET" })}
        >
          Close
        </Button>
      )}
    </>
  );
};

export default DetailsDissolve;
