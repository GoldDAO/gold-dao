import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SNS_GOVERNANCE_CANISTER_ID } from "@constants";

import { idlFactory as idlFactoryGovernance } from "@services/sns_governance/idlFactory";
import list_neurons from "@services/sns_governance/list_neurons";

const useGetAllNeuronsTotalStakedAmount = (
  options: Omit<UseQueryOptions<bigint, Error>, "queryKey" | "queryFn"> & {
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
    queryKey: ["USER_NEURONS_TOKEN_TOTAL_STAKED_AMOUNT"],
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
            return neuron.staked_amount;
          })
        );

        const data = results.reduce((acc, curr) => acc + curr, 0n);

        return data;
      } catch (err) {
        console.log(err);
        throw new Error(
          "Fetch neurons total staked amount error! Please retry later."
        );
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useGetAllNeuronsTotalStakedAmount;
