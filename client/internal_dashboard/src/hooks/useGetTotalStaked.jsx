import { useQuery, keepPreviousData } from "@tanstack/react-query";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryGLDTStake } from "../services/gldt_stake/idlFactory";
import { idlFactory as idlFactoryLedger } from "../services/ledger/idlFactory";
import get_total_staked from "../services/gldt_stake/get_total_staked";
import icrc1_decimals from "../services/ledger/icrc1_decimals";

const useGetTotalStaked = (
  canister_id_gldt_stake,
  canister_id_gldt_ledger,
  options = {}
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
    queryKey: ["GET_TOTAL_STAKED", canister_id_gldt_stake],
    queryFn: async () => {
      try {
        const agent = await HttpAgent.create({ host: "https://ic0.app" });
        const actor = Actor.createActor(idlFactoryGLDTStake, {
          agent,
          canisterId: canister_id_gldt_stake,
        });
        const actorLedger = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: canister_id_gldt_ledger,
        });

        const decimals = await icrc1_decimals(actorLedger);
        const total_staked_e8s = await get_total_staked(actor);
        const total_staked = Number(total_staked_e8s) / 10 ** decimals;

        return total_staked;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch total staked error");
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

export default useGetTotalStaked;
