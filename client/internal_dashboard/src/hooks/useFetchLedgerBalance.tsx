import {
  useQuery,
  UseQueryOptions,
  UseQueryResult,
} from "@tanstack/react-query";
import { Actor, HttpAgent } from "@dfinity/agent";
import { ICPSWAP_CANISTER_ID } from "../constants";
import { idlFactory as idlFactoryLedger } from "../services/ledger/idlFactory";
import { idlFactory as idlFactoryIcpswap } from "../services/icpswap/idlFactory";
import icrc1_balance_of from "../services/ledger/icrc1_balance_of";
import icrc1_decimals from "../services/ledger/icrc1_decimals";
import get_token_price_usd from "../services/icpswap/get_token_price_usd";

interface LedgerBalanceData {
  amount: number;
  amount_e8s: bigint;
  amount_usd: number;
  decimals: number;
  price_usd: number;
}

interface UseFetchLedgerBalanceOptions
  extends Omit<UseQueryOptions<LedgerBalanceData>, "queryKey" | "queryFn"> {
  ledger: string;
  owner: string;
  subaccount: string;
}

type UseFetchLedgerBalanceResult = UseQueryResult<LedgerBalanceData, Error>;

const useFetchLedgerBalance = (
  canisterId: string,
  options: UseFetchLedgerBalanceOptions
): UseFetchLedgerBalanceResult => {
  const {
    ledger,
    owner,
    subaccount,
    enabled = true,
    refetchInterval = false,
    placeholderData = undefined,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    ...queryOptions
  } = options;

  return useQuery<LedgerBalanceData>({
    queryKey: ["FETCH_LEDGER_BALANCE", canisterId, ledger, owner, subaccount],
    queryFn: async (): Promise<LedgerBalanceData> => {
      const agent = await HttpAgent.create({ host: "https://ic0.app" });

      const actorLedger = Actor.createActor(idlFactoryLedger, {
        agent,
        canisterId,
      });

      const amount_e8s = await icrc1_balance_of({
        actor: actorLedger,
        owner,
        subaccount,
      });

      const actorIcpswap = Actor.createActor(idlFactoryIcpswap, {
        agent,
        canisterId: ICPSWAP_CANISTER_ID,
      });

      const decimals = await icrc1_decimals(actorLedger);

      const price_usd = await get_token_price_usd(
        actorIcpswap, canisterId, ledger, { agent }
      );

      const amount = Number(amount_e8s) / 10 ** decimals;
      const amount_usd = amount * price_usd;

      return {
        amount,
        amount_e8s,
        amount_usd,
        decimals,
        price_usd: amount * price_usd,
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
