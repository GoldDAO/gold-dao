import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";
import { SwapAmountsReply } from "../interfaces";
import swap_amounts from "../swap_amounts";

const useFetchSwapAmount = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<SwapAmountsReply>, "queryKey" | "queryFn"> & {
    from: string;
    to: string;
    amount: bigint;
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
    queryKey: [`FETCH_${from}_${to}_PRICE`, from, to, Number(amount)],
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

export default useFetchSwapAmount;
