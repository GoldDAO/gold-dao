import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  SNS_REWARDS_CANISTER_ID,
  SNS_GOVERNANCE_CANISTER_ID,
  KONGSWAP_CANISTER_ID_IC,
} from "@constants";

import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryGovernance } from "@services/sns_governance/idlFactory";
import { idlFactory as idlFactoryKongswap } from "@services/kongswap/idlFactory";
import { icrc1_balance_of } from "@services/ledger/icrc1_balance_of";
import list_neurons from "@services/sns_governance/list_neurons";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import swap_amounts from "@services/kongswap/swap_amounts";
import { TokensList } from "./index";
import { Ledger } from "@services/ledger/utils/interfaces";
import { Neuron } from "./index";

export type TokensRewards = {
  id: Ledger;
  amount: bigint;
  amount_usd: number;
  neurons: Neuron[];
};

const useGetAllNeuronsRewards = (
  options: Omit<
    UseQueryOptions<TokensRewards[], Error>,
    "queryKey" | "queryFn"
  > & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    agent,
    owner,
  } = options;

  return useQuery({
    queryKey: ["USER_NEURONS_REWARDS"],
    queryFn: async (): Promise<TokensRewards[]> => {
      try {
        const actor = Actor.createActor(idlFactoryGovernance, {
          agent,
          canisterId: SNS_GOVERNANCE_CANISTER_ID,
        });

        const actorKongswap = Actor.createActor(idlFactoryKongswap, {
          agent,
          canisterId: KONGSWAP_CANISTER_ID_IC,
        });

        const neurons = await list_neurons(actor, {
          limit: 100,
          start_page_at: null,
          owner,
        });

        const data = await Promise.all(
          TokensList.map(async (token) => {
            const actorLedger = Actor.createActor(idlFactoryLedger, {
              agent,
              canisterId: token.canisterId,
            });
            const decimals = await icrc1_decimals(actorLedger);
            const neuronData = await Promise.all(
              neurons.map(async (neuron) => {
                const reward = await icrc1_balance_of({
                  actor: actorLedger,
                  owner: SNS_REWARDS_CANISTER_ID,
                  subaccount: neuron.id,
                });

                const price = await swap_amounts(actorKongswap, {
                  from: token.name,
                  to: "ckUSDC",
                  amount: reward,
                });

                return {
                  id: neuron.id,
                  reward,
                  reward_usd:
                    price.mid_price * (Number(reward) / 10 ** decimals),
                };
              })
            );
            const amount = neuronData.reduce(
              (acc, curr) => acc + curr.reward,
              0n
            );
            const amount_usd = neuronData.reduce(
              (acc, curr) => acc + curr.reward_usd,
              0
            );
            return {
              id: token.id,
              amount,
              amount_usd,
              neurons: neuronData,
            };
          })
        );
        return data;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch neurons all rewards error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useGetAllNeuronsRewards;
