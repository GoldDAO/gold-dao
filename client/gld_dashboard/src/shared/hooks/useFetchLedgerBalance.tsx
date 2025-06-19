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
      price_usd: number;
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
    queryKey: ["FETCH_LEDGER_BALANCE", ledger, owner],
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
      const fee_e8s = await icrc1_fee(actorLedger);
      const decimals = await icrc1_decimals(actorLedger);

      const price = await swap_amounts(actorKongswap, {
        from: ledger,
        to: "ckUSDT",
        amount: BigInt(1 * 10 ** decimals),
      });
      const fee = Number(fee_e8s) / 10 ** decimals;
      const balance = Number(balance_e8s) / 10 ** decimals;
      const balance_usd = balance * price.mid_price;

      return {
        balance,
        balance_e8s,
        balance_usd,
        decimals,
        fee,
        fee_e8s,
        fee_usd: fee * price.mid_price,
        price_usd: price.mid_price,
      };
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchLedgerBalance;
