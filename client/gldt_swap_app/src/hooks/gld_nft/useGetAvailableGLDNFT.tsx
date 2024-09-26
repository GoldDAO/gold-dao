import { useEffect, useState } from "react";
import { useQueries, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { SWAP_CANISTER_ID } from "@constants";

import { useAuth } from "@context/auth";
import { TokenId, Nft, useNft } from "@context/index";

import { bigintTo32ByteArray } from "@utils/index";

import { useGetActiveSwaps } from "@hooks/gldt_swap";

export const useGetAvailableGLDNFT = () => {
  const { state: authState, getActor } = useAuth();
  const { isConnected } = authState;
  const { setNfts } = useNft();
  const [isInitializing, setIsInitializing] = useState(true);
  const [error, setError] = useState("");
  const [isError, setIsError] = useState(false);

  const active_swaps = useGetActiveSwaps();

  const getNFTByCanister = async (canisterName: string): Promise<Nft> => {
    const actor = getActor(canisterName);
    const token_ids_bigint = (await actor.icrc7_tokens_of(
      {
        owner: Principal.fromText(SWAP_CANISTER_ID as string),
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
    };
  };

  const availableNFTs = useQueries({
    queries: [
      {
        queryKey: ["GET_AVAILABLE_GLD_NFT_1G"],
        queryFn: () => getNFTByCanister("gld_nft_1g"),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!active_swaps.isSuccess,
        refetchOnWindowFocus: false,
      },
      // {
      //   queryKey: ["GET_AVAILABLE_GLD_NFT_10G"],
      //   queryFn: () => getNFTByCanister("gld_nft_10g"),
      //   placeholderData: keepPreviousData,
      //   enabled: !!isConnected,
      //   refetchOnWindowFocus: false,
      // },
      // {
      //   queryKey: ["GET_USER_GLD_NFT_100G"],
      //   queryFn: () => getNFTByCanister("gld_nft_100g"),
      //   placeholderData: keepPreviousData,
      //   enabled: !!isConnected,
      // },
      // {
      //   queryKey: ["GET_USER_GLD_NFT_1000G"],
      //   queryFn: () => getNFTByCanister("gld_nft_1000g"),
      //   placeholderData: keepPreviousData,
      //   enabled: !!isConnected,
      // },
    ],
  });

  const isSuccess = availableNFTs.every((result) => result.isSuccess);
  const isLoading = availableNFTs.some((result) => result.isLoading);
  const isFetching = availableNFTs.some((result) => result.isFetching);
  const _isError = availableNFTs.some((result) => result.isError);
  const _error = availableNFTs.map((result) => result.error).filter(Boolean)[0];
  const data = availableNFTs.map((result) => result.data);

  useEffect(() => {
    if (isLoading || isFetching) {
      setIsError(false);
      setIsInitializing(true);
    } else if (isSuccess && isInitializing) {
      // ? Filter nft's currently being swapped
      const nftIdStrings =
        active_swaps.data?.rows.map((row) => row.nft_id_string) ?? [];
      const filteredData = data.map((obj) => {
        return {
          ...obj,
          tokenIds:
            obj?.tokenIds.filter(
              (token) => !nftIdStrings.includes(token.id_string)
            ) ?? [],
        };
      });
      const updateNfts = async () => {
        await new Promise<void>((resolve) => {
          setNfts(filteredData as Nft[]);
          resolve();
        });
      };
      updateNfts();
      setIsInitializing(false);
    } else if (_isError || active_swaps.isError) {
      console.log(_error ?? active_swaps.error);
      setIsError(true);
      setError("Error while fetching available NFTs :(.");
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
