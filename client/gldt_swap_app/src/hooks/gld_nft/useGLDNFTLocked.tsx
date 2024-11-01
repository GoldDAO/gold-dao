import { useQuery, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { SWAP_CANISTER_ID } from "@constants";

import { useAuth } from "@auth/index";

export const useGLDNFTLocked = () => {
  const { createActor } = useAuth();

  const nfts = [
    { canister: "gld_nft_1g", value: 1 },
    { canister: "gld_nft_10g", value: 10 },
    { canister: "gld_nft_100g", value: 100 },
    {
      canister: "gld_nft_1000g",
      value: 1000,
    },
  ];

  const test = useQuery({
    queryKey: ["FETCH_AVAILABLE_NFTS"],
    queryFn: async (): Promise<number> => {
      const results = await Promise.allSettled(
        nfts.map(async ({ canister, value }) => {
          const actor = createActor(canister);
          const result = (await actor.icrc7_balance_of([
            {
              owner: Principal.fromText(SWAP_CANISTER_ID as string),
              subaccount: [],
            },
          ])) as Array<bigint>;

          const data = Number(result[0]) * value;

          return data;
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
        throw new Error("Error while fetching GLD NFTs total locked.");
      }

      const fulfilledResults = results
        .filter(
          (result): result is PromiseFulfilledResult<number> =>
            result.status === "fulfilled"
        )
        .map((result) => result.value);

      return (
        fulfilledResults.reduce(
          (accumulator: number, currentValue) =>
            accumulator + (currentValue ?? 0),
          0
        ) / 1000
      );
    },
    enabled: true,
    placeholderData: keepPreviousData,
  });

  return {
    data: test.data,
    isSuccess: test.isSuccess,
    isLoading: test.isLoading || test.isPending,
    isError: test.isError,
    error: test.error?.message ?? "",
  };
};
