import { useEffect, useState } from "react";
import {
  useQuery,
  UseQueryOptions,
  QueryKey,
  keepPreviousData,
} from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { useAuth } from "@auth/index";
import { SwapInfo, Result_1, SwapData } from "@canisters/gldt_swap/interfaces";
import { getSwapData } from "./utils/index";

interface GetUserHistoricSwapParams {
  page: number;
  principal: string;
  limit: number;
}

interface UseGetUserHistoricSwapParams
  extends Partial<GetUserHistoricSwapParams>,
    Omit<UseQueryOptions<[Result_1, number]>, "queryKey" | "queryFn"> {}

export const useGetUserHistoricSwap = ({
  page = 0,
  limit = 5,
  ...queryParams
}: UseGetUserHistoricSwapParams) => {
  const { isConnected, principalId, createActor } = useAuth();
  const [data, setData] = useState<{
    rows: SwapData[];
    pageCount: number;
    rowCount: number;
  } | null>(null);
  const [isInitializing, setIsInitializing] = useState(true);
  const [error, setError] = useState("");
  const [isError, setIsError] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);

  const get_historic_swaps_by_user = async ({
    page,
    principal,
    limit,
  }: GetUserHistoricSwapParams): Promise<[Result_1, number]> => {
    const actor = createActor("gldt_swap");
    const result_history = (await actor.get_historic_swaps_by_user({
      page: page,
      user: Principal.fromText(principal),
      limit: limit,
    })) as Result_1;
    const result_count = (await actor.get_history_total([
      Principal.fromText(principal),
    ])) as bigint;
    return [result_history, Number(result_count)];
  };

  const history = useQuery({
    queryKey: [
      "USER_FETCH_TRANSACTIONS_HISTORY",
      page,
      principalId,
      limit,
    ] as QueryKey,
    queryFn: () =>
      get_historic_swaps_by_user({
        page,
        principal: principalId as string,
        limit,
      }),
    enabled: !!isConnected && !!principalId && queryParams.enabled !== false,
    placeholderData: keepPreviousData,
    ...queryParams,
  });

  useEffect(() => {
    if (history.isLoading) {
      setIsSuccess(false);
      setIsError(false);
      setError("");
      setIsInitializing(true);
    } else if (history.isSuccess) {
      if ("Err" in history.data[0]) {
        console.log(history.data[0].Err);
        setIsError(true);
        setError("Error while fetching swap history :(.");
      } else {
        const rows = (
          history.data[0].Ok as Array<[[bigint, bigint], SwapInfo]>
        ).map((r) => getSwapData(r[1]));
        setData({
          rows,
          pageCount: Math.ceil(history.data[1] / limit),
          rowCount: history.data[1],
        });
        setIsSuccess(true);
      }
      setIsInitializing(false);
    } else if (history.isError) {
      console.log(history.error);
      setIsError(true);
      setError("Error while fetching swap history 2 :(.");
      setIsInitializing(false);
    }
  }, [
    history.data,
    history.error,
    history.isError,
    history.isLoading,
    history.isSuccess,
    limit,
  ]);

  return {
    isSuccess: isSuccess && !isInitializing,
    data,
    isError,
    error,
    isLoading: isInitializing,
  };
};
