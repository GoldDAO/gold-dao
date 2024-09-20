import { useEffect, useState } from "react";
import {
  useQuery,
  UseQueryOptions,
  QueryKey,
  keepPreviousData,
} from "@tanstack/react-query";
import { useWallet, getActor } from "@amerej/artemis-react";

import { canisters } from "@providers/Auth";

import { SwapInfo, SwapData } from "@canisters/gldt_swap/interfaces";
import { getSwapData } from "./utils/index";

type UseGetActiveSwapsParams = Omit<
  UseQueryOptions<Array<[[bigint, bigint], SwapInfo]>>,
  "queryKey" | "queryFn"
>;

const get_active_swaps = async () => {
  const { canisterId, idlFactory } = canisters["gldt_swap"];
  const actor = await getActor(canisterId, idlFactory, {
    isAnon: false,
  });
  const result = (await actor.get_active_swaps(null)) as Array<
    [[bigint, bigint], SwapInfo]
  >;
  return result;
};

export const useGetActiveSwaps = ({
  ...queryParams
}: UseGetActiveSwapsParams = {}) => {
  const { isConnected } = useWallet();
  const [data, setData] = useState<{ rows: SwapData[] } | null>(null);
  const [isInitializing, setIsInitializing] = useState(true);
  const [error, setError] = useState("");

  const active_swaps = useQuery({
    queryKey: ["FETCH_ACTIVE_SWAPS"] as QueryKey,
    queryFn: () => get_active_swaps(),
    enabled: !!isConnected && queryParams.enabled !== false,
    placeholderData: keepPreviousData,
    ...queryParams,
  });

  useEffect(() => {
    if (active_swaps.isLoading) {
      setIsInitializing(true);
    } else if (active_swaps.isSuccess) {
      const rows = active_swaps.data.map((r) => getSwapData(r[1]));
      setData({
        rows,
      });
      setIsInitializing(false);
    } else if (active_swaps.isError) {
      console.log(active_swaps.error);
      setError("Error while fetching active swaps :(.");
      setIsInitializing(false);
    }
  }, [
    active_swaps.isLoading,
    active_swaps.isSuccess,
    active_swaps.isError,
    active_swaps.data,
    active_swaps.error,
  ]);

  return {
    data,
    isSuccess: active_swaps.isSuccess && !isInitializing,
    isError: active_swaps.isError,
    error,
    isLoading: isInitializing,
  };
};
