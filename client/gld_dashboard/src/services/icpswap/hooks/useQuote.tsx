import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent, ActorSubclass } from "@dfinity/agent";

import { idlFactory as idlFactorySwapFactory } from "../idls/swap_factory";
import { Result } from "../interfaces/swap_factory";

const quote = async (
  actor: ActorSubclass,
  options: { amountIn: string; zeroForOne: boolean; amountOutMinimum: string }
): Promise<bigint> => {
  const { amountIn, zeroForOne, amountOutMinimum } = options;

  const result = (await actor.quote({
    amountIn,
    zeroForOne,
    amountOutMinimum,
  })) as Result;

  if ("err" in result) throw result.err;

  return result.ok;
};

const useQuote = (
  swapPoolCanisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<bigint>, "queryKey" | "queryFn"> & {
    amountIn: string;
    zeroForOne?: boolean;
    amountOutMinimum?: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    amountIn,
    zeroForOne = false,
    amountOutMinimum = "0",
  } = options;

  return useQuery({
    queryKey: [
      `FETCH_SWAP_FACTORY_QUOTE`,
      swapPoolCanisterId,
      amountIn,
      amountOutMinimum,
      zeroForOne,
    ],
    queryFn: async () => {
      try {
        const actorSwapFactory = Actor.createActor(idlFactorySwapFactory, {
          agent,
          canisterId: swapPoolCanisterId,
        });

        const result = await quote(actorSwapFactory, {
          amountIn,
          zeroForOne,
          amountOutMinimum,
        });
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`Fetch swap factory quote error! Please retry later.`);
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useQuote;
