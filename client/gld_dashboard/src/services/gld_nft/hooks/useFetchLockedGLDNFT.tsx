import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

import { roundAndFormatLocale } from "@shared/utils/numbers";

const useFetchLockedGLDNFT = (
  canisterId1G: string,
  canisterId10G: string,
  canisterId100G: string,
  canisterId1K: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<string>, "queryKey" | "queryFn"> & {
    owner: string;
  }
) => {
  const { owner, enabled = true, placeholderData = keepPreviousData } = options;

  const nfts = [
    { canister: canisterId1G, value: 1 },
    { canister: canisterId10G, value: 10 },
    { canister: canisterId100G, value: 100 },
    {
      canister: canisterId1K,
      value: 1000,
    },
  ];

  return useQuery({
    queryKey: ["FETCH_AVAILABLE_NFTS"],
    queryFn: async () => {
      const results = await Promise.allSettled(
        nfts.map(async ({ canister, value }) => {
          const actor = Actor.createActor(idlFactory, {
            agent,
            canisterId: canister,
          });

          const result = (await actor.icrc7_balance_of([
            {
              owner: Principal.fromText(owner),
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

      const data =
        fulfilledResults.reduce(
          (accumulator: number, currentValue) =>
            accumulator + (currentValue ?? 0),
          0
        ) / 1000;
      return roundAndFormatLocale({ number: data });
    },
    enabled,
    placeholderData,
  });
};

export default useFetchLockedGLDNFT;
