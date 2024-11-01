import { useEffect, useState } from "react";
import {
  useQuery,
  UseQueryOptions,
  QueryKey,
  keepPreviousData,
  useQueryClient,
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
  const queryClient = useQueryClient();
  const { isConnected, principalId, createActor } = useAuth();
  const [counterTransactions, setCounterTransactions] = useState<
    number | undefined
  >(undefined);
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
      const count = Number(historic_count?.data ?? 0);
      setData(count);
      if (counterTransactions === undefined) {
        setCounterTransactions(count);
      }
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
    counterTransactions,
  ]);

  useEffect(() => {
    if (
      historic_count.isSuccess &&
      data &&
      counterTransactions !== undefined &&
      counterTransactions !== data
    ) {
      queryClient.refetchQueries({
        queryKey: ["USER_FETCH_BALANCE_GLDT"],
        type: "active",
      });
      queryClient.refetchQueries({
        queryKey: ["USER_FETCH_BALANCE_OGY"],
        type: "active",
      });
      queryClient.refetchQueries({
        queryKey: ["USER_FETCH_NFTS_METRICS"],
        type: "active",
      });
      queryClient.refetchQueries({
        queryKey: ["USER_FETCH_TRANSACTIONS_HISTORY"],
        type: "active",
      });
      setCounterTransactions(undefined);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [counterTransactions, data, historic_count.isSuccess]);

  return {
    isSuccess: historic_count.isSuccess && !isInitializing,
    data,
    isError: historic_count.isError,
    error: historic_count.error,
    isLoading: isInitializing,
  };
};
