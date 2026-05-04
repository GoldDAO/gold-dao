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
import { TokenName } from "@shared/utils/tokens";

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
    token: TokenName;
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

        const price_usd = await get_token_price_usd(
          actorIcpswap,
          token_canister_id,
          token,
          { agent }
        );

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
