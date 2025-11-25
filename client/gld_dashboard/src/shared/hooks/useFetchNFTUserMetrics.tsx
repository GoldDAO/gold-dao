import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { NFT, NFTCollection } from "@services/nft/utils/interfaces";
import { idlFactory as idlFactoryNFT } from "@services/nft/idlFactory";
import { idlFactory as idlFactorySwap } from "@services/swap/idlFactory";
import icrc7_tokens_of from "@services/nft/icrc7_tokens_of";
import get_active_swaps_by_user from "@services/swap/get_active_swaps_by_user";
import icrc7_token_metadata from "@services/nft/icrc7_token_metadata";
import { parseMetadata } from "@services/nft/utils";

const useFetchUserNFTMetrics = (
  options: Omit<
    UseQueryOptions<{
      totalCount: number;
      totalGrams: number;
      totalUSD: number;
    }>,
    "queryKey" | "queryFn"
  > & {
    owner: string;
    nft_collections: NFTCollection[];
    agent: Agent | HttpAgent | undefined;
  }
) => {
  const {
    owner,
    agent,
    nft_collections,
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
    queryKey: ["FETCH_USER_NFT_METRICS", owner],
    queryFn: async () => {
      const actorSwap = Actor.createActor(idlFactorySwap, {
        agent,
        canisterId: SWAP_CANISTER_ID,
      });
      const activeSwaps = await get_active_swaps_by_user(actorSwap, owner);

      // const priceGoldUSD = await fetch_gold_price();

      const results = await Promise.all(
        nft_collections.map(async ({ canisterId, grams }) => {
          const actorNFT = Actor.createActor(idlFactoryNFT, {
            agent: agent,
            canisterId,
          });
          const icrc7_tokens_of_results = await icrc7_tokens_of(actorNFT, {
            owner,
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

          //? Filter out NFT's that are currently being swapped
          const activeSwapSet = new Set(activeSwaps.map((swap) => swap.nft_id));
          const filtered = nfts.filter((nft) => !activeSwapSet.has(nft.id));
          return { count: filtered.length, grams: filtered.length * grams };
        })
      );

      const totalCount = results.reduce((acc, cur) => acc + cur.count, 0);
      const totalGrams = results.reduce((acc, cur) => acc + cur.grams, 0);
      const totalUSD = totalGrams;

      return { totalCount, totalGrams, totalUSD };
    },
    placeholderData,
    enabled,
    refetchInterval,
    staleTime,
    refetchOnMount,
    refetchOnWindowFocus,
    refetchOnReconnect,
    ...queryOptions,
  });
};

export default useFetchUserNFTMetrics;
