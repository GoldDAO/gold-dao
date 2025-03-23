import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";

import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { UnstakeStateReducerAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useUnstake from "@services/gldt_stake/hooks/useUnstake";

const DetailsUnstake = () => {
  const { authenticatedAgent } = useAuth();
  const queryClient = useQueryClient();
  const [unstakeState, dispatch] = useAtom(UnstakeStateReducerAtom);
  const unstake = useUnstake(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  const handleOnUnstake = () => {
    unstake.mutate(
      {
        id: unstakeState.stake_id as bigint,
      },
      {
        onSuccess: (res) => {
          console.log("unstaked");
          console.log(res);
          queryClient.invalidateQueries({
            queryKey: ["USER_STAKE_FETCH_ALL"],
          });
          queryClient.invalidateQueries({
            queryKey: ["USER_STAKE_FETCH_TOTAL_STAKED"],
          });
        },
      }
    );
  };

  useEffect(() => {
    if (unstake.isIdle) {
      handleOnUnstake();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [unstake.isIdle]);

  const handleOnRetry = () => {
    unstake.reset();
    handleOnUnstake();
  };

  return (
    <>
      <div className="grid grid-cols-1 gap-4 my-8">
        <div className="p-4 border border-border rounded-md">
          <div className="flex items-center gap-4">
            <MutationStatusIcons status={unstake.status} />
            <div>Unstake stake</div>
          </div>
        </div>
      </div>
      {unstake.isError && (
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
      {unstake.isSuccess && (
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

export default DetailsUnstake;
