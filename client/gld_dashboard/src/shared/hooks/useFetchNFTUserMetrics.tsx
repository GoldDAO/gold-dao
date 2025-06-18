import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { NFTCollection } from "@services/gld_nft/utils/interfaces";
import { idlFactory as idlFactoryNFT } from "@services/gld_nft/idlFactory";
import { idlFactory as idlFactorySwap } from "@services/gldt_swap/idlFactory";
import unlisted_tokens_of from "@services/gld_nft/fn/unlisted_tokens_of";
import get_nat_as_token_id_origyn from "@services/gld_nft/fn/get_nat_as_token_id_origyn";
import get_active_swaps_by_user from "@services/gldt_swap/fn/get_active_swaps_by_user";
import { bigIntTo32ByteArray } from "@services/gld_nft/utils";
import { fetch_gold_price } from "@services/bity_gold_api";

const useFetchUserNFTMetrics = (
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<{
      totalCount: number;
      totalGrams: number;
      totalUSD: number;
      priceGramUSD: number;
    }>,
    "queryKey" | "queryFn"
  > & {
    owner: string;
    nft_collections: NFTCollection[];
  }
) => {
  const {
    owner,
    nft_collections,
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["FETCH_USER_NFT_METRICS", owner],
    queryFn: async () => {
      const actorSwap = Actor.createActor(idlFactorySwap, {
        agent,
        canisterId: SWAP_CANISTER_ID,
      });
      const activeSwaps = await get_active_swaps_by_user(actorSwap, owner);
      const activeSwapSet = new Set(
        activeSwaps.map((swap) => swap.nft_id_string)
      );

      const priceGoldUSD = await fetch_gold_price();

      const results = await Promise.all(
        nft_collections.map(async ({ canisterId, grams }) => {
          const actorNFT = Actor.createActor(idlFactoryNFT, {
            agent,
            canisterId,
          });
          const tokens = await unlisted_tokens_of(actorNFT, {
            owner,
          });
          const nfts = await Promise.all(
            tokens.map(async (tokenId: bigint) => {
              const id_string = (await get_nat_as_token_id_origyn(actorNFT, {
                tokenId,
              })) as string;
              return {
                id_string,
                id_bigint: tokenId,
                id_byte_array: bigIntTo32ByteArray(tokenId),
              };
            })
          );

          //? Filter out NFT's that are currently being swapped
          const filtered = nfts.filter(
            (nft) => !activeSwapSet.has(nft.id_string)
          );
          return { count: filtered.length, grams: filtered.length * grams };
        })
      );

      const totalCount = results.reduce((acc, cur) => acc + cur.count, 0);
      const totalGrams = results.reduce((acc, cur) => acc + cur.grams, 0);
      const totalUSD = totalGrams * priceGoldUSD;

      return { totalCount, totalGrams, totalUSD, priceGramUSD: priceGoldUSD };
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchUserNFTMetrics;
