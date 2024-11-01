import { useEffect, useState } from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { SWAP_CANISTER_ID } from "@constants";

import { useAuth } from "@auth/index";
import { TokenId, Nft, useNft, CollectionIndex } from "@context/index";

import { bigintTo32ByteArray } from "@utils/index";

import { useGetActiveSwaps } from "@hooks/gldt_swap";

export const useGetAvailableGLDNFT = () => {
  const { isConnected, createActor } = useAuth();
  const { setNfts } = useNft();
  const [unavailableNfts, setUnavailableNfts] = useState<string[] | undefined>(
    undefined
  );
  const active_swaps = useGetActiveSwaps();

  const nfts = [
    { canister: "gld_nft_1g", collectionIndex: CollectionIndex.GLD_NFT_1G },
    { canister: "gld_nft_10g", collectionIndex: CollectionIndex.GLD_NFT_10G },
    { canister: "gld_nft_100g", collectionIndex: CollectionIndex.GLD_NFT_100G },
    {
      canister: "gld_nft_1000g",
      collectionIndex: CollectionIndex.GLD_NFT_1000G,
    },
  ];

  // ? nft's currently being swapped
  useEffect(() => {
    if (active_swaps.isSuccess && active_swaps.data) {
      setUnavailableNfts(
        active_swaps.data.rows.map((row) => row.nft_id_string)
      );
    }
  }, [active_swaps.isSuccess, active_swaps.data]);

  const availableNFTs = useQuery({
    queryKey: ["FETCH_AVAILABLE_NFTS"],
    queryFn: async (): Promise<Nft[]> => {
      const results = await Promise.allSettled(
        nfts.map(async ({ canister, collectionIndex }) => {
          const actor = createActor(canister);
          const token_ids_bigint = (await actor.icrc7_tokens_of(
            {
              owner: Principal.fromText(SWAP_CANISTER_ID as string),
              subaccount: [],
            },
            [],
            []
          )) as Array<bigint>;

          const allTokenIds = await Promise.all(
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

          // ? filter nft's currently being swapped
          const tokenIds =
            allTokenIds.filter(
              (token) => !unavailableNfts?.includes(token.id_string)
            ) ?? [];

          return {
            tokenIds,
            collectionIndex,
          };
        })
      );

      const rejectedResults = results.filter(
        (result): result is PromiseRejectedResult =>
          result.status === "rejected"
      );
      if (rejectedResults.length > 0) {
        console.error(
          "Some requests to GLD NFTs canisters failed:",
          rejectedResults.map((r) => r.reason)
        );
        throw new Error("Error while fetching available GLD NFTs.");
      }

      return results
        .filter(
          (
            result
          ): result is PromiseFulfilledResult<{
            tokenIds: TokenId[];
            collectionIndex: CollectionIndex;
          }> => result.status === "fulfilled"
        )
        .map((result) => result.value);
    },
    enabled: !!isConnected && !!active_swaps.isSuccess,
    refetchOnWindowFocus: false,
    placeholderData: keepPreviousData,
  });

  useEffect(() => {
    if (availableNFTs.isSuccess) setNfts(availableNFTs.data);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [availableNFTs.isSuccess, availableNFTs.data]);

  return {
    isSuccess: availableNFTs.isSuccess,
    isLoading:
      availableNFTs.isLoading ||
      availableNFTs.isFetching ||
      availableNFTs.isPending,
    isError: availableNFTs.isError,
    error: availableNFTs.error?.message ?? "",
  };
};
