import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { ListNeuronsResponse } from "../interfaces/ogy";
import { parseNeuronsOGY } from "./utils/index";
import { idlFactory } from "../idlFactoryOGY";

interface NeuronsOGY {
  totalStakedAmount: {
    string: string;
    number: number;
  };
  neurons: Array<{
    id: number;
    dissolving: boolean;
    dissolve_delay: string;
    staked_amount: number;
    maturity: number;
  }>;
}

const useFetchSNSNeuronsOGY = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<NeuronsOGY>, "queryKey" | "queryFn"> = {}
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["FETCH_SNS_NEURONS_OGY"],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const results = (await actor.list_ogy_neurons()) as ListNeuronsResponse;
        const neurons = parseNeuronsOGY(results.neurons.ogy_neurons ?? []);
        return neurons;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch SNS OGY neurons error! Please retry later.");
      }
    },
    placeholderData,
    refetchInterval,
    enabled,
  });
};

export default useFetchSNSNeuronsOGY;
