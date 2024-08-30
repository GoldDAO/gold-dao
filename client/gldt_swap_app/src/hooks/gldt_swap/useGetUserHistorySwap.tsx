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

import { useGetUserHistoricCountSwap } from "@hooks/gldt_swap";

interface GetUserHistoricSwapParams {
  page: number;
  principal: string;
  limit: number;
}

interface UseGetUserHistoricSwapParams
  extends Partial<GetUserHistoricSwapParams>,
    Omit<UseQueryOptions<Result_1>, "queryKey" | "queryFn"> {}

const get_historic_swaps_by_user = async ({
  page,
  principal,
  limit,
}: GetUserHistoricSwapParams) => {
  const { canisterId, idlFactory } = canisters["gldt_swap"];
  const actor = await getActor(canisterId, idlFactory, {
    isAnon: false,
  });
  const result = (await actor.get_historic_swaps_by_user({
    page: page,
    user: Principal.fromText(principal),
    limit: limit,
  })) as Result_1;
  return result;
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
  const [isInitializing, setIsInitializing] = useState(true);

  const historic = useQuery({
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

  const historic_count = useGetUserHistoricCountSwap({
    enabled: !!historic.isSuccess,
  });

  useEffect(() => {
    if (historic.isLoading || historic_count.isLoading) {
      setIsInitializing(true);
      setIsError(false);
    }
  }, [historic.isLoading, historic_count.isLoading]);

  useEffect(() => {
    if (historic.isError || historic_count.isError) {
      setIsInitializing(false);
    }
  }, [historic.isError, historic_count.isError]);

  useEffect(() => {
    if (
      historic.isSuccess &&
      historic.data &&
      historic_count.isSuccess &&
      historic_count.data
    ) {
      if ("Err" in historic.data) {
        setError(Object.keys(historic.data.Err)[0]);
        setIsInitializing(false);
        setIsError(true);
        setError("Error while fetching history swap.");
      }
      if ("Ok" in historic.data) {
        const rows = (
          historic.data.Ok as Array<[[bigint, bigint], SwapInfo]>
        ).map((r) => getSwapData(r[1]));
        setData({
          rows,
          pageCount: Math.ceil(historic_count.data / limit),
          rowCount: historic_count.data,
        });
        setIsInitializing(false);
      }
    }
  }, [
    historic.isSuccess,
    historic.data,
    historic_count.isSuccess,
    historic_count.data,
    limit,
  ]);

  return {
    isSuccess:
      historic.isSuccess && historic_count.isSuccess && !isInitializing,
    data,
    isError: historic.isError || isError,
    error: historic.error || error,
    isLoading: isInitializing,
  };
};
