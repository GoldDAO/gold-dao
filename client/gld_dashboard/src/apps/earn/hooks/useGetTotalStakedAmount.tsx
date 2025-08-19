import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { KONGSWAP_CANISTER_ID_IC, GLDT_LEDGER_CANISTER_ID } from "@constants";
import { idlFactory as idlFactoryStake } from "@services/gldt_stake/interfaces/idlFactory";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryKongswap } from "@services/kongswap/idlFactory";
import get_total_staked from "@services/gldt_stake/get_total_staked";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import swap_amounts from "@services/kongswap/swap_amounts";
import { TOKEN_GLDT } from "@shared/utils/tokens";

const useGetTotalStakedAmount = (
  canister_id: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<{ amount: number; amount_e8s: bigint; amount_usd: number }>,
    "queryKey" | "queryFn"
  >
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    ...queryOptions
  } = options;

  return useQuery({
    queryKey: [`FETCH_TOTAL_STAKED_AMOUNT`, canister_id],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactoryStake, {
          agent,
          canisterId: canister_id,
        });

        const actorKongswap = Actor.createActor(idlFactoryKongswap, {
          agent,
          canisterId: KONGSWAP_CANISTER_ID_IC,
        });

        const actorLedgerGLDT = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: GLDT_LEDGER_CANISTER_ID,
        });

        const total_amount_staked = await get_total_staked(actor);
        const decimals = await icrc1_decimals(actorLedgerGLDT);
        const price = await swap_amounts(actorKongswap, {
          from: TOKEN_GLDT.name,
          to: "ckUSDT",
          amount: 1n,
        });

        const amount = Number(total_amount_staked) / 10 ** decimals;

        return {
          amount,
          amount_e8s: total_amount_staked,
          amount_usd: price.mid_price * amount,
        };
      } catch (err) {
        console.error(err);
        throw new Error(`Fetch total staked amount error! Please retry later.`);
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

export default useGetTotalStakedAmount;
