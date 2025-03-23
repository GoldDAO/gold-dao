import { useQuery, UseQueryOptions } from "@tanstack/react-query";

import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "./idlFactory";

import { StakePositionResponse } from "@services/gldt_stake/interfaces";

export const useGetUserStakePositions = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<StakePositionResponse[]>,
    "queryKey" | "queryFn"
  > & { owner: string }
) => {
  const { enabled = true, refetchOnWindowFocus = false, owner } = options;

  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });

  const positions_query = useQuery({
    queryKey: ["FETCH_USER_STAKE_POSITIONS", owner],
    queryFn: async (): Promise<StakePositionResponse[]> => {
      const res = (await actor.get_active_user_positions(
        []
      )) as StakePositionResponse[];
      return res;
    },
    enabled,
    refetchOnWindowFocus,
  });

  return {
    positions_query,
    isSuccess: positions_query.isSuccess,
    isLoading: positions_query.isLoading,
    isError: positions_query.isError,
    error: positions_query.error?.message ?? "",
  };
};
