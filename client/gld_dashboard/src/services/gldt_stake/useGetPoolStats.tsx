import { useQuery, UseQueryOptions } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "./idlFactory";

import { StakePositionResponse } from "@services/gldt_stake/interfaces";

export const useGetPoolStats = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<StakePositionResponse[]>,
    "queryKey" | "queryFn"
  > & { owner: string }
) => {
  const { enabled = true, refetchOnWindowFocus = false, owner } = options;

  const total_staked_query = useQuery({
    queryKey: ["FETCH_TOTAL_STAKED", owner],
    queryFn: async (): Promise<StakePositionResponse[]> => {
      const actor = Actor.createActor(idlFactory, {
        agent,
        canisterId,
      });

      const res = (await actor.get_total_staked(
        null
      )) as StakePositionResponse[];
      return res;
    },
    enabled,
    refetchOnWindowFocus,
  });

  return {
    total_staked_query,
  };
};
