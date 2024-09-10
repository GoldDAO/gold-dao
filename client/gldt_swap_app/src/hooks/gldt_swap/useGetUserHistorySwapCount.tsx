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

interface GetUserHistoricSwapTotalParams {
  principal: string;
}

interface UseGetUserHistoricSwapParams
  extends Partial<GetUserHistoricSwapTotalParams>,
    Omit<UseQueryOptions<bigint>, "queryKey" | "queryFn"> {}

const get_history_total = async ({
  principal,
}: GetUserHistoricSwapTotalParams): Promise<bigint> => {
  const { canisterId, idlFactory } = canisters["gldt_swap"];
  const actor = await getActor(canisterId, idlFactory, {
    isAnon: false,
  });
  const result = await actor.get_history_total([Principal.fromText(principal)]);
  return result as bigint;
};

export const useGetUserHistoricCountSwap = ({
  ...queryParams
}: UseGetUserHistoricSwapParams) => {
  const { isConnected, principalId } = useWallet();
  const [data, setData] = useState<number | null>(null);
  const [isInitializing, setIsInitializing] = useState(true);

  const historic_count = useQuery({
    queryKey: [
      "USER_FETCH_TRANSACTIONS_HISTORY_COUNT",
      principalId,
    ] as QueryKey,
    queryFn: () =>
      get_history_total({
        principal: principalId as string,
      }),
    enabled: !!isConnected && !!principalId && queryParams.enabled !== false,
    placeholderData: keepPreviousData,
    ...queryParams,
  });

  useEffect(() => {
    if (historic_count.isLoading) {
      setIsInitializing(true);
    }
  }, [historic_count.isLoading]);

  useEffect(() => {
    if (historic_count.isError) {
      setIsInitializing(false);
    }
  }, [historic_count.isError]);

  useEffect(() => {
    if (historic_count.isSuccess && historic_count.data) {
      setData(Number(historic_count?.data ?? 0));
      setIsInitializing(false);
    }
  }, [historic_count.isSuccess, historic_count.data, historic_count]);

  return {
    isSuccess: historic_count.isSuccess && !isInitializing,
    data,
    isError: historic_count.isError,
    error: historic_count.error,
    isLoading: isInitializing,
  };
};
