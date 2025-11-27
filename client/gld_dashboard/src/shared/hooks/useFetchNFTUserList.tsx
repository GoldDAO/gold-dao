import {
  useInfiniteQuery,
  UseInfiniteQueryOptions,
  keepPreviousData,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { idlFactory as idlFactoryNFT } from "@services/nft/idlFactory";
import { NFT } from "@services/nft/utils/interfaces";
import icrc7_tokens_of from "@services/nft/icrc7_tokens_of";
import get_active_swaps_by_user from "@services/swap/get_active_swaps_by_user";
import icrc7_token_metadata from "@services/nft/icrc7_token_metadata";
import { idlFactory as idlFactorySwap } from "@services/swap/idlFactory";
import { parseMetadata } from "@services/nft/utils/index";

export interface FetchNFTUserParams {
  data: NFT[];
  page: number;
  page_count: number;
  total_count: number;
}

const useFetchUserNft = (
  canister_id: string,
  options: Omit<
    UseInfiniteQueryOptions<FetchNFTUserParams>,
    "queryKey" | "queryFn" | "getNextPageParam" | "initialPageParam"
  > & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
    subaccount?: string[];
    limit: number;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    owner,
    subaccount,
    agent,
    limit,
  } = options;

  const actorNFT = Actor.createActor(idlFactoryNFT, {
    agent,
    canisterId: canister_id,
  });

  const actorSwap = Actor.createActor(idlFactorySwap, {
    agent,
    canisterId: SWAP_CANISTER_ID,
  });

  return useInfiniteQuery({
    queryKey: ["FETCH_USER_NFT", canister_id, owner, subaccount, limit],
    queryFn: async ({ pageParam }): Promise<FetchNFTUserParams> => {
      const activeSwaps = await get_active_swaps_by_user(actorSwap, owner);
      const activeSwapSet = new Set(activeSwaps.map((swap) => swap.nft_id));

      const icrc7_tokens_of_results = await icrc7_tokens_of(actorNFT, {
        owner,
        subaccount,
      });

      const filteredTokens = icrc7_tokens_of_results.filter(
        (token_id) => !activeSwapSet.has(token_id)
      );

      const total_count = filteredTokens.length;
      const start = (pageParam as number) * limit;
      const end = start + limit;
      const pageTokens = filteredTokens.slice(start, end);

      const data = await Promise.all(
        pageTokens.map(async (token_id: bigint): Promise<NFT> => {
          const result = await icrc7_token_metadata(actorNFT, {
            token_id,
          });
          const metadata = await parseMetadata(result);

          return {
            id: token_id,
            ...metadata,
          };
        })
      );

      return {
        data,
        page: pageParam as number,
        page_count: Math.ceil(total_count / limit),
        total_count,
      };
    },
    getNextPageParam: (lastPage, allPages) => {
      const nextPage = allPages.length;
      if (nextPage < lastPage.page_count) {
        return nextPage;
      }
      return undefined;
    },
    initialPageParam: 0,
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchUserNft;
