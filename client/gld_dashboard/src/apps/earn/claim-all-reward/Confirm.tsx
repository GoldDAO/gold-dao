import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import { Logo } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { ClaimRewardStateReducerAtom, ConfirmClaimEnableAtom } from "./atoms";
import useGetAllStakePositionRewards from "./utils/useGetAllStakePositions";
import { Reward } from "./utils/index";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";

import useRewardsFee from "@utils/useRewardsFee";

const RewardItem = ({ name }: { name: string }) => {
  const { unauthenticatedAgent, isConnected } = useAuth();
  const [claimRewardState, dispatch] = useAtom(ClaimRewardStateReducerAtom);
  const reward = claimRewardState.rewards.find(
    (r) => r.name === name
  ) as Reward;

  const decimals = useFetchDecimals(reward.canister_id, unauthenticatedAgent, {
    ledger: reward.id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  return (
    <button
      className={clsx(
        "p-4 border border-border rounded-xl",
        `${reward.is_selected ? "bg-green-700/10 border-green-700 hover:bg-green-700/15" : "bg-surface hover:bg-surface-secondary"}`,
        `${reward.is_claimable ? "cursor-pointer " : "cursor-not-allowed"}`
      )}
      disabled={!reward.is_claimable}
      onClick={() =>
        dispatch({ type: "SET_SELECTED_REWARD", value: { name: reward.name } })
      }
    >
      <div className="flex justify-between items-center p-2">
        <div className="font-semibold text-sm flex items-center gap-4">
          <Logo name={reward.id} className="h-10 w-10" />
          <div className="text-left">
            <div>{reward.name}</div>
            <div className="text-content/60">{reward.label}</div>
          </div>
        </div>
        <div className="text-end">
          <div className="font-semibold text-lg">
            {decimals.isSuccess ? (
              <TokenValueToLocaleString
                value={reward.amount as bigint}
                tokenDecimals={decimals.data}
              />
            ) : (
              <div>Loading...</div>
            )}
          </div>
          <div className="text-content/60 text-sm">$todo</div>
        </div>
      </div>
    </button>
  );
};

const Confirm = () => {
  const { authenticatedAgent, principalId, isConnected, unauthenticatedAgent } =
    useAuth();
  const [claimRewardState, dispatch] = useAtom(ClaimRewardStateReducerAtom);
  // const [totalSelectedAmount] = useAtom(TotalSelectedAmountAtom);
  const [confirmClaimEnable] = useAtom(ConfirmClaimEnableAtom);

  const rewards = useGetAllStakePositionRewards({
    agent: authenticatedAgent,
    owner: principalId,
    enabled: isConnected && !!authenticatedAgent,
  });

  const rewardsFee = useRewardsFee(unauthenticatedAgent, {
    enabled: isConnected && !!unauthenticatedAgent,
  });

  useEffect(() => {
    if (rewards.isSuccess && rewardsFee.isSuccess) {
      dispatch({
        type: "SET_REWARDS",
        value: {
          rewards: rewards.data,
          rewards_fee: rewardsFee.data,
        },
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rewards.isSuccess, rewardsFee.isSuccess]);

  useEffect(() => {}, [
    claimRewardState.rewards,
    claimRewardState.is_rewards_initialized,
  ]);

  if (
    !rewards.isSuccess ||
    !rewardsFee.isSuccess ||
    !claimRewardState.is_rewards_initialized
  ) {
    return (
      <div className="flex justify-center items-center px-4 py-16 lg:py-32">
        Loading...
      </div>
    );
  }

  return (
    <>
      <div className="grid grid-cols-1 gap-4 my-8">
        {claimRewardState.rewards.map((reward) => (
          <RewardItem key={reward.name} name={reward.name} />
        ))}
      </div>

      <Button
        className={clsx(
          "px-4 py-3 rounded-md w-full",
          "bg-secondary text-white"
        )}
        onClick={() => dispatch({ type: "CONFIRM" })}
        disabled={!confirmClaimEnable}
      >
        Confirm
      </Button>
    </>
  );
};

export default Confirm;
