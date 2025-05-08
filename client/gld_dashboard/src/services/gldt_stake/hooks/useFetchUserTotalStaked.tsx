import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";

import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

import get_active_user_positions from "../get_active_user_positions";

const useFetchUserTotalStaked = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<bigint, Error>, "queryKey" | "queryFn">
) => {
  const {
    enabled,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["USER_POSITIONS_TOTAL_STAKED_AMOUNT"],
    queryFn: async (): Promise<bigint> => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const res = await get_active_user_positions(actor);

        return res.reduce(
          (acc, stake) => acc + BigInt(stake.staked),
          BigInt(0)
        );
      } catch (err) {
        console.log(err);
        throw new Error(
          "Fetch user positions total staked amount error! Please retry later."
        );
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useFetchUserTotalStaked;
