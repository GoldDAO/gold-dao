import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent, ActorSubclass } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";
import { SwapAmountsResult, SwapAmountsReply } from "../interfaces";

const swap_amounts = async (
  actor: ActorSubclass,
  options: { from: string; to: string; amount: number }
) => {
  const { from, to, amount } = options;
  const result = (await actor.swap_amounts(
    from,
    BigInt(Math.round(amount * 1e8)),
    to
  )) as SwapAmountsResult;

  if ("Err" in result) throw new Error(result.Err);

  return result.Ok;
};

const useFetchTokenPrice = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<SwapAmountsReply>, "queryKey" | "queryFn"> & {
    from: string;
    to: string;
    amount: number;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    from,
    to,
    amount,
  } = options;

  return useQuery({
    queryKey: [`FETCH_${from}_${to}_PRICE`, from, to, amount],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const result = await swap_amounts(actor, {
          from,
          to,
          amount,
        });
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(
          `Fetch ${from} to ${to} price error! Please retry later.`
        );
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchTokenPrice;
