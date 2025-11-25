import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  useInfiniteQuery,
  UseInfiniteQueryOptions,
  keepPreviousData,
} from "@tanstack/react-query";
import { idlFactory as idlFactoryCollection } from "@services/nft/idlFactory";
import { idlFactory as idlFactoryCollectionIndexer } from "@services/nft_indexer/idlFactory";
import get_blocks from "@services/nft_indexer/get_blocks";
import type { Args, Account, SortBy } from "@services/nft_indexer/interfaces";
import { Principal } from "@dfinity/principal";
import icrc7_token_metadata from "@services/nft/icrc7_token_metadata";
import { parseMetadata } from "@services/nft/utils/index";
import parseBlockWithId, { BlockTx } from "@shared/utils/parsers/BlockWithId";

export interface FetchNFTBlocksParams {
  data: BlockTx[];
  page: number;
  page_count: number;
  total_count: number;
}

const useFetchNFTBlocks = (
  options: Omit<
    UseInfiniteQueryOptions<FetchNFTBlocksParams>,
    "queryKey" | "queryFn" | "getNextPageParam" | "initialPageParam"
  > & {
    unauthenticated_agent: Agent | HttpAgent | undefined;
    canister_id_collection: string;
    canister_id_collection_indexer: string;
    limit: number;
    principal: string;
    sort_by?: SortBy;
  }
) => {
  const {
    unauthenticated_agent,
    canister_id_collection,
    canister_id_collection_indexer,
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    principal,
    limit,
    sort_by,
  } = options;

  const actorCollection = Actor.createActor(idlFactoryCollection, {
    agent: unauthenticated_agent,
    canisterId: canister_id_collection,
  });

  const actorCollectionIndexer = Actor.createActor(
    idlFactoryCollectionIndexer,
    {
      agent: unauthenticated_agent,
      canisterId: canister_id_collection_indexer,
    }
  );

  return useInfiniteQuery({
    queryKey: [
      "FETCH_NFT_BLOCKS",
      canister_id_collection,
      principal,
      limit,
      sort_by,
    ],
    queryFn: async ({ pageParam }): Promise<FetchNFTBlocksParams> => {
      const account: Account = {
        owner: Principal.fromText(principal),
        subaccount: [],
      };

      const args: Args = {
        sort_by: sort_by ? [sort_by] : [],
        filters: [{ Account: account }],
        start: BigInt((pageParam as number) * limit),
        length: BigInt(limit),
      };

      const response = await get_blocks(actorCollectionIndexer, args);
      const total_count = Number(response.total);
      const blocks = response.blocks.map(parseBlockWithId);

      const data = await Promise.all(
        blocks.map(async (block: BlockTx): Promise<BlockTx> => {
          const result = await icrc7_token_metadata(actorCollection, {
            token_id: block.nft_id,
          });
          const metadata = await parseMetadata(result);

          return {
            ...block,
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

export default useFetchNFTBlocks;
