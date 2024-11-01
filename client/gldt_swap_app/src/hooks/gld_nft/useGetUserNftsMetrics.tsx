import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { GLDT_VALUE_1G_NFT } from "@constants";

import { useAuth } from "@auth/index";

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
  const { isConnected, principalId, createActor } = useAuth();

  const nfts = [
    { canister: "gld_nft_1g", value: 1 },
    { canister: "gld_nft_10g", value: 10 },
    { canister: "gld_nft_100g", value: 100 },
    {
      canister: "gld_nft_1000g",
      value: 1000,
    },
  ];

  const userNFTs = useQuery({
    queryKey: ["USER_FETCH_NFTS_METRICS", principalId],
    queryFn: async (): Promise<NFTMetrics> => {
      const results = await Promise.allSettled(
        nfts.map(async ({ canister, value }) => {
          const actor = createActor(canister);
          const result = (await actor.count_unlisted_tokens_of({
            owner: Principal.fromText(principalId as string),
            subaccount: [],
          })) as bigint;

          const countNFT = Number(result);
          return {
            countNFT,
            valueNFT: value,
            countGLDT: countNFT * (value * GLDT_VALUE_1G_NFT),
            countWeight: countNFT * value,
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
        throw new Error("Error while fetching your GLD NFTs metrics.");
      }

      const fulfilledResults = results
        .filter(
          (result): result is PromiseFulfilledResult<NFTMetric> =>
            result.status === "fulfilled"
        )
        .map((result) => result.value);

      return {
        nfts: fulfilledResults as NFTMetric[],
        totalCountNFT: (fulfilledResults as NFTMetric[]).reduce(
          (accumulator, currentValue) => accumulator + currentValue.countNFT,
          0
        ),
        totalCountGLDT: 0, // todo: ask freddie for route
        totalCountWeight: (fulfilledResults as NFTMetric[]).reduce(
          (accumulator, currentValue) =>
            accumulator + currentValue?.countWeight,
          0
        ),
      };
    },
    enabled: !!isConnected,
    placeholderData: keepPreviousData,
    refetchOnWindowFocus: false,
  });

  return {
    data: userNFTs.data,
    isSuccess: userNFTs.isSuccess,
    isLoading: userNFTs.isLoading || userNFTs.isPending,
    isError: userNFTs.isError,
    error: userNFTs.error?.message ?? "",
  };
};
