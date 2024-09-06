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

const get_historic_swaps_by_user = async ({
  page,
  principal,
  limit,
}: GetUserHistoricSwapParams): Promise<[Result_1, number]> => {
  const { canisterId, idlFactory } = canisters["gldt_swap"];
  const actor = await getActor(canisterId, idlFactory, {
    isAnon: false,
  });
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

export const useGetUserHistoricSwap = ({
  page = 0,
  limit = 5,
  ...queryParams
}: UseGetUserHistoricSwapParams) => {
  const { isConnected, principalId } = useWallet();
  const [data, setData] = useState<{
    rows: SwapData[];
    pageCount: number;
    rowCount: number;
  } | null>(null);
  const [error, setError] = useState("");
  const [isError, setIsError] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

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
    if (history.isSuccess) {
      setIsError(false);
      setError("");
      setIsLoading(true);
      setIsSuccess(false);
      const initData = async () => {
        await new Promise<void>((resolve) => {
          if ("Err" in history.data[0]) {
            // setError(Object.keys(historic.data.Err)[0]);
            setIsError(true);
            setError("Error while fetching history swap.");
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
          resolve();
          setIsLoading(false);
        });
      };
      initData();
    }
  }, [history.data, history.isSuccess, limit]);

  return {
    isSuccess,
    data,
    isError: history.isError || isError,
    error: history.error || error,
    isLoading: history.isLoading || isLoading,
  };
};
