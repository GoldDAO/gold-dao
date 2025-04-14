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

const useGetTokenTotalStakedRewards = (
  options: Omit<UseQueryOptions<bigint, Error>, "queryKey" | "queryFn"> & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
    canisterIdLedger: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    agent,
    owner,
    canisterIdLedger,
  } = options;

  return useQuery({
    queryKey: ["USER_NEURONS_TOKEN_TOTAL_STAKED_REWARDS"],
    queryFn: async (): Promise<bigint> => {
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

        const results = await Promise.all(
          neurons.map(async (neuron) => {
            const actorLedger = Actor.createActor(idlFactoryLedger, {
              agent,
              canisterId: canisterIdLedger,
            });
            const neuronStakedAmount = await icrc1_balance_of({
              actor: actorLedger,
              owner: SNS_REWARDS_CANISTER_ID,
              subaccount: neuron.id,
            });
            return neuronStakedAmount;
          })
        );

        const data = results.reduce((acc, curr) => acc + curr, 0n);

        return data;
      } catch (err) {
        console.log(err);
        throw new Error(
          "Fetch neurons total staked rewards error! Please retry later."
        );
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useGetTokenTotalStakedRewards;
