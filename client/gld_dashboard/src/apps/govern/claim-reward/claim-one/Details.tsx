import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAtom } from "jotai";
import { SNS_REWARDS_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import MutationStatusIcon from "@shared/components/MutationStatusIcon";
import { ClaimRewardStateReducerAtom, SelectedRewardsAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useClaimReward from "@services/sns_rewards/hooks/useClaimReward";
import Button from "@shared/ui/button/BtnPrimary";

const Details = () => {
  const navigate = useNavigate();
  const { authenticatedAgent } = useAuth();
  const [, dispatch] = useAtom(ClaimRewardStateReducerAtom);
  const [selectedRewards] = useAtom(SelectedRewardsAtom);

  const claim = useClaimReward(SNS_REWARDS_CANISTER_ID, authenticatedAgent);

  // console.log("claim-one", selectedRewards);

  const handleClaimReward = () => {
    claim.mutate({
      claim_reward_args: selectedRewards.flatMap((reward) =>
        reward.neurons.map((neuron) => ({
          neuron_id: neuron.id,
          token: reward.name,
        }))
      ),
    });
  };

  const onRetry = () => {
    claim.reset();
    handleClaimReward();
  };

  useEffect(() => {
    if (claim.isIdle) handleClaimReward();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [claim.isIdle]);

  const onClose = () => {
    dispatch({ type: "RESET" });
  };

  const onNavigateToWallet = () => {
    onClose();
    navigate("/wallet");
  };

  return (
    <div className="mt-8">
      <div className="flex flex-col items-center justify-center mb-4">
        <MutationStatusIcon size="md" status={claim.status} />
      </div>

      <div className="flex justify-center">
        {(claim.isIdle || claim.isPending) && <div>Claiming Rewards...</div>}
      </div>

      {claim.isError && (
        <>
          <div className="flex justify-center mb-8">Claim rewards error!</div>
          <div className="flex justify-center items-center gap-2">
            <Button variant="outlined" className="w-full" onClick={onRetry}>
              Retry
            </Button>
            <Button className="w-full" onClick={onClose}>
              Close
            </Button>
          </div>
        </>
      )}
      {claim.isSuccess && (
        <>
          <div className="flex justify-center mb-8">Claim rewards success!</div>
          <Button className="w-full" onClick={onNavigateToWallet}>
            View balance
          </Button>
        </>
      )}
    </div>
  );
};

export default Details;
