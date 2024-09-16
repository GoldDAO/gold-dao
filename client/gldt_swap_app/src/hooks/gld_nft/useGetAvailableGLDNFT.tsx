import { useEffect, useState } from "react";
import { useQueries, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { useWallet, getActor } from "@amerej/artemis-react";

import { TokenId, Nft, useNft } from "@context/index";
import { canisters } from "@providers/Auth";
import { SWAP_CANISTER_ID } from "@constants";
import { bigintTo32ByteArray } from "@utils/index";

export const useGetAvailableGLDNFT = () => {
  const { principalId, isConnected } = useWallet();
  const { setNfts } = useNft();
  const [isInitializing, setIsInitializing] = useState(true);
  const [error, setError] = useState("");

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
        queryFn: () => getUserNFTByCanister("gld_nft_1g"),
        placeholderData: keepPreviousData,
        enabled: !!isConnected && !!principalId,
        refetchOnWindowFocus: false,
      },
      // {
      //   queryKey: ["GET_AVAILABLE_GLD_NFT_10G"],
      //   queryFn: () => getUserNFTByCanister("gld_nft_10g"),
      //   placeholderData: keepPreviousData,
      //   enabled: !!isConnected && !!principalId,
      //   refetchOnWindowFocus: false,
      // },
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
  const _error = availableNFTs.map((result) => result.error).filter(Boolean)[0];
  const data = availableNFTs.map((result) => result.data);

  useEffect(() => {
    if (isLoading || isFetching) {
      setIsInitializing(true);
    } else if (isSuccess && isInitializing) {
      const updateNfts = async () => {
        await new Promise<void>((resolve) => {
          setNfts(data as Nft[]);
          resolve();
        });
      };
      updateNfts();
      setIsInitializing(false);
    } else if (isError) {
      console.log(_error);
      setError("Error while fetching available NFTs :(.");
      setIsInitializing(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [data, isSuccess, isLoading, isFetching, isError, _error, isInitializing]);

  return {
    data,
    isSuccess: isSuccess && !isInitializing,
    isError,
    isLoading: isInitializing,
    error,
  };
};
