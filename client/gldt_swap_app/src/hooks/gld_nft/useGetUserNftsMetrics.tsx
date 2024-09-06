import { useEffect, useState } from "react";
import { useQueries, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { useWallet, getActor } from "@amerej/artemis-react";

import { GLDT_VALUE_1G_NFT } from "@constants";

import { canisters } from "@providers/Auth";

interface NFTMetric {
  countNFT: number;
  valueNFT: number;
  countGLDT: number;
  countWeight: number;
}

interface NFTMetrics {
  nfts: NFTMetric[];
  totalCountNFT: number;
  totalCountGLDT: number;
  totalCountWeight: number;
}

export const useGetUserNftsMetrics = () => {
  const { principalId, isConnected } = useWallet();
  const [data, setData] = useState<NFTMetrics | null>(null);
  const [isInitializing, setIsInitializing] = useState(true);

  const getUserNFTCountByCanister = async (
    canisterName: string,
    valueNFT: number
  ): Promise<NFTMetric> => {
    const { canisterId, idlFactory } = canisters[canisterName];
    const actor = await getActor(canisterId, idlFactory, {
      isAnon: false,
    });
    const result = (await actor.count_unlisted_tokens_of({
      owner: Principal.fromText(principalId as string),
      subaccount: [],
    })) as bigint;

    const countNFT = Number(result);
    return {
      countNFT,
      valueNFT,
      countGLDT: countNFT * (valueNFT * GLDT_VALUE_1G_NFT),
      countWeight: countNFT * valueNFT,
    };
  };

  const userNFTs = useQueries({
    queries: [
      {
        queryKey: ["GET_USER_GLD_NFT_1G_COUNT"],
        queryFn: () => getUserNFTCountByCanister("gld_nft_1g", 1),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!principalId,
        refetchOnWindowFocus: false,
      },
      {
        queryKey: ["GET_USER_GLD_NFT_10G_COUNT"],
        queryFn: () => getUserNFTCountByCanister("gld_nft_10g", 10),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!principalId,
        refetchOnWindowFocus: false,
      },
      //   {
      //     queryKey: ["GET_USER_GLD_NFT_100G_COUNT"],
      //     queryFn: () => getUserNFTCountByCanister("gld_nft_100g", 100),
      //     placeholderData: keepPreviousData,
      //     enabled: !!isConnected && !!principalId,
      //   },
      //   {
      //     queryKey: ["GET_USER_GLD_NFT_1000G_COUNT"],
      //     queryFn: () => getUserNFTCountByCanister("gld_nft_1000g", 1000),
      //     placeholderData: keepPreviousData,
      //     enabled: !!isConnected && !!principalId,
      //   },
    ],
  });

  const isSuccess = userNFTs.every((result) => result.isSuccess);
  const isLoading = userNFTs.some((result) => result.isLoading);
  const isFetching = userNFTs.some((result) => result.isFetching);
  const isError = userNFTs.some((result) => result.isError);
  const error = userNFTs.map((result) => result.error).filter(Boolean)[0];
  const _data = userNFTs.map((result) => result.data);

  useEffect(() => {
    if (isLoading || isFetching) {
      setIsInitializing(true);
    }
  }, [isLoading, isFetching]);

  useEffect(() => {
    if (isSuccess && _data) {
      const initData = async () => {
        if (isSuccess && _data && isInitializing) {
          await new Promise<void>((resolve) => {
            setData({
              nfts: _data as NFTMetric[],
              totalCountNFT: (_data as NFTMetric[]).reduce(
                (accumulator, currentValue) =>
                  accumulator + currentValue.countNFT,
                0
              ),
              totalCountGLDT: 0, // todo: ask freddie for route
              totalCountWeight: (_data as NFTMetric[]).reduce(
                (accumulator, currentValue) =>
                  accumulator + currentValue?.countWeight,
                0
              ),
            });
            resolve();
          });
          setIsInitializing(false);
        }
      };
      initData();
      setIsInitializing(false);
    }
  }, [_data, isInitializing, isSuccess]);

  return {
    data,
    isSuccess: isSuccess && !isInitializing,
    isError,
    isLoading: isInitializing,
    error,
  };
};
