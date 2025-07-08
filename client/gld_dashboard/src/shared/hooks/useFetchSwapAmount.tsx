import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryKongSwap } from "@services/kongswap/idlFactory";
import { SwapAmountsReply } from "@services/kongswap//interfaces";
import swap_amounts from "@services/kongswap//swap_amounts";
import icrc1_decimals from "@services/ledger/icrc1_decimals";

const useFetchSwapAmount = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<SwapAmountsReply>, "queryKey" | "queryFn"> & {
    from: string;
    from_canister_id: string;
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
    from_canister_id,
  } = options;

  return useQuery({
    queryKey: [`FETCH_SWAP_AMOUNT`, from, to, amount],
    queryFn: async () => {
      try {
        const actorLedger = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: from_canister_id,
        });

        const actorKongSwap = Actor.createActor(idlFactoryKongSwap, {
          agent,
          canisterId,
        });

        const decimals = await icrc1_decimals(actorLedger);

        const result = await swap_amounts(actorKongSwap, {
          from,
          to,
          amount: BigInt(Math.round(amount * 10 ** decimals)),
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
