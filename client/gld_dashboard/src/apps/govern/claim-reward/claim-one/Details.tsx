import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";

import { SNS_REWARDS_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { ClaimRewardStateReducerAtom, SelectedRewardsAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useClaimReward from "@services/sns_rewards/hooks/useClaimReward";
import { Reward } from "../../utils";

const TokenItem = ({
  reward,
  neuron_id,
}: {
  reward: Reward;
  neuron_id: string;
}) => {
  const { authenticatedAgent } = useAuth();

  const claim = useClaimReward(SNS_REWARDS_CANISTER_ID, authenticatedAgent);

  // const decimals = useFetchDecimals(reward.canisterId, unauthenticatedAgent, {
  //   ledger: reward.id,
  //   enabled: !!unauthenticatedAgent && isConnected,
  // });

  const handleClaimReward = () => {
    claim.mutate(
      {
        neuron_ids: [neuron_id],
        token: reward.name === "GOLDAO" ? "GLDGov" : reward.name, // !TODO fix when sns_rewards canister will be updated
      },
      {
        onSuccess: (res) => {
          console.log("claimed");
          console.log(res);
        },
      }
    );
  };

  useEffect(() => {
    if (claim.isIdle) handleClaimReward();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [claim.isIdle]);

  // <div className="flex justify-center items-center">Loading...</div>

  const handleOnRetry = () => {
    claim.reset();
    handleClaimReward();
  };

  return (
    <div className="p-4 border border-border rounded-md">
      <div className="flex justify-between items-center">
        <div className="flex items-center gap-4">
          <MutationStatusIcons status={claim.status} />
          <div>Claiming {reward.name} reward</div>
        </div>
        {claim.isError && (
          <div>
            <Button
              className={clsx(
                "px-2 py-1 rounded-md",
                "bg-secondary text-white text-sm"
              )}
              onClick={handleOnRetry}
            >
              Retry
            </Button>
          </div>
        )}
      </div>
    </div>
  );
};

const Details = () => {
  const [claimRewardState, dispatch] = useAtom(ClaimRewardStateReducerAtom);
  const [selectedRewards] = useAtom(SelectedRewardsAtom);

  return (
    <>
      <div className="grid grid-cols-1 gap-4 my-8">
        {selectedRewards.map((reward) => (
          <TokenItem
            key={reward.id}
            reward={reward}
            neuron_id={claimRewardState.neuron_id as string}
          />
        ))}
      </div>
      <Button
        className={clsx(
          "px-4 py-3 rounded-md w-full",
          "bg-secondary text-white"
        )}
        onClick={() => dispatch({ type: "RESET" })}
      >
        Go to govern view
      </Button>
    </>
  );
};

export default Details;
