import { useQuery, keepPreviousData } from "@tanstack/react-query";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../services/gldt_stake/idlFactory";
import get_all_stake_positions from "../services/gldt_stake/get_all_stake_positions";

const useGetNumberOfStakers = (canister_id_gldt_stake, options = {}) => {
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
    queryKey: ["GET_NUMBER_OF_STAKERS", canister_id_gldt_stake],
    queryFn: async () => {
      try {
        const agent = await HttpAgent.create({ host: "https://ic0.app" });
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId: canister_id_gldt_stake,
        });

        const stake_positions = await get_all_stake_positions(actor);

        return stake_positions.length;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch number of stakers error");
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

export default useGetNumberOfStakers;
