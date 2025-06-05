import {
  keepPreviousData,
  useQuery,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "@services/gldt_swap/idlFactory";
import get_active_swaps_by_user from "@services/gldt_swap/fn/get_active_swaps_by_user";
import { SwapData } from "@services/gldt_swap/utils";

const useFetchNFTOnGoingTxs = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<SwapData[]>, "queryKey" | "queryFn"> & {
    principal: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    principal,
  } = options;
  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });
  return useQuery({
    queryKey: ["FETCH_NFT_ONGOING_TXS"],
    queryFn: async (): Promise<SwapData[]> => {
      try {
        const data = await get_active_swaps_by_user(actor, principal);
        return data;
      } catch (err) {
        console.error(err);
        if (err instanceof Error) {
          throw new Error(err.message, { cause: err });
        } else {
          throw new Error("An unknown error occurred", { cause: err });
        }
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchNFTOnGoingTxs;
