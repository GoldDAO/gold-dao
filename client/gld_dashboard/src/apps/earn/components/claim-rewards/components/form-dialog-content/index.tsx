import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import {
  ClaimRewardsStateReducerAtom,
  IsDisabledClaimingRewardsAtom,
} from "../../atoms";
import useFetchUserPosition from "@earn/hooks/useFetchUserPosition";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import { Reward } from "@earn/interfaces";
import Icon from "@shared/ui/icons";

const RewardItem = ({
  reward,
  handleOnClick,
}: {
  reward: Reward;
  handleOnClick: (reward: Reward) => void;
}) => {
  return (
    <button
      className={clsx(
        "p-4 border border-border rounded-xl",
        `${
          reward.is_selected
            ? "bg-success/10 border-success hover:bg-success/15"
            : "bg-surface hover:bg-surface-secondary"
        }`,
        `${reward.is_claimable ? "cursor-pointer" : "cursor-not-allowed"}`
      )}
      disabled={!reward.is_claimable}
      onClick={() => handleOnClick(reward)}
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
          <div className="flex items-center gap-2">
            {reward.is_amount_below_fee && (
              <Icon.Warning
                width={16}
                className="text-yellow-500"
                data-tooltip-id="tooltip"
                data-tooltip-html={
                  "Reward amount is below the transaction fee."
                }
                data-tooltip-place="left"
              />
            )}
            <div className="font-semibold text-lg">
              <NumberToLocaleString value={reward.amount} decimals={5} />
            </div>
          </div>

          <div className="text-content/60 text-sm">
            $<NumberToLocaleString value={reward.amount_usd} decimals={5} />
          </div>
        </div>
      </div>
    </button>
  );
};

const Confirm = () => {
  const { isConnected, unauthenticatedAgent, principalId } = useAuth();
  const [claimRewardsState, dispatch] = useAtom(ClaimRewardsStateReducerAtom);
  const [isDisabledClaimingRewards] = useAtom(IsDisabledClaimingRewardsAtom);

  const position = useFetchUserPosition(GLDT_STAKE_CANISTER_ID, {
    enabled: isConnected && !!unauthenticatedAgent && !!principalId,
    agent: unauthenticatedAgent,
    owner: principalId,
  });

  useEffect(() => {
    if (position.isSuccess && position.data) {
      dispatch({
        type: "SET_REWARDS",
        value: {
          rewards: position.data.rewards,
        },
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [position.isSuccess]);

  const handleOnClickReward = (reward: Reward) => {
    dispatch({
      type: "SET_SELECTED_REWARD",
      value: { name: reward.name },
    });
  };

  if (position.isSuccess && position.data) {
    return (
      <>
        <div className="grid grid-cols-1 gap-4 my-8">
          {claimRewardsState.rewards.map((reward) => (
            <RewardItem
              key={reward.name}
              reward={reward}
              handleOnClick={handleOnClickReward}
            />
          ))}
        </div>

        <BtnPrimary
          onClick={() => dispatch({ type: "SET_IS_STEP_DETAILS" })}
          disabled={isDisabledClaimingRewards}
          className="w-full"
        >
          Confirm
        </BtnPrimary>
      </>
    );
  }

  return (
    <div className="flex justify-center items-center px-4 py-16 xl:py-32">
      Loading...
    </div>
  );
};

export default Confirm;
