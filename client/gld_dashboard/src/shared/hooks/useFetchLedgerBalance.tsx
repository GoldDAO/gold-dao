import {
  useQuery,
  // keepPreviousData,
  UseQueryOptions,
  UseQueryResult,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { ICPSWAP_CANISTER_ID } from "@constants";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryIcpswap } from "@services/icpswap/idls/swap_pool";
import { icrc1_balance_of } from "@services/ledger/icrc1_balance_of";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import icrc1_fee from "@services/ledger/icrc1_fee";
import get_token_price_usd from "@services/icpswap/get_token_price_usd";

interface LedgerBalanceData {
  balance: number;
  balance_e8s: bigint;
  balance_usd: number;
  decimals: number;
  fee: number;
  fee_usd: number;
  fee_e8s: bigint;
  price_usd: number;
}

interface UseFetchLedgerBalanceOptions
  extends Omit<UseQueryOptions<LedgerBalanceData>, "queryKey" | "queryFn"> {
  ledger: string;
  owner: string;
}

type UseFetchLedgerBalanceResult = UseQueryResult<LedgerBalanceData, Error>;

const useFetchLedgerBalance = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: UseFetchLedgerBalanceOptions
): UseFetchLedgerBalanceResult => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = undefined,
    ledger,
    owner,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    ...queryOptions
  } = options;

  return useQuery<LedgerBalanceData>({
    queryKey: ["FETCH_LEDGER_BALANCE", ledger, owner, canisterId],
    queryFn: async (): Promise<LedgerBalanceData> => {
      const actorLedger = Actor.createActor(idlFactoryLedger, {
        agent,
        canisterId,
      });

      const balance_e8s = await icrc1_balance_of({
        actor: actorLedger,
        owner,
      });
      const actorIcpswap = Actor.createActor(idlFactoryIcpswap, {
        agent,
        canisterId: ICPSWAP_CANISTER_ID,
      });
      const fee_e8s = await icrc1_fee(actorLedger);
      const decimals = await icrc1_decimals(actorLedger);

      const price_usd = await get_token_price_usd(
        actorIcpswap, canisterId, ledger, { agent }
      );
      const fee = Number(fee_e8s) / 10 ** decimals;
      const balance = Number(balance_e8s) / 10 ** decimals;
      const balance_usd = balance * price_usd;

      return {
        balance,
        balance_e8s,
        balance_usd,
        decimals,
        fee,
        fee_e8s,
        fee_usd: fee * price_usd,
        price_usd,
      };
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

export default useFetchLedgerBalance;
export type {
  LedgerBalanceData,
  UseFetchLedgerBalanceOptions,
  UseFetchLedgerBalanceResult,
};
