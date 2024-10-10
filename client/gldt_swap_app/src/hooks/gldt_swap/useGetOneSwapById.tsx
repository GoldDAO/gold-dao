import { useEffect, useState } from "react";
import {
  useQuery,
  UseQueryOptions,
  QueryKey,
  keepPreviousData,
} from "@tanstack/react-query";

import { useAuth } from "@auth/index";

import { SwapInfo, SwapData } from "@canisters/gldt_swap/interfaces";
import { getSwapData } from "./utils/index";

interface GetOneSwapByIdParams {
  nft_id: string;
  index: string;
}

interface UseGetOneSwapByIdParams
  extends Partial<GetOneSwapByIdParams>,
    Omit<
      UseQueryOptions<[] | [[[bigint, bigint], SwapInfo]]>,
      "queryKey" | "queryFn"
    > {}

export const useGetOneSwapById = ({
  nft_id,
  index,
  ...queryParams
}: UseGetOneSwapByIdParams) => {
  const { isConnected, principalId, createActor } = useAuth();
  const [data, setData] = useState<SwapData | null>(null);
  const [isSuccess, setIsSuccess] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  const get_swap = async ({ nft_id, index }: GetOneSwapByIdParams) => {
    const actor = createActor("gldt_swap");
    const result = (await actor.get_swap([BigInt(nft_id), BigInt(index)])) as
      | []
      | [[[bigint, bigint], SwapInfo]];
    return result;
  };

  const {
    data: response,
    error: error,
    isSuccess: isSuccessResponse,
    isError: isError,
    isLoading: isLoadingResponse,
  } = useQuery({
    queryKey: ["FETCH_ONE_SWAP", nft_id, index] as QueryKey,
    queryFn: () =>
      get_swap({
        nft_id: nft_id as string,
        index: index as string,
      }),
    enabled: !!isConnected && !!principalId && queryParams.enabled !== false,
    placeholderData: keepPreviousData,
    ...queryParams,
  });

  useEffect(() => {
    if (isLoadingResponse) {
      setIsLoading(true);
    }
  }, [isLoadingResponse]);

  useEffect(() => {
    if (isSuccessResponse && response) {
      const _data = response.map((r) => getSwapData(r[1]));
      setData(_data[0] ?? null);
      setIsSuccess(true);
      setIsLoading(false);
    }
  }, [isSuccessResponse, response]);

  return {
    isSuccess: isSuccess && !isLoading,
    data,
    isError,
    error,
    isLoading,
  };
};
