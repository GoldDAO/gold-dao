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
import { Ledger } from "@services/ledger/utils/interfaces";

const useFetchTokenData = (
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<{
      decimals: number;
      fee: number;
      fee_usd: number;
      fee_e8s: bigint;
      price_usd: number;
    }>,
    "queryKey" | "queryFn"
  > & {
    token: Ledger;
    token_canister_id: string;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    refetchInterval = false,
    token,
    token_canister_id,
  } = options;

  return useQuery({
    queryKey: ["FETCH_TOKEN_DATA", token, token_canister_id],
    queryFn: async () => {
      try {
        const actorTokenLedger = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: token_canister_id,
        });
        const actorKongswap = Actor.createActor(idlFactoryKongswap, {
          agent,
          canisterId: KONGSWAP_CANISTER_ID_IC,
        });
        const fee = await icrc1_fee(actorTokenLedger);
        const decimals = await icrc1_decimals(actorTokenLedger);

        const price = await swap_amounts(actorKongswap, {
          from: token.toLocaleUpperCase(),
          to: "ckUSDC",
          amount: 1n,
        });

        const price_usd = price.mid_price;

        return {
          decimals,
          fee: Number(fee) / 10 ** decimals,
          fee_e8s: fee,
          fee_usd: (Number(fee) / 10 ** decimals) * price_usd,
          price_usd,
        };
      } catch (err) {
        console.log(err);
        throw new Error(`Fetch token data error! Please retry later.`);
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchTokenData;
