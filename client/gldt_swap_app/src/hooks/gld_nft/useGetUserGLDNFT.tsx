import { useEffect, useState } from "react";
import { useQueries, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { useWallet, getActor } from "@amerej/artemis-react";

import { TokenId, Nft, useNft } from "@context/index";
import { canisters } from "@providers/Auth";

export const useGetUserGLDNFT = () => {
  const { principalId, isConnected } = useWallet();
  const { setNfts } = useNft();
  const [isLoadingInit, setIsLoadingInit] = useState(true);

  const getUserNFTByCanister = async (canisterName: string): Promise<Nft> => {
    const { canisterId, idlFactory } = canisters[canisterName];
    const actor = await getActor(canisterId, idlFactory, {
      isAnon: false,
    });
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
        };
      })
    );

    return {
      tokenIds,
    };
  };

  const userNFTs = useQueries({
    queries: [
      {
        queryKey: ["GET_USER_GLD_NFT_1G"],
        queryFn: () => getUserNFTByCanister("gld_nft_1g"),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!principalId,
        refetchOnWindowFocus: false,
      },
      {
        queryKey: ["GET_USER_GLD_NFT_10G"],
        queryFn: () => getUserNFTByCanister("gld_nft_10g"),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!principalId,
        refetchOnWindowFocus: false,
      },
      // {
      //   queryKey: ["GET_USER_GLD_NFT_100G"],
      //   queryFn: () => getUserNFTByCanister("gld_nft_100g"),
      //   placeholderData: keepPreviousData,
      //   enabled: !!isConnected && !!principalId,
      // },
      // {
      //   queryKey: ["GET_USER_GLD_NFT_1000G"],
      //   queryFn: () => getUserNFTByCanister("gld_nft_1000g"),
      //   placeholderData: keepPreviousData,
      //   enabled: !!isConnected && !!principalId,
      // },
    ],
  });

  const isSuccess = userNFTs.every((result) => result.isSuccess);
  const isLoading = userNFTs.some((result) => result.isLoading);
  const isFetching = userNFTs.some((result) => result.isFetching);
  const isError = userNFTs.some((result) => result.isError);
  const error = userNFTs.map((result) => result.error).filter(Boolean)[0];
  const data = userNFTs.map((result) => result.data);

  useEffect(() => {
    if (isLoading || isFetching) {
      setIsLoadingInit(true);
    }
  }, [isLoading, isFetching]);

  useEffect(() => {
    const updateNfts = async () => {
      if (isSuccess && data && isLoadingInit) {
        await new Promise<void>((resolve) => {
          setNfts(data as Nft[]);
          resolve();
        });
        setIsLoadingInit(false);
      }
    };

    updateNfts();
  }, [data, isSuccess, isLoadingInit, setNfts]);

  return {
    data,
    isSuccess,
    isError,
    isLoading: isLoadingInit,
    error,
  };
};
