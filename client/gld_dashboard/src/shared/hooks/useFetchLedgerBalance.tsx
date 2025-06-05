import {
  useQuery,
  // keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryKongswap } from "@services/kongswap/idlFactory";
import { icrc1_balance_of } from "@services/ledger/icrc1_balance_of";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import icrc1_fee from "@services/ledger/icrc1_fee";
import swap_amounts from "@services/kongswap/swap_amounts";

const useFetchLedgerBalance = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<{
      balance: number;
      balance_e8s: bigint;
      balance_usd: number;
      decimals: number;
      fee: number;
      fee_usd: number;
      fee_e8s: bigint;
    }>,
    "queryKey" | "queryFn"
  > & {
    ledger: string;
    owner: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = undefined,
    ledger,
    owner,
  } = options;

  return useQuery({
    queryKey: ["FETCH_LEDGER_BALANCE_V2", ledger, owner],
    queryFn: async () => {
      const actorLedger = Actor.createActor(idlFactoryLedger, {
        agent,
        canisterId,
      });

      const balance_e8s = await icrc1_balance_of({
        actor: actorLedger,
        owner,
      });
      const actorKongswap = Actor.createActor(idlFactoryKongswap, {
        agent,
        canisterId: KONGSWAP_CANISTER_ID_IC,
      });
      const fee = await icrc1_fee(actorLedger);
      const decimals = await icrc1_decimals(actorLedger);

      const price = await swap_amounts(actorKongswap, {
        from: ledger,
        to: "ckUSDC",
        amount: 1n,
      });

      const balance_usd = price.mid_price;

      return {
        balance: Number(balance_e8s) / 10 ** decimals,
        balance_e8s,
        balance_usd,
        decimals,
        fee: Number(fee) / 10 ** decimals,
        fee_e8s: fee,
        fee_usd: (Number(fee) / 10 ** decimals) * balance_usd,
      };
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchLedgerBalance;
