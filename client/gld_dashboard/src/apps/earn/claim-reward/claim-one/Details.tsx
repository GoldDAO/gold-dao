import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAtom } from "jotai";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
// import E8sToLocaleString from "@components/numbers/E8sToLocaleString";
import { ClaimRewardStateReducerAtom, SelectedRewardsAtom } from "./atoms";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useClaimReward from "@services/gldt_stake/hooks/useClaimReward";
import { Reward } from "../../utils";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const TokenItem = ({
  reward,
  stake_id,
}: {
  reward: Reward;
  stake_id: bigint;
}) => {
  const { authenticatedAgent } = useAuth();

  const claim = useClaimReward(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  // const decimals = useFetchDecimals(reward.canisterId, unauthenticatedAgent, {
  //   ledger: reward.id,
  //   enabled: !!unauthenticatedAgent && isConnected,
  // });

  const handleClaimReward = () => {
    claim.mutate({
      position_ids: [stake_id],
      token: reward.name,
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

  const handleNavigateToWallet = () => {
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
            stake_id={claimRewardState.stake_id as bigint}
          />
        ))}
      </div>
      <BtnPrimary className="w-full" onClick={handleNavigateToWallet}>
        Go to wallet view
      </BtnPrimary>
    </>
  );
};

export default Details;
