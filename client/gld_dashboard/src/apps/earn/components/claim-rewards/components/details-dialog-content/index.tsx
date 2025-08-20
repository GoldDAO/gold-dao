import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import {
  ClaimRewardsStateReducerAtom,
  TotalSelectedAmountUSDAtom,
} from "../../atoms";
import Button from "@shared/ui/button/HorizontalButton";
import useClaimRewards from "./hooks/useClaimRewards";
import { LoaderSpin } from "@components/loaders";
import { Reward } from "@earn/interfaces";
import { Logo } from "@components/index";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const RewardItem = ({ reward }: { reward: Reward }) => {
  return (
    <button className={clsx("p-4 border border-border rounded-xl bg-surface")}>
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
            <NumberToLocaleString value={reward.amount} decimals={5} />
          </div>

          <div className="text-content/60 text-sm">
            $<NumberToLocaleString value={reward.amount_usd} />
          </div>
        </div>
      </div>
    </button>
  );
};

const DetailsDialogContent = () => {
  const { authenticatedAgent } = useAuth();
  const [claimRewardsState, dispatchClaimRewardsState] = useAtom(
    ClaimRewardsStateReducerAtom
  );
  const [totalSelectedAmountUSD] = useAtom(TotalSelectedAmountUSDAtom);

  const claimRewards = useClaimRewards(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent
  );

  const handleClaimRewards = () => {
    claimRewards.mutate({
      tokens: claimRewardsState.rewards
        .filter((reward) => reward.is_selected)
        .map((reward) => reward.name),
    });
  };

  useEffect(() => {
    if (claimRewards.isIdle && claimRewardsState.is_step_details) {
      handleClaimRewards();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [claimRewards.isIdle, claimRewardsState.is_step_details]);

  useEffect(() => {
    return () => {
      claimRewards.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const onRetry = () => {
    claimRewards.reset();
    handleClaimRewards();
  };

  const onClose = () => {
    dispatchClaimRewardsState({
      type: "SET_IS_OPEN_DIALOG",
      value: false,
    });
  };

  return (
    <>
      {(claimRewards.isIdle || claimRewards.isPending) && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-6">
            <LoaderSpin />
            <div>Claiming rewards....</div>
          </div>
        </div>
      )}
      {claimRewards.isError && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-2">
            <div className="mb-2">Claim rewards error</div>
            <div className="text-content/60 text-center">
              {claimRewards.error.message}
            </div>
          </div>
          <div className="mt-4 flex items-center gap-2 w-full">
            <Button onClick={onRetry} className="w-full">
              Retry
            </Button>
            <Button onClick={onClose} className="w-full">
              Close
            </Button>
          </div>
        </div>
      )}
      {claimRewards.isSuccess && (
        <div className="flex flex-col items-center gap-4">
          <div className="text-center text-2xl xl:text-4xl">
            <span className="text-gold">Rewards</span> successfully claimed!
          </div>

          <div className="grid grid-cols-1 gap-4 w-full mt-4">
            {claimRewardsState.rewards
              .filter((reward) => reward.is_selected)
              .map((reward) => (
                <RewardItem key={reward.name} reward={reward} />
              ))}
          </div>
          <div className="flex justify-between items-center w-full px-2 mt-4">
            <div>Total redeemed</div>
            <div className="text-content/60">
              $<NumberToLocaleString value={totalSelectedAmountUSD} />
            </div>
          </div>
          <Button size="lg" onClick={onClose} className="mt-4 w-full">
            Close
          </Button>
        </div>
      )}
    </>
  );
};

export default DetailsDialogContent;
