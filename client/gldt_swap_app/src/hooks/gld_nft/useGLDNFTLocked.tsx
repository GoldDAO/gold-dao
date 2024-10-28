import { useEffect, useState } from "react";
import { useQueries, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { SWAP_CANISTER_ID } from "@constants";

import { useAuth } from "@auth/index";

export const useGLDNFTLocked = () => {
  const { createActor } = useAuth();
  const [isInitializing, setIsInitializing] = useState(true);
  const [data, setData] = useState<number | undefined>(undefined);
  const [error, setError] = useState("");
  const [isError, setIsError] = useState(false);

  const getNFTByCanister = async (
    canisterName: string,
    nftValue: number
  ): Promise<number> => {
    const actor = createActor(canisterName);
    const result = (await actor.icrc7_balance_of([
      {
        owner: Principal.fromText(SWAP_CANISTER_ID as string),
        subaccount: [],
      },
    ])) as Array<bigint>;

    const data = Number(result[0]) * nftValue;

    return data;
  };

  const availableNFTs = useQueries({
    queries: [
      {
        queryKey: ["GET_AVAILABLE_GLD_NFT_1G"],
        queryFn: () => getNFTByCanister("gld_nft_1g", 1),
        placeholderData: keepPreviousData,
        enabled: true,
      },
      {
        queryKey: ["GET_AVAILABLE_GLD_NFT_10G"],
        queryFn: () => getNFTByCanister("gld_nft_10g", 10),
        placeholderData: keepPreviousData,
        enabled: true,
      },
      {
        queryKey: ["GET_AVAILABLE_GLD_NFT_100G"],
        queryFn: () => getNFTByCanister("gld_nft_100g", 100),
        enabled: true,
        placeholderData: keepPreviousData,
      },
      {
        queryKey: ["GET_AVAILABLE_GLD_NFT_1000G"],
        queryFn: () => getNFTByCanister("gld_nft_1000g", 1000),
        enabled: true,
        placeholderData: keepPreviousData,
      },
    ],
  });

  const isSuccess = availableNFTs.every((result) => result.isSuccess);
  const isLoading = availableNFTs.some((result) => result.isLoading);
  const isFetching = availableNFTs.some((result) => result.isFetching);
  const _isError = availableNFTs.some((result) => result.isError);
  const _error = availableNFTs.map((result) => result.error).filter(Boolean)[0];
  const _data = availableNFTs.map((result) => result.data);

  useEffect(() => {
    if (isLoading || isFetching) {
      setIsError(false);
      setIsInitializing(true);
    } else if (isSuccess && isInitializing && _data) {
      setData(
        _data.reduce(
          (accumulator: number, currentValue) =>
            accumulator + (currentValue ?? 0),
          0
        ) / 1000
      );
      setIsInitializing(false);
    } else if (_isError) {
      if (_error) console.log(_error);
      setIsError(true);
      setError("Error while fetching locked GLD NFTs.");
      setIsInitializing(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    _data,
    isSuccess,
    isLoading,
    isFetching,
    isError,
    _error,
    isInitializing,
  ]);

  return {
    data,
    isSuccess: isSuccess && !isInitializing,
    isError,
    isLoading: isInitializing,
    error,
  };
};
