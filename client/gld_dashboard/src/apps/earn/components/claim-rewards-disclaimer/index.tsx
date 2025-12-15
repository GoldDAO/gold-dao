import clsx from "clsx";
import { useAtom } from "jotai";
import { ClaimRewardsStateReducerAtom } from "../claim-rewards/atoms";
import RewardsAvailable from "./RewardsAvailable";
import ClaimRewards from "../claim-rewards";

import { Position } from "@earn/interfaces";

const styles = {
  buttonClaim: clsx(
    "bg-success text-white border border-success rounded-xl",
    "px-6 py-3 text-sm shrink-0 cursor-pointer",
    "disabled:cursor-not-allowed disabled:opacity-50"
  ),
};

const ClaimRewardsDisclaimer = ({ position }: { position: Position }) => {
  const [, dispatchClaimRewards] = useAtom(ClaimRewardsStateReducerAtom);

  const onOpen = () => {
    dispatchClaimRewards({
      type: "SET_IS_OPEN_DIALOG",
      value: true,
    });
  };

  return (
    <>
      <RewardsAvailable
        amount={position.total_rewards_amount_usd}
        claimRewardsButton={
          <button type="button" className={styles.buttonClaim} onClick={onOpen}>
            Claim rewards
          </button>
        }
      />
      {position.is_enable_claiming_rewards && <ClaimRewards />}
    </>
  );
};

export default ClaimRewardsDisclaimer;
