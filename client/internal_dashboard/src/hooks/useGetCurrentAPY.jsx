import { useQuery, keepPreviousData } from "@tanstack/react-query";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../services/gldt_stake/idlFactory";
import get_apy_timeseries from "../services/gldt_stake/get_apy_timeseries";

const useGetCurrentAPY = (canister_id_gldt_stake, options = {}) => {
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
    queryKey: ["GET_CURRENT_APY", canister_id_gldt_stake],
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
        if (apy_timeseries.length === 0) {
          return 0;
        }
        return apy_timeseries[0][1];
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

export default useGetCurrentAPY;
