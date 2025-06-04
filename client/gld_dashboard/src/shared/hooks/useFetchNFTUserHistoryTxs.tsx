import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  useInfiniteQuery,
  UseInfiniteQueryOptions,
  keepPreviousData,
} from "@tanstack/react-query";
import { idlFactory } from "@services/gldt_swap/idlFactory";
import { SwapData } from "@services/gldt_swap/utils/index";
import get_historic_swaps_by_user from "@services/gldt_swap/fn/get_historic_swaps_by_user";
import get_history_total from "@services/gldt_swap/fn/get_history_total";

export interface FetchNFTUserHistoryTxsParams {
  data: SwapData[];
  total_count: number;
  page: number;
  page_count: number;
}

const useFetchNFTUserHistoryTxs = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseInfiniteQueryOptions<FetchNFTUserHistoryTxsParams>,
    "queryKey" | "queryFn" | "getNextPageParam" | "initialPageParam"
  > & {
    limit: number;
    principal: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    principal,
    limit,
  } = options;

  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });

  return useInfiniteQuery({
    queryKey: ["FETCH_NFT_HISTORY_TXS", principal, limit],
    queryFn: async ({ pageParam }): Promise<FetchNFTUserHistoryTxsParams> => {
      const history = await get_historic_swaps_by_user(actor, {
        page: pageParam as number,
        principal,
        limit,
      });
      const total_count = await get_history_total(actor, principal);
      return {
        total_count,
        page: pageParam as number,
        page_count: Math.ceil(total_count / limit),
        data: history,
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

export default useFetchNFTUserHistoryTxs;
