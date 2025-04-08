import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { GLDT_STAKE_CANISTER_ID } from "@constants";

import { idlFactory } from "@services/gldt_stake/idlFactory";
import get_active_user_positions from "@services/gldt_stake/get_active_user_positions";
import { PositionRewards } from "./index";

const useGetAllStakePositionRewards = (
  options: Omit<
    UseQueryOptions<PositionRewards[], Error>,
    "queryKey" | "queryFn"
  > & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    agent,
    owner,
  } = options;

  return useQuery({
    queryKey: ["USER_POSITIONS_ALL_TOTAL_STAKED_REWARDS", owner],
    queryFn: async (): Promise<PositionRewards[]> => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId: GLDT_STAKE_CANISTER_ID,
        });

        const positions = await get_active_user_positions(actor);

        const data = ["GOLDAO", "OGY", "ICP"].map((token) => {
          const res = positions.map((position) => {
            const filtered = position.claimable_rewards.filter(
              ([name]) => name === token
            );

            const positionRewards = filtered.map(([, amount]) => {
              return {
                id: position.id,
                amount: amount,
              };
            });

            return positionRewards;
          });
          return {
            name: token,
            positions: res.flatMap((item) => item),
            amount: res
              .flatMap((item) => item)
              .reduce((acc, curr) => acc + curr.amount, 0n),
          };
        });
        return data;
      } catch (err) {
        console.log(err);
        throw new Error(
          "Fetch all stake position rewards error! Please retry later."
        );
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useGetAllStakePositionRewards;
