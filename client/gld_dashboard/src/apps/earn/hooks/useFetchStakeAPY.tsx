import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "@services/gldt_stake/interfaces/idlFactory";
import get_apy_overall from "@services/gldt_stake/get_apy_overall";

const useFetchStakeAPY = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<number, Error>, "queryKey" | "queryFn">
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    ...queryOptions
  } = options;

  return useQuery({
    queryKey: ["FETCH_STAKE_APY"],
    queryFn: async (): Promise<number> => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const res = await get_apy_overall(actor);

        return res;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch stake APY error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
    staleTime,
    refetchOnMount,
    refetchOnWindowFocus,
    refetchOnReconnect,
    ...queryOptions,
  });
};

export default useFetchStakeAPY;
