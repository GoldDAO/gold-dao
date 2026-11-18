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
import { get_token_price_usd_or_null } from "@services/icpswap/get_token_price_usd";

const useFetchTokenData = (
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<{
      decimals: number;
      fee: number;
      fee_usd: number;
      fee_e8s: bigint;
      price_usd: number;
      // False for a token the user added that has no price; the USD values
      // are 0.
      has_price: boolean;
    }>,
    "queryKey" | "queryFn"
  > & {
    token: string;
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
        const actorIcpswap = Actor.createActor(idlFactoryIcpswap, {
          agent,
          canisterId: ICPSWAP_CANISTER_ID,
        });
        const fee = await icrc1_fee(actorTokenLedger);
        const decimals = await icrc1_decimals(actorTokenLedger);

        const price = await get_token_price_usd_or_null(
          actorIcpswap,
          token_canister_id,
          token,
          { agent }
        );
        const price_usd = price ?? 0;

        return {
          decimals,
          fee: Number(fee) / 10 ** decimals,
          fee_e8s: fee,
          fee_usd: (Number(fee) / 10 ** decimals) * price_usd,
          price_usd,
          has_price: price !== null,
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
