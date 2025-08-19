import {
  useQuery,
  UseQueryOptions,
  UseQueryResult,
} from "@tanstack/react-query";
import { Actor, HttpAgent } from "@dfinity/agent";
import { KONGSWAP_CANISTER_ID_IC } from "../constants";
import { idlFactory as idlFactoryLedger } from "../services/ledger/idlFactory";
import { idlFactory as idlFactoryKongswap } from "../services/kongswap/idlFactory";
import icrc1_balance_of from "../services/ledger/icrc1_balance_of";
import icrc1_decimals from "../services/ledger/icrc1_decimals";
import swap_amounts from "../services/kongswap/swap_amounts";

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

      const actorKongswap = Actor.createActor(idlFactoryKongswap, {
        agent,
        canisterId: KONGSWAP_CANISTER_ID_IC,
      });

      const decimals = await icrc1_decimals(actorLedger);

      const price = await swap_amounts(actorKongswap, {
        from: ledger,
        to: "ckUSDT",
        amount: 1n,
      });

      const amount = Number(amount_e8s) / 10 ** decimals;
      const amount_usd = amount * price.mid_price;

      return {
        amount,
        amount_e8s,
        amount_usd,
        decimals,
        price_usd: amount * price.mid_price,
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
