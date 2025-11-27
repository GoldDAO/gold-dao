import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { idlFactory as idlFactoryNFT } from "@services/nft/idlFactory";
import icrc7_tokens_of from "@services/nft/icrc7_tokens_of";
import get_active_swaps_by_user from "@services/swap/get_active_swaps_by_user";
import { idlFactory as idlFactorySwap } from "@services/swap/idlFactory";

const useFetchNFTUserCount = (
  canister_id: string,
  options: Omit<UseQueryOptions<number>, "queryKey" | "queryFn"> & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
    subaccount?: string[];
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    owner,
    subaccount,
    agent,
  } = options;

  return useQuery({
    queryKey: ["FETCH_USER_NFT_COUNT", canister_id, owner, subaccount],
    queryFn: async () => {
      try {
        const actorNFT = Actor.createActor(idlFactoryNFT, {
          agent,
          canisterId: canister_id,
        });
        const actorSwap = Actor.createActor(idlFactorySwap, {
          agent,
          canisterId: SWAP_CANISTER_ID,
        });

        const activeSwaps = await get_active_swaps_by_user(actorSwap, owner);
        const icrc7_tokens_of_results = await icrc7_tokens_of(actorNFT, {
          owner,
          subaccount,
        });

        const activeSwapSet = new Set(activeSwaps.map((swap) => swap.nft_id));
        const count = icrc7_tokens_of_results.filter(
          (token_id) => !activeSwapSet.has(token_id)
        ).length;

        return count;
      } catch (err) {
        console.error(err);
        throw new Error(`Fetch user NFTs count error! Please retry later.`);
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchNFTUserCount;
