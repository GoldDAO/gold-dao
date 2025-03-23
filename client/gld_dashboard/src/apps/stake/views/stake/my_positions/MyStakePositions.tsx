import { StakePositionResponse } from "@canisters/gldt_stake/interfaces/gldt_stake";
import { Button } from "@components/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useGetUserStakePositions } from "@services/gldt_stake/useGetUserStakePositions";
import { useStakePosition } from "@services/gldt_stake/useStakePosition";
import { useQueryClient } from "@tanstack/react-query";
import { divideBy1e8 } from "@utils/numbers";
import { DateTime } from "luxon";

import { useAuth } from "@auth/index";

const NoStakePositions = () => {
  return (
    <div className="flex justify-center">
      <p>No stake positions at the moment</p>
    </div>
  );
};

const Rewards = (position: StakePositionResponse) => {
  return position.claimable_rewards.map(([token, amount]) => {
    return (
      <p key={token}>
        - {token} : {divideBy1e8(amount)}
      </p>
    );
  });
};

const dissolveStateEquals = (
  position: StakePositionResponse,
  stateType: "Dissolving" | "NotDissolving" | "Dissolved"
): boolean => {
  return Object.keys(position.dissolve_state)[0] == stateType || false;
};

const canClaimRewards = (position: StakePositionResponse): boolean => {
  return (
    position.claimable_rewards.filter(([, amount]) => amount > 0).length > 0
  );
};

const canUnstake = (position: StakePositionResponse): boolean => {
  const afterDissolveDate =
    DateTime.now() > DateTime.fromMillis(Number(position.dissolved_date));
  return (
    Object.keys(position.dissolve_state)[0] == "Dissolving" && afterDissolveDate
  );
};

const canUnstakeEarly = (position: StakePositionResponse): boolean => {
  return Object.keys(position.dissolve_state)[0] == "NotDissolving";
};

export const MyStakePositions = () => {
  const { authenticatedAgent, principalId, isConnected } = useAuth();

  const queryClient = useQueryClient();

  const { positions_query } = useGetUserStakePositions(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    { owner: principalId, enabled: !!isConnected }
  );
  const {
    startDissolving,
    unstakeEarly,
    unstake,
    claimReward,
    isPositionLoading,
  } = useStakePosition(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  if (positions_query.isLoading)
    return (
      <div className="flex justify-center">
        <p>Loading...</p>
      </div>
    );
  if (positions_query.isError)
    return <p>Error: {positions_query.error?.message}</p>;

  const positions = positions_query.data || [];

  if (positions.length === 0) {
    return <NoStakePositions />;
  }

  const handleStartDissolving = async (position_id: bigint) => {
    try {
      await startDissolving(position_id);
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_STAKE_POSITIONS"],
      });
      positions_query.refetch();
    } catch (e) {
      console.log(e);
    }
  };

  const handleUnstakeEarly = async (position_id: bigint) => {
    try {
      await unstakeEarly(position_id);
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_STAKE_POSITIONS"],
      });
      positions_query.refetch();
    } catch (e) {
      console.log(e);
    }
  };

  const handleUnstake = async (position_id: bigint) => {
    try {
      await unstake(position_id);
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_STAKE_POSITIONS"],
      });
      positions_query.refetch();
    } catch (e) {
      console.log(e);
    }
  };

  const handleClaimRewards = async (position: StakePositionResponse) => {
    // TODO - check the amount is more than the transfer fee instead of 0.

    const rewards_to_claim = position.claimable_rewards.filter(
      ([, amount]) => amount > 0
    );

    if (rewards_to_claim.length > 0) {
      // promise.allSettled
      const promises = rewards_to_claim.map(([token]) => {
        return claimReward(position.id, token);
      });
      await Promise.allSettled(promises);
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_STAKE_POSITIONS"],
      });
      positions_query.refetch();
    }
  };

  return (
    <div className="flex justify-center">
      <div className="rounded-xl p-4 h-full shadow-[10px_10px_60px_-20px_rgba(252,148,88,100)] flex flex-col">
        {positions.map((position) => (
          <div
            key={position.id}
            className="stake-position flex flex-row justify-between gap-x-3 border rounded-xl bg-surface-primary p-6 [&:not(:last-child)]:mb-4 min-w-[600px]"
          >
            <div className="flex flex-col text-xs">
              <p>id : {`${position.id}`}</p>
              <p>Staked : {divideBy1e8(position.staked)} GLDT</p>
              <p>Age bonus {position.age_bonus_multiplier}</p>
              <div>
                <p>Rewards</p>
                {Rewards(position)}
              </div>
              <div>
                dissolve status : {Object.keys(position.dissolve_state)[0]}
              </div>
              {position.dissolved_date.length ? (
                <div>
                  Dissolved date:{" "}
                  {DateTime.fromMillis(
                    Number(position.dissolved_date)
                  ).toString()}
                </div>
              ) : (
                ""
              )}
              <div>
                created at :{" "}
                {DateTime.fromMillis(Number(position.created_at)).toString()}
              </div>
            </div>
            <div
              className="flex flex-col gap-y-2 justify-self-end"
              style={{
                pointerEvents: isPositionLoading(position.id) ? "none" : "auto",
                opacity: isPositionLoading(position.id) ? 0.5 : 1,
              }}
            >
              <Button
                className="align-center"
                disabled={!dissolveStateEquals(position, "NotDissolving")}
                onClick={() => {
                  handleStartDissolving(position.id);
                }}
              >
                Start Dissolving
              </Button>
              <Button
                className="align-center"
                disabled={!canClaimRewards(position)}
                onClick={() => handleClaimRewards(position)}
              >
                Claim Rewards
              </Button>
              <Button
                className="align-center"
                disabled={!canUnstake(position)}
                onClick={() => handleUnstake(position.id)}
              >
                Unstake
              </Button>
              <Button
                className="align-center"
                disabled={!canUnstakeEarly(position)}
                onClick={() => handleUnstakeEarly(position.id)}
              >
                Unstake Early
              </Button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
