import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  SNS_REWARDS_CANISTER_ID,
  SNS_GOVERNANCE_CANISTER_ID,
} from "@constants";

import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryGovernance } from "@services/sns_governance/idlFactory";
import { icrc1_balance_of } from "@services/ledger/icrc1_balance_of";
import list_neurons from "@services/sns_governance/list_neurons";
import { TokensList } from "../../utils/index";
import { Ledger } from "@services/ledger/utils/interfaces";
import { Neuron } from "./index";

export type TokensRewards = {
  id: Ledger;
  amount: bigint;
  neurons: Neuron[];
};

const useGetTokenTotalStakedAmount = (
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
    queryKey: ["USER_NEURONS_ALL_TOTAL_STAKED_REWARDS"],
    queryFn: async (): Promise<TokensRewards[]> => {
      try {
        const actor = Actor.createActor(idlFactoryGovernance, {
          agent,
          canisterId: SNS_GOVERNANCE_CANISTER_ID,
        });

        const neurons = await list_neurons(actor, {
          limit: 100,
          start_page_at: null,
          owner,
        });

        const data = await Promise.all(
          TokensList.map(async (token) => {
            const neuronStakedAmount = await Promise.all(
              neurons.map(async (neuron) => {
                const actorLedger = Actor.createActor(idlFactoryLedger, {
                  agent,
                  canisterId: token.canisterId,
                });
                const neuronStakedAmount = await icrc1_balance_of({
                  actor: actorLedger,
                  owner: SNS_REWARDS_CANISTER_ID,
                  subaccount: neuron.id,
                });

                return {
                  id: neuron.id,
                  staked_amount: neuronStakedAmount,
                };
              })
            );
            const neuronsStakedAmount = neuronStakedAmount.reduce(
              (acc, curr) => acc + curr.staked_amount,
              0n
            );
            return {
              id: token.id,
              amount: neuronsStakedAmount,
              neurons: neuronStakedAmount,
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

export default useGetTokenTotalStakedAmount;
