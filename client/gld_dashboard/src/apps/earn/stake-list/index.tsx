import clsx from "clsx";
import { useAtom } from "jotai";
import { Clock } from "iconsax-react";
import { GLDT_LEDGER_CANISTER_ID, GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button, Logo } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import useFetchUserPositions from "@services/gldt_stake/hooks/useFetchUserPositions";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import { ClaimRewardStateReducerAtom } from "../claim-reward/claim-one/atoms";
import { UnlockStateReducerAtom } from "../unlock/atoms";
import { UnstakeStateReducerAtom } from "../unstake/atoms";
import ClaimRewardsConfirm from "../claim-reward/claim-one/Confirm";
import ClaimRewardsDetails from "../claim-reward/claim-one/Details";
import ConfirmUnlock from "../unlock/Confirm";
import DetailsDissolve from "../unlock/DetailsDissolve";
import DetailsUnstakeEarly from "../unlock/DetailsUnstakeEarly";
import ConfirmUnstake from "../unstake/Confirm";
import DetailsUnstake from "../unstake/Details";
import useRewardsFee, { RewardFeeData } from "@shared/hooks/useRewardsFee";
import { Reward } from "@services/gldt_stake/utils/interfaces";

const StakeList = () => {
  const { authenticatedAgent, unauthenticatedAgent, isConnected } = useAuth();
  const [claimRewardState, dispatchClaimReward] = useAtom(
    ClaimRewardStateReducerAtom
  );
  const [unlockState, dispatchUnlock] = useAtom(UnlockStateReducerAtom);
  const [unstakeState, dispatchUnstake] = useAtom(UnstakeStateReducerAtom);

  const stakeRewardsFee = useRewardsFee(unauthenticatedAgent, {
    enabled: isConnected && !!unauthenticatedAgent,
  });

  const decimals = useFetchDecimals(
    GLDT_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "gldt",
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const fetchUserPositions = useFetchUserPositions(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    {
      enabled: isConnected && !!authenticatedAgent,
    }
  );

  const renderDisconnectedPlaceholder = () => {
    return (
      <div className="flex flex-col gap-4 relative">
        {[...Array(2)].map((_, index) => (
          <div key={index}>
            <div
              className={clsx(
                "@container",
                "shrink-0",
                "rounded-md xl:rounded-xl border border-surface-secondary p-4 cursor-pointer"
              )}
            >
              <div className="flex justify-between items-center p-2">
                <div className="flex items-center gap-2">
                  <div className="h-5 w-5 bg-surface-secondary rounded-full" />
                  <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
                </div>
                <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
              </div>
            </div>
          </div>
        ))}
        <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-background to-transparent" />
      </div>
    );
  };

  const renderEmptyStakeList = () => {
    return (
      <div className="p-4 xl:p-8 border border-border rounded-md xl:rounded-xl text-center bg-surface-primary">
        <div className="font-semibold">You currently don’t have any stakes</div>
        <div className="text-content/60">
          Start staking to earn rewards in GOLDAO, OGY & ICP.
        </div>
      </div>
    );
  };

  if (!isConnected) {
    return renderDisconnectedPlaceholder();
  }

  if (!fetchUserPositions.isSuccess || !decimals.isSuccess) {
    return <div className="flex justify-center p-4">Loading...</div>;
  }

  if (
    !!isConnected &&
    fetchUserPositions.isSuccess &&
    !fetchUserPositions.data.length
  ) {
    return renderEmptyStakeList();
  }

  const enableClaimRewards = (
    rewards: Reward[],
    rewardsFee: RewardFeeData[]
  ): boolean => {
    return rewards.some((reward) => {
      const found = rewardsFee.find(
        (rewardFee) => rewardFee.name === reward.name
      );
      return found ? reward.amount >= found.fee : false;
    });
  };

  const renderClaimRewardsButton = (
    stake_id: bigint,
    rewards: Reward[],
    rewards_fee: RewardFeeData[] | undefined
  ) => {
    const enabled = !!rewards_fee && enableClaimRewards(rewards, rewards_fee);
    return (
      <Button
        className={clsx(
          "px-2 py-1 rounded-md shrink-0",
          "bg-surface-secondary dark:bg-transparent border border-border text-black dark:text-white text-sm"
        )}
        disabled={!enabled}
        onClick={() =>
          dispatchClaimReward({
            type: "OPEN_DIALOG_CONFIRM",
            value: { stake_id },
          })
        }
      >
        {rewards_fee !== undefined ? (
          <div className="flex items-center gap-2">
            <div className="flex items-center gap-1">
              <Logo name="goldao" className="h-4 w-4" />
              <Logo name="icp" className="h-4 w-4" />
              <Logo name="ogy" className="h-4 w-4" />
            </div>
            Claim rewards
          </div>
        ) : (
          "Loading..."
        )}
      </Button>
    );
  };

  return (
    <div className="flex flex-col pb-4 xl:overflow-y-auto xl:pr-4">
      {fetchUserPositions.data.map((stake, index) => (
        <div
          key={index}
          className="@container flex justify-between items-center p-3 border-b border-border/60 last:border-0 odd:bg-gold/5"
        >
          <div className="flex flex-col @sm:flex-row @sm:items-center @sm:justify-between w-full">
            <div className="flex items-center gap-8">
              <div className="flex items-center gap-2">
                <Logo name="gldt" className="h-5 w-5" />
                <E8sToLocaleString
                  value={stake.amount}
                  tokenDecimals={decimals.data}
                />{" "}
                GLDT
              </div>
              <div className="flex gap-1 items-center text-content/60">
                <Clock size={16} />
                Age Bonus{" "}
                <NumberToLocaleString value={stake.age_bonus_multiplier} />
              </div>
            </div>
            <div className="flex flex-row gap-2 mt-4 @xl:mt-0">
              {stake.is_unlockable && (
                <Button
                  className="shrink-0 px-2 py-1 bg-surface-secondary dark:bg-transparent border border-border text-black dark:text-white text-sm rounded-md"
                  onClick={() =>
                    dispatchUnlock({
                      type: "OPEN_DIALOG_CONFIRM",
                      value: { stake_id: stake.id },
                    })
                  }
                >
                  Unlock
                </Button>
              )}
              {stake.is_unstakable && (
                <Button
                  className="shrink-0 px-2 py-1 bg-surface-secondary dark:bg-transparent border border-border text-black dark:text-white text-sm rounded-md"
                  onClick={() =>
                    dispatchUnstake({
                      type: "OPEN_DIALOG_CONFIRM",
                      value: { stake_id: stake.id },
                    })
                  }
                >
                  Unstake
                </Button>
              )}
              {renderClaimRewardsButton(
                stake.id,
                stake.rewards,
                stakeRewardsFee.data
              )}
            </div>
          </div>
        </div>
      ))}
      {/* CLAIM REWARDS DIALOGS */}
      <Dialog
        open={claimRewardState.is_open_claim_dialog_confirm}
        handleOnClose={() => dispatchClaimReward({ type: "CANCEL" })}
        title="Claim rewards"
      >
        <ClaimRewardsConfirm />
      </Dialog>

      <Dialog
        open={claimRewardState.is_open_claim_dialog_details}
        handleOnClose={() => dispatchClaimReward({ type: "RESET" })}
        title="Claim details"
      >
        <ClaimRewardsDetails />
      </Dialog>

      {/* UNSTAKE DIALOGS */}
      <Dialog
        open={unstakeState.is_open_unstake_dialog_confirm}
        handleOnClose={() => dispatchUnstake({ type: "CANCEL" })}
        title="Confirm unstake"
      >
        <ConfirmUnstake />
      </Dialog>

      <Dialog
        open={unstakeState.is_open_unstake_dialog_details}
        handleOnClose={() => dispatchUnstake({ type: "RESET" })}
        title="Unstake details"
      >
        <DetailsUnstake />
      </Dialog>

      {/* UNLOCK DIALOGS */}
      <Dialog
        open={unlockState.is_open_unlock_dialog_confirm}
        handleOnClose={() => dispatchUnlock({ type: "CANCEL" })}
        title="Confirm unlock"
      >
        <ConfirmUnlock />
      </Dialog>

      <Dialog
        open={unlockState.is_open_dissolve_dialog_details}
        handleOnClose={() => dispatchUnlock({ type: "RESET" })}
        title="Dissolve details"
      >
        <DetailsDissolve />
      </Dialog>

      <Dialog
        open={unlockState.is_open_unstake_early_dialog_details}
        handleOnClose={() => dispatchUnlock({ type: "RESET" })}
        title="Unstake early stake"
      >
        <DetailsUnstakeEarly />
      </Dialog>
    </div>
  );
};

export default StakeList;
