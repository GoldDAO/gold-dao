import { useEffect, useState } from "react";
import {
  useQuery,
  UseQueryOptions,
  QueryKey,
  keepPreviousData,
} from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { useAuth } from "@auth/index";

interface GetUserHistoricSwapTotalParams {
  principal: string;
}

interface UseGetUserHistoricSwapParams
  extends Partial<GetUserHistoricSwapTotalParams>,
    Omit<UseQueryOptions<bigint>, "queryKey" | "queryFn"> {}

export const useGetUserHistoricCountSwap = ({
  ...queryParams
}: UseGetUserHistoricSwapParams) => {
  const { isConnected, principalId, createActor } = useAuth();
  const [data, setData] = useState<number | null>(null);
  const [isInitializing, setIsInitializing] = useState(true);

  const get_history_total = async ({
    principal,
  }: GetUserHistoricSwapTotalParams): Promise<bigint> => {
    const actor = createActor("gldt_swap");
    const result = await actor.get_history_total([
      Principal.fromText(principal),
    ]);
    return result as bigint;
  };

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
    } else if (historic_count.isSuccess) {
      setData(Number(historic_count?.data ?? 0));
      setIsInitializing(false);
    } else if (historic_count.isError) {
      setData(0);
      console.log(historic_count.error);
      setIsInitializing(false);
    }
  }, [
    historic_count?.data,
    historic_count.error,
    historic_count.isError,
    historic_count.isLoading,
    historic_count.isSuccess,
  ]);

  return {
    isSuccess: historic_count.isSuccess && !isInitializing,
    data,
    isError: historic_count.isError,
    error: historic_count.error,
    isLoading: isInitializing,
  };
};
