import { useEffect, useRef, useState } from "react";
import { useQueries, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { useWallet, getActor } from "@amerej/artemis-react";
import _isEqual from "lodash/isEqual";

import { TokenId, Nft, useNft } from "@context/index";
import { canisters } from "@providers/Auth";
import { SWAP_CANISTER_ID } from "@constants";
import { bigintTo32ByteArray } from "@utils/index";

const usePrevious = (value: Nft[]) => {
  const ref = useRef<Nft[]>();
  useEffect(() => {
    ref.current = value;
  }, [value]);
  return ref.current;
};

export const useGetAvailableGLDNFT = () => {
  const { principalId, isConnected } = useWallet();
  const { setNfts } = useNft();
  const [isLoadingInit, setIsLoadingInit] = useState(true);

  const getUserNFTByCanister = async (canisterName: string): Promise<Nft> => {
    const { canisterId, idlFactory } = canisters[canisterName];
    const actor = await getActor(canisterId, idlFactory, {
      isAnon: false,
    });
    const token_ids_bigint = (await actor.icrc7_tokens_of(
      {
        owner: Principal.fromText(SWAP_CANISTER_ID as string),
        subaccount: [],
      },
      [],
      []
    )) as Array<bigint>;

    // console.log(token_ids_bigint);

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

    // console.log(tokenIds);
    return {
      tokenIds,
    };
  };

  const availableNFTs = useQueries({
    queries: [
      {
        queryKey: ["GET_AVAILABLE_GLD_NFT_1G"],
        queryFn: () => getUserNFTByCanister("gld_nft_1g"),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!principalId,
        refetchOnWindowFocus: false,
      },
      {
        queryKey: ["GET_AVAILABLE_GLD_NFT_10G"],
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

  const isSuccess = availableNFTs.every((result) => result.isSuccess);
  const isLoading = availableNFTs.some((result) => result.isLoading);
  const isFetching = availableNFTs.some((result) => result.isFetching);
  const isError = availableNFTs.some((result) => result.isError);
  const error = availableNFTs.map((result) => result.error).filter(Boolean)[0];
  const data = availableNFTs.map((result) => result.data);
  const prevData = usePrevious(data as Nft[]);
  // const refetchAll = useCallback(() => {
  //   availableNFTs.forEach((result) => result.refetch());
  // }, [availableNFTs]);

  useEffect(() => {
    if (isLoading || isFetching) {
      setIsLoadingInit(true);
    }
  }, [isLoading, isFetching]);

  useEffect(() => {
    if (isSuccess && data && !_isEqual(data, prevData) && isLoadingInit) {
      setNfts(data as Nft[]);
      setIsLoadingInit(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [data, isSuccess, isConnected, prevData, isLoadingInit]);

  return {
    data,
    isSuccess,
    isError,
    isLoading: isLoading || isFetching || isLoadingInit,
    error,
  };
};
