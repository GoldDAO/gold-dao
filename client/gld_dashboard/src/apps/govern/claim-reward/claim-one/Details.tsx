import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAtom } from "jotai";
import { SNS_REWARDS_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
import { ClaimRewardStateReducerAtom, SelectedRewardsAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useClaimReward from "@services/sns_rewards/hooks/useClaimReward";
import { Reward } from "../../utils";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

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
    claim.mutate({
      neuron_ids: [neuron_id],
      token: reward.name === "GOLDAO" ? "GLDGov" : reward.name, // !TODO fix when sns_rewards canister will be updated
    });
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
            <BtnPrimary size="sm" onClick={handleOnRetry}>
              Retry
            </BtnPrimary>
          </div>
        )}
      </div>
    </div>
  );
};

const Details = () => {
  const [claimRewardState, dispatch] = useAtom(ClaimRewardStateReducerAtom);
  const [selectedRewards] = useAtom(SelectedRewardsAtom);

  const navigate = useNavigate();

  const handleOnClickViewBalance = () => {
    dispatch({ type: "RESET" });
    navigate("/wallet");
  };

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
      <BtnPrimary className="w-full" onClick={handleOnClickViewBalance}>
        View balance
      </BtnPrimary>
    </>
  );
};

export default Details;
