import clsx from "clsx";
import { UseQueryResult } from "@tanstack/react-query";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { ClaimRewardsStateReducerAtom } from "../claim-rewards/atoms";
import RewardsAvailable from "./RewardsAvailable";
import RewardsNotAvailable from "./RewardsNotAvailable";
import RewardsLoading from "./RewardsLoading";
import ClaimRewards from "../claim-rewards";

import { Position } from "@earn/interfaces";

const styles = {
  buttonClaim: clsx(
    "bg-success text-white border border-success rounded-xl",
    "px-6 py-3 text-sm shrink-0 cursor-pointer",
    "disabled:cursor-not-allowed disabled:opacity-50"
  ),
};

const ClaimRewardsDisclaimer = ({
  position,
}: {
  position: UseQueryResult<Position, Error>;
}) => {
  const { isConnected } = useAuth();
  const [, dispatchClaimRewards] = useAtom(ClaimRewardsStateReducerAtom);

  if (!isConnected) return <RewardsNotAvailable />;

  const onOpen = () => {
    dispatchClaimRewards({
      type: "SET_IS_OPEN_DIALOG",
      value: true,
    });
  };

  return (
    <>
      {!position.isSuccess && <RewardsLoading />}
      {position.isSuccess && position.data.is_enable_claiming_rewards && (
        <RewardsAvailable
          amount={position.data.total_rewards_amount_usd}
          claimRewardsButton={
            <button
              type="button"
              className={styles.buttonClaim}
              onClick={onOpen}
            >
              Claim rewards
            </button>
          }
        />
      )}
      {position.isSuccess && !position.data.is_enable_claiming_rewards && (
        <RewardsNotAvailable />
      )}
      {position.isSuccess && <ClaimRewards />}
    </>
  );
};

export default ClaimRewardsDisclaimer;
