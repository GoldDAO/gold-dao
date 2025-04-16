import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryKongswap } from "@services/kongswap/idlFactory";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import icrc1_fee from "@services/ledger/icrc1_fee";
import swap_amounts from "@services/kongswap/swap_amounts";

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
        const actorKongswap = Actor.createActor(idlFactoryKongswap, {
          agent,
          canisterId: KONGSWAP_CANISTER_ID_IC,
        });
        const feeToken = await icrc1_fee(actorTokenLedger);
        const decimalsToken = await icrc1_decimals(actorTokenLedger);

        const price = await swap_amounts(actorKongswap, {
          from: from,
          to: "ckUSDC",
          amount,
        });

        return {
          from: from,
          to: "ckUSDC",
          amount,
          decimals: decimalsToken,
          fee: feeToken,
          amount_usd: (amount_number / 10 ** decimalsToken) * price.mid_price,
        };
      } catch (err) {
        console.log(err);
        throw new Error(`Fetch token ${from} price error! Please retry later.`);
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchTokenPrice;
