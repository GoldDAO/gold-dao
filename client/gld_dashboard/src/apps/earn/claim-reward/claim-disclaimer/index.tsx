import clsx from "clsx";
import { useEffect, useState } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { ClaimRewardStateReducerAtom } from "../claim-all/atoms";
import useGetAllPositionsRewards from "../../utils/useGetAllPositionsRewards";
import useRewardsFee from "@utils/useRewardsFee";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";

const ClaimRewardDisclaimer = () => {
  const { authenticatedAgent, principalId, isConnected, unauthenticatedAgent } =
    useAuth();
  const [, dispatchClaimReward] = useAtom(ClaimRewardStateReducerAtom);
  const [enableClaimAll, setEnableClaimAll] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [totalRewards, setTotalRewards] = useState(0);

  const rewards = useGetAllPositionsRewards({
    agent: authenticatedAgent,
    owner: principalId,
    enabled: isConnected && !!authenticatedAgent,
  });

  const rewardsFee = useRewardsFee(unauthenticatedAgent, {
    enabled: isConnected && !!unauthenticatedAgent,
  });

  useEffect(() => {
    if (rewards.isSuccess && rewardsFee.isSuccess) {
      const enabled = rewards.data.some((reward) => {
        const found = rewardsFee.data.find(
          (rewardFee) => rewardFee.name === reward.name
        );
        return found ? reward.amount >= found.fee : false;
      });
      const totalRewards = rewards.data.reduce((acc, reward) => {
        return acc + reward.amount_usd;
      }, 0);
      setTotalRewards(totalRewards);
      setIsSuccess(true);
      setEnableClaimAll(enabled);
    }
  }, [rewards.data, rewards.isSuccess, rewardsFee.data, rewardsFee.isSuccess]);

  return (
    <div className="border border-green-700 bg-surface-primary rounded-xl">
      <div className="rounded-[inherit] p-4 bg-green-700/10">
        <div className="text-green-700 text-center lg:text-left">
          Unclaimed rewards available
        </div>
        <div className="flex flex-col lg:flex-row justify-between items-center mt-2 gap-4">
          <div className="flex flex-col items-center lg:items-start shrink-0">
            <div className="font-semibold text-xl">
              Total of:{" "}
              {isSuccess ? (
                <span>
                  <NumberToLocaleString value={totalRewards} />$
                </span>
              ) : (
                <span>Loading...</span>
              )}
            </div>
            <div className="text-sm text-content/60">
              dispatched in GOLDAO, ICP and OGY
            </div>
          </div>
          <button
            type="button"
            className={clsx(
              "bg-green-700 text-white border border-green-700 rounded-xl",
              "px-4 py-4 text-sm font-semibold shrink-0 cursor-pointer",
              "disabled:cursor-not-allowed disabled:opacity-50"
            )}
            disabled={!enableClaimAll}
            onClick={() => dispatchClaimReward({ type: "OPEN_DIALOG_CONFIRM" })}
          >
            {isSuccess ? "Claim rewards" : "Loading..."}
          </button>
        </div>
      </div>
    </div>
  );
};

export default ClaimRewardDisclaimer;
