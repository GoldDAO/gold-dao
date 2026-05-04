import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { ICPSWAP_CANISTER_ID } from "@constants";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryIcpswap } from "@services/icpswap/idls/swap_pool";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import icrc1_fee from "@services/ledger/icrc1_fee";
import get_token_price_usd from "@services/icpswap/get_token_price_usd";

const useFetchTokenPrice = (
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<{
      from: string;
      to: string;
      amount: bigint;
      decimals: number;
      fee: bigint;
      amount_usd: number;
    }>,
    "queryKey" | "queryFn"
  > & {
    from: string;
    from_canister_id: string;
    amount: bigint;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    refetchInterval = false,
    from,
    from_canister_id,
    amount = 0n,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    ...queryOptions
  } = options;

  const from_token = from.toLocaleUpperCase();
  const amount_number = Number(amount);

  return useQuery({
    queryKey: [
      `FETCH_TOKEN_PRICE_${from_token}`,
      from,
      from_canister_id,
      amount_number,
    ],
    queryFn: async () => {
      try {
        const actorTokenLedger = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: from_canister_id,
        });
        const actorIcpswap = Actor.createActor(idlFactoryIcpswap, {
          agent,
          canisterId: ICPSWAP_CANISTER_ID,
        });
        const fee = await icrc1_fee(actorTokenLedger);
        const decimals = await icrc1_decimals(actorTokenLedger);

        const priceUSD = await get_token_price_usd(
          actorIcpswap,
          from_canister_id,
          from,
          { agent }
        );

        return {
          from: from,
          to: "ckUSDC",
          amount,
          decimals,
          fee,
          amount_usd: (amount_number / 10 ** decimals) * priceUSD,
        };
      } catch (err) {
        console.log(err);
        throw new Error(`Fetch token ${from} price error! Please retry later.`);
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
    staleTime,
    refetchOnMount,
    refetchOnWindowFocus,
    refetchOnReconnect,
    ...queryOptions,
  });
};

export default useFetchTokenPrice;
