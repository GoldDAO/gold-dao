import { useEffect } from "react";
import { useAtom } from "jotai";
import { GLDT_LEDGER_CANISTER_ID, GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import useApprove from "@services/ledger/hooks/useApprove";
import useCreateStake from "@services/gldt_stake/hooks/useCreateStake";
import { StakeStateReducerAtom } from "./atoms";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const Details = () => {
  const { authenticatedAgent } = useAuth();
  const [stakeState, dispatch] = useAtom(StakeStateReducerAtom);
  const amount = (stakeState.amount as bigint) + (stakeState.fee as bigint);

  const approve = useApprove(GLDT_LEDGER_CANISTER_ID, authenticatedAgent);
  const createStakePosition = useCreateStake(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent
  );

  const handleCreateStake = () => {
    createStakePosition.mutate({
      amount,
    });
  };

  useEffect(() => {
    if (createStakePosition.isIdle) {
      approve.mutate(
        {
          amount,
          spender: { owner: GLDT_STAKE_CANISTER_ID },
        },
        {
          onSuccess: (res) => {
            console.log("approved");
            console.log(res);
            handleCreateStake();
          },
        }
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [createStakePosition.isIdle]);

  useEffect(() => {
    return () => {
      approve.reset();
      createStakePosition.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleRetry = () => {
    if (approve.isError) approve.reset();
    if (createStakePosition.isError) {
      createStakePosition.reset();
      handleCreateStake();
    }
  };
  return (
    <div className="grid grid-cols-1 gap-8 mt-4 xl:mt-6">
      {(approve.isIdle ||
        createStakePosition.isIdle ||
        approve.isPending ||
        createStakePosition.isPending) && (
        <div className="flex justify-center items-center px-4 py-8 xl:py-16">
          <div className="flex flex-col gap-4 text-center">
            <div>Loading...</div>
            {approve.isPending && <div className="mt-2">Approving...</div>}
            {createStakePosition.isPending && (
              <div className="mt-2">Creating stake position...</div>
            )}
          </div>
        </div>
      )}
      {(approve.isError || createStakePosition.isError) && (
        <div className="flex flex-col items-center gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-amber-700">Create stake error</div>
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
      {approve.isSuccess && createStakePosition.isSuccess && (
        <div className="flex flex-col items-center gap-8">
          <div className="grid grid-cols-1 gap-2 text-center">
            <div className="text-xl text-green-700">Create stake success</div>
            <div>You successfully created a stake position.</div>
          </div>
          <BtnPrimary onClick={() => dispatch({ type: "RESET" })} size="sm">
            Close
          </BtnPrimary>
        </div>
      )}
    </div>
  );
};

export default Details;
