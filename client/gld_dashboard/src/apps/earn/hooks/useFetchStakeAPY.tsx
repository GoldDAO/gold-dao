import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "@services/stake/idlFactory";
import get_apy_timeseries from "@services/stake/get_apy_timeseries";

const useFetchStakeAPY = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<number, Error>, "queryKey" | "queryFn"> & {
    starting_day?: number;
    limit?: number;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    starting_day = 0,
    limit = 1,
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

        const res = await get_apy_timeseries(actor, starting_day, limit);

        return res[0][1];
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
