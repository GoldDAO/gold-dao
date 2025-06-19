import { useEffect } from "react";
import { useAtom } from "jotai";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
import { UnstakeStateReducerAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useUnstake from "@services/gldt_stake/hooks/useUnstake";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const DetailsUnstake = () => {
  const { authenticatedAgent } = useAuth();

  const [unstakeState, dispatch] = useAtom(UnstakeStateReducerAtom);
  const unstake = useUnstake(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  const handleOnUnstake = () => {
    unstake.mutate({
      id: unstakeState.stake_id as bigint,
    });
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
          <BtnPrimary variant="outlined" onClick={handleOnRetry}>
            Retry
          </BtnPrimary>
          <BtnPrimary onClick={() => dispatch({ type: "RESET" })}>
            Close
          </BtnPrimary>
        </div>
      )}
      {unstake.isSuccess && (
        <BtnPrimary
          className="w-full"
          onClick={() => dispatch({ type: "RESET" })}
        >
          Close
        </BtnPrimary>
      )}
    </>
  );
};

export default DetailsUnstake;
