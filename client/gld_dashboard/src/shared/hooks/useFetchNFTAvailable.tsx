import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { idlFactory as idlFactoryNFT } from "@services/nft/idlFactory";
import { NFT } from "@services/nft/utils/interfaces";
import icrc7_tokens_of from "@services/nft/icrc7_tokens_of";
import get_active_swaps from "@services/swap/get_active_swaps";
import { idlFactory as idlFactorySwap } from "@services/swap/idlFactory";
import icrc7_token_metadata from "@services/nft/icrc7_token_metadata";
import { parseMetadata } from "@services/nft/utils";

const useFetchNFTAvailable = (
  canisterId: string,
  options: Omit<UseQueryOptions<NFT[]>, "queryKey" | "queryFn"> & {
    subaccount?: string[];
    agent: Agent | HttpAgent | undefined;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    subaccount,
    agent,
  } = options;

  return useQuery({
    queryKey: ["FETCH_AVAILABLE_NFT", canisterId, subaccount],
    queryFn: async () => {
      try {
        const actorNFT = Actor.createActor(idlFactoryNFT, {
          agent: agent,
          canisterId,
        });
        const actorSwap = Actor.createActor(idlFactorySwap, {
          agent: agent,
          canisterId: SWAP_CANISTER_ID,
        });

        const icrc7_tokens_of_results = await icrc7_tokens_of(actorNFT, {
          owner: SWAP_CANISTER_ID,
          subaccount,
        });

        const nfts = await Promise.all(
          icrc7_tokens_of_results.map(
            async (token_id: bigint): Promise<NFT> => {
              const result = await icrc7_token_metadata(actorNFT, {
                token_id,
              });
              const metadata = await parseMetadata(result);

              return {
                id: token_id,
                ...metadata,
              };
            }
          )
        );

        // ? Filter out NFT's that are currently being swapped
        const data = await get_active_swaps(actorSwap);
        const activeSwapSet = new Set(data.map((swap) => swap.nft_id));
        const result = nfts.filter((nft) => !activeSwapSet.has(nft.id));

        return result ?? [];
      } catch (err) {
        console.error(err);
        throw new Error(
          `Fetch available NFT's collection error! Please retry later.`
        );
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchNFTAvailable;
