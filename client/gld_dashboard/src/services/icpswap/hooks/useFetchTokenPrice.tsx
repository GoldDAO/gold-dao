import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent, ActorSubclass } from "@dfinity/agent";

import { idlFactory as idlFactorySwapPool } from "../idls/swap_pool";

import { GetPoolsForTokenResponse } from "../interfaces/swap_pool";

const get_pools_for_token = async (
  actor: ActorSubclass,
  canisterId: string,
  options: { tokenSymbol0: string; tokenSymbol1?: string }
) => {
  const results = (await actor.getPoolsForToken(
    canisterId
  )) as GetPoolsForTokenResponse[];
  const { tokenSymbol0, tokenSymbol1 = "ICP" } = options;

  const result1 = results?.find(
    (res) =>
      res?.token0Symbol === tokenSymbol0 && res?.token1Symbol === tokenSymbol1
  );
  const result2 = results?.find(
    (res) =>
      res?.token1Symbol === tokenSymbol0 && res?.token0Symbol === tokenSymbol1
  );

  const result = result1?.token0Price ?? result2?.token1Price;
  if (!result)
    throw new Error(
      "Fetch getPoolsForToken error! No results from icpswap pools for tokens."
    );
  return result;
};

const useFetchTokenPrice = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<number>, "queryKey" | "queryFn"> & {
    tokenSymbol: string;
    tokenCanisterId: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    tokenSymbol,
    tokenCanisterId,
  } = options;

  return useQuery({
    queryKey: [`FETCH_${tokenSymbol}_PRICE`],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactorySwapPool, {
          agent,
          canisterId,
        });

        const result = await get_pools_for_token(actor, tokenCanisterId, {
          tokenSymbol0: tokenSymbol,
        });
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(
          `Fetch ${tokenSymbol} price error! Please retry later.`
        );
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchTokenPrice;
