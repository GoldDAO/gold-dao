import { useEffect } from "react";
import { useAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";

import { GLDT_LEDGER_CANISTER_ID, GLDT_STAKE_CANISTER_ID } from "@constants";

import { useAuth } from "@auth/index";

// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import Dialog from "@components/dialogs/Dialog";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
import { Button } from "@components/index";

import useApprove from "@services/ledger/hooks/useApprove";
import useCreateStake from "@services/gldt_stake/hooks/useCreateStake";

import { StakeStateReducerAtom } from "./atoms";

const DetailsDialog = () => {
  const queryClient = useQueryClient();
  const { authenticatedAgent } = useAuth();
  const [stakeState, dispatch] = useAtom(StakeStateReducerAtom);

  const { is_open_stake_dialog_details, amount, fee } = stakeState;

  const approve = useApprove(GLDT_LEDGER_CANISTER_ID, authenticatedAgent);
  const createStakePosition = useCreateStake(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent
  );

  useEffect(() => {
    if (
      createStakePosition.isIdle &&
      is_open_stake_dialog_details &&
      amount &&
      fee
    ) {
      approve.mutate(
        {
          amount: amount + fee,
          spender: { owner: GLDT_STAKE_CANISTER_ID },
        },
        {
          onSuccess: (res) => {
            console.log("approved");
            console.log(res);
            createStakePosition.mutate(
              {
                amount: amount + fee,
              },
              {
                onSuccess: (res) => {
                  console.log("stake position opened");
                  console.log(res);
                  queryClient.invalidateQueries({
                    queryKey: [`USER_FETCH_LEDGER_BALANCE_GLDT`],
                  });
                  queryClient.invalidateQueries({
                    queryKey: ["USER_STAKE_FETCH_ALL"],
                  });
                  queryClient.invalidateQueries({
                    queryKey: ["USER_STAKE_FETCH_TOTAL_STAKED"],
                  });
                },
              }
            );
          },
        }
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [createStakePosition.isIdle, is_open_stake_dialog_details, amount, fee]);

  const handleRetry = () => {
    if (approve.isError) approve.reset();
    if (createStakePosition.isError && amount && fee) {
      createStakePosition.reset();
      createStakePosition.mutate(
        {
          amount: amount + fee,
        },
        {
          onSuccess: (res) => {
            console.log("stake position opened");
            console.log(res);
          },
        }
      );
    }
  };

  const handleCloseStakeDetailsDialog = () => {
    dispatch({ type: "RESET" });
    approve.reset();
    createStakePosition.reset();
  };

  return (
    <>
      {is_open_stake_dialog_details && (
        <Dialog
          open={is_open_stake_dialog_details}
          handleOnClose={handleCloseStakeDetailsDialog}
          title="Stake details"
        >
          <div className="flex flex-col gap-4 mt-4 lg:mt-8">
            <div className="flex items-center gap-4">
              <MutationStatusIcons status={approve.status} />
              <div>1. Approve</div>
            </div>
            <div className="flex items-center gap-4">
              <MutationStatusIcons status={createStakePosition.status} />
              <div>2. Create stake position</div>
            </div>
          </div>

          <div className="flex justify-center items-center gap-2 mt-4 lg:mt-8">
            {(approve.isError || createStakePosition.isError) && (
              <Button
                onClick={handleRetry}
                className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
              >
                Retry
              </Button>
            )}
            {!(approve.isPending || createStakePosition.isPending) && (
              <Button
                onClick={handleCloseStakeDetailsDialog}
                className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
              >
                Close
              </Button>
            )}
          </div>
        </Dialog>
      )}
    </>
  );
};

export default DetailsDialog;
