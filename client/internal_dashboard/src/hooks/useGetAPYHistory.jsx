import { useQuery, keepPreviousData } from "@tanstack/react-query";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../services/gldt_stake/idlFactory";
import get_apy_timeseries from "../services/gldt_stake/get_apy_timeseries";

const useGetAPYHistory = (canister_id_gldt_stake, options = {}) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    starting_day = 0,
    limit = 7,
    ...queryOptions
  } = options;

  return useQuery({
    queryKey: ["GET_APY_HISTORY", canister_id_gldt_stake],
    queryFn: async () => {
      try {
        const agent = await HttpAgent.create({ host: "https://ic0.app" });
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId: canister_id_gldt_stake,
        });

        const apy_timeseries = await get_apy_timeseries(
          actor,
          starting_day,
          limit
        );
        const res = apy_timeseries
          .map(([timestamp, percentage]) => {
            if (timestamp < 1_756_425_600_000) {
              percentage = 0;
            }
            return {
              date: new Date(Number(timestamp)).toISOString().split("T")[0],
              value: Number(percentage.toFixed(2)),
            };
          })
          .reverse();

        return res;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch current APY error");
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

export default useGetAPYHistory;
