import clsx from "clsx";
import { useEffect, useState } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { ClaimRewardStateReducerAtom } from "../claim-all/atoms";
import useGetAllPositionsRewards from "../../utils/useGetAllPositionsRewards";
import useRewardsFee from "@shared/hooks/useRewardsFee";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { LoaderSpin } from "@components/loaders";

const ClaimRewardDisclaimer = () => {
  const { principalId, isConnected, unauthenticatedAgent } = useAuth();
  const [, dispatchClaimReward] = useAtom(ClaimRewardStateReducerAtom);
  const [enableClaimAll, setEnableClaimAll] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [totalRewards, setTotalRewards] = useState(0);

  const rewards = useGetAllPositionsRewards({
    agent: unauthenticatedAgent,
    owner: principalId,
    enabled: isConnected && !!unauthenticatedAgent,
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

  const renderNoRewardsDisclaimer = () => {
    return (
      <div className="border border-border bg-surface-primary rounded-xl">
        <div className="rounded-[inherit] p-4 bg-surface-secondary/40">
          <div className="text-content/60 text-center xl:text-left">
            No rewards available to claim
          </div>
          <div className="flex flex-col xl:flex-row justify-between items-center mt-2 gap-4">
            <div className="flex flex-col items-center xl:items-start shrink-0">
              <div className="font-semibold text-xl text-content/60">
                Total of:{" "}
                <span>
                  <NumberToLocaleString value={totalRewards} />$
                </span>
              </div>
              <div className="text-sm text-content/60">
                dispatched in GOLDAO, ICP and OGY
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  };

  const renderDisclaimer = () => {
    if (isConnected) {
      if (isSuccess) {
        if (enableClaimAll) {
          return (
            <div className="border border-green-700 bg-surface-primary rounded-xl">
              <div className="rounded-[inherit] p-4 bg-green-700/10">
                <div className="text-green-700 text-center xl:text-left">
                  Unclaimed rewards available
                </div>
                <div className="flex flex-col xl:flex-row justify-between items-center mt-2 gap-4">
                  <div className="flex flex-col items-center xl:items-start shrink-0">
                    <div className="font-semibold text-xl">
                      Total of:{" "}
                      <span>
                        <NumberToLocaleString value={totalRewards} />$
                      </span>
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
                    onClick={() =>
                      dispatchClaimReward({ type: "OPEN_DIALOG_CONFIRM" })
                    }
                  >
                    Claim rewards
                  </button>
                </div>
              </div>
            </div>
          );
        } else {
          return renderNoRewardsDisclaimer();
        }
      } else {
        return (
          <div className="flex items-center justify-center border border-border bg-surface-primary rounded-xl p-4 py-10 gap-4">
            <LoaderSpin size="sm" />
            <div>Fetching your rewards...</div>
          </div>
        );
      }
    } else {
      return renderNoRewardsDisclaimer();
    }
  };

  return <>{renderDisclaimer()}</>;
};

export default ClaimRewardDisclaimer;
