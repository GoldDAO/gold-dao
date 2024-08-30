import { useEffect, useState } from "react";
import {
  useQuery,
  UseQueryOptions,
  QueryKey,
  keepPreviousData,
} from "@tanstack/react-query";
import { useWallet, getActor } from "@amerej/artemis-react";

import { canisters } from "@providers/Auth";

import { SwapInfo } from "@canisters/gldt_swap/interfaces";
import { SwapData } from "./interfaces";
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

const get_swap = async ({ nft_id, index }: GetOneSwapByIdParams) => {
  const { canisterId, idlFactory } = canisters["gldt_swap"];
  const actor = await getActor(canisterId, idlFactory, {
    isAnon: false,
  });
  const result = (await actor.get_swap([BigInt(nft_id), BigInt(index)])) as
    | []
    | [[[bigint, bigint], SwapInfo]];
  return result;
};

export const useGetOneSwapById = ({
  nft_id,
  index,
  ...queryParams
}: UseGetOneSwapByIdParams) => {
  const { isConnected, principalId } = useWallet();
  const [data, setData] = useState<SwapData | null>(null);
  const [isSuccess, setIsSuccess] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

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
