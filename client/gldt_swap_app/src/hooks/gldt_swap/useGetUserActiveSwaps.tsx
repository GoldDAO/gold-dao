import { useEffect, useState } from "react";
import {
  useQuery,
  UseQueryOptions,
  QueryKey,
  keepPreviousData,
} from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { useWallet, getActor } from "@amerej/artemis-react";

import { canisters } from "@providers/Auth";

import { SwapInfo, SwapData } from "@canisters/gldt_swap/interfaces";
import { getSwapData } from "./utils/index";

interface GetUserActiveSwapsParams {
  principal: string;
}
interface UseGetUserActiveSwapsParams
  extends Partial<GetUserActiveSwapsParams>,
    Omit<
      UseQueryOptions<Array<[[bigint, bigint], SwapInfo]>>,
      "queryKey" | "queryFn"
    > {}

const get_active_swaps_by_user = async ({
  principal,
}: GetUserActiveSwapsParams) => {
  const { canisterId, idlFactory } = canisters["gldt_swap"];
  const actor = await getActor(canisterId, idlFactory, {
    isAnon: false,
  });
  const result = (await actor.get_active_swaps_by_user([
    Principal.fromText(principal),
  ])) as Array<[[bigint, bigint], SwapInfo]>;

  return result;
};

export const useGetUserActiveSwaps = ({
  ...queryParams
}: UseGetUserActiveSwapsParams) => {
  const { isConnected, principalId } = useWallet();
  const [data, setData] = useState<{ rows: SwapData[] } | null>(null);
  const [isInitializing, setIsInitializing] = useState(true);

  const active_swaps = useQuery({
    queryKey: ["USER_FETCH_ACTIVE_SWAP", principalId] as QueryKey,
    queryFn: () =>
      get_active_swaps_by_user({
        principal: principalId as string,
      }),
    enabled: !!isConnected && !!principalId && queryParams.enabled !== false,
    placeholderData: keepPreviousData,
    ...queryParams,
  });

  useEffect(() => {
    if (active_swaps.isLoading) {
      setIsInitializing(true);
    }
  }, [active_swaps.isLoading]);

  useEffect(() => {
    if (active_swaps.isSuccess && active_swaps.data) {
      console.log(active_swaps.data);
      const rows = active_swaps.data.map((r) => getSwapData(r[1]));
      setData({
        rows,
      });
      setIsInitializing(false);
    }
  }, [active_swaps.isSuccess, active_swaps.data]);

  return {
    isSuccess: active_swaps.isSuccess && !isInitializing,
    data,
    isError: active_swaps.isError,
    error: active_swaps.error,
    isLoading: isInitializing,
  };
};
