import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import get_apy_timeseries from "@hooks/stake/get_apy_timeseries";

import { useAuth } from "@auth/index";

const useFetchStakeAPY = (
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

  const { createActor } = useAuth();

  return useQuery({
    queryKey: ["FETCH_STAKE_APY"],
    queryFn: async (): Promise<number> => {
      try {
        const actor = createActor("gldt_stake");

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
