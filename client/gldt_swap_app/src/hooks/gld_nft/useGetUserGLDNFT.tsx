import { useEffect, useState } from "react";
import { useQueries, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { useAuth } from "@auth/index";
import { TokenId, Nft, useNft, CollectionIndex } from "@context/index";

import { bigintTo32ByteArray } from "@utils/index";

import { useGetUserActiveSwaps } from "@hooks/gldt_swap";

export const useGetUserGLDNFT = () => {
  const { isConnected, principalId, createActor } = useAuth();
  const { setNfts } = useNft();
  const [isInitializing, setIsInitializing] = useState(true);
  const [error, setError] = useState("");
  const [isError, setIsError] = useState(false);

  const active_swaps = useGetUserActiveSwaps();

  const getUserNFTByCanister = async (
    canisterName: string,
    collectionIndex: CollectionIndex
  ): Promise<Nft> => {
    const actor = createActor(canisterName);
    const token_ids_bigint = (await actor.unlisted_tokens_of(
      {
        owner: Principal.fromText(principalId as string),
        subaccount: [],
      },
      [],
      []
    )) as Array<bigint>;

    const tokenIds = await Promise.all(
      token_ids_bigint.map(async (tokenId: bigint): Promise<TokenId> => {
        const result = (await actor.get_nat_as_token_id_origyn(
          tokenId
        )) as string;
        return {
          id_string: result,
          id_bigint: tokenId,
          id_byte_array: bigintTo32ByteArray(tokenId),
        };
      })
    );

    return {
      tokenIds,
      collectionIndex,
    };
  };

  const userNFTs = useQueries({
    queries: [
      {
        queryKey: ["USER_GET_GLD_NFT_1G"],
        queryFn: () =>
          getUserNFTByCanister("gld_nft_1g", CollectionIndex.GLD_NFT_1G),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!active_swaps.isSuccess,
        refetchOnWindowFocus: false,
      },
      {
        queryKey: ["USER_GET_GLD_NFT_10G"],
        queryFn: () =>
          getUserNFTByCanister("gld_nft_10g", CollectionIndex.GLD_NFT_10G),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!active_swaps.isSuccess,
        refetchOnWindowFocus: false,
      },
      // {
      //   queryKey: ["USER_GET_GLD_NFT_100G"],
      //   queryFn: () =>
      //     getUserNFTByCanister("gld_nft_100g", CollectionIndex.GLD_NFT_100G),
      //   placeholderData: keepPreviousData,
      //   enabled: !!isConnected && !!active_swaps.isSuccess,
      //   refetchOnWindowFocus: false,
      // },
      // {
      //   queryKey: ["USER_GET_GLD_NFT_1000G"],
      //   queryFn: () => getUserNFTByCanister("gld_nft_1000g", CollectionIndex.GLD_NFT_1000G),
      //   placeholderData: keepPreviousData,
      //   enabled: !!isConnected && !!principalId,
      // },
    ],
  });

  const isSuccess = userNFTs.every((result) => result.isSuccess);
  const isLoading = userNFTs.some((result) => result.isLoading);
  const isFetching = userNFTs.some((result) => result.isFetching);
  const _isError = userNFTs.some((result) => result.isError);
  const _error = userNFTs.map((result) => result.error).filter(Boolean)[0];
  const data = userNFTs.map((result) => result.data);

  useEffect(() => {
    if (isLoading || isFetching) {
      setIsInitializing(true);
      setIsError(false);
    } else if (isSuccess && active_swaps.isSuccess && isInitializing) {
      // ? Filter nft's currently being swapped
      const unavailableNfts =
        active_swaps.data?.rows.map((row) => row.nft_id_string) ?? [];
      const availableNfts = data.map((obj) => {
        return {
          ...obj,
          tokenIds:
            obj?.tokenIds.filter(
              (token) => !unavailableNfts.includes(token.id_string)
            ) ?? [],
        };
      });
      (async function () {
        await setNfts(availableNfts as Nft[]);
      })();
      setIsInitializing(false);
    } else if (_isError || active_swaps.isError) {
      if (_error) console.log(_error);
      setIsError(true);
      setError("Error while fetching your NFTs :(.");
      setIsInitializing(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    data,
    isSuccess,
    isLoading,
    isFetching,
    isError,
    _error,
    isInitializing,
    active_swaps.isError,
  ]);

  return {
    data,
    isSuccess: isSuccess && !isInitializing,
    isError,
    isLoading: isInitializing,
    error,
  };
};
