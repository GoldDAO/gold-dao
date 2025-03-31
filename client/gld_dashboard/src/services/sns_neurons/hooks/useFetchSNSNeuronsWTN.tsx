import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { ListNeuronsResponse } from "../interfaces/ogy";
import { parseNeuronsOGY } from "./utils/index";
import { idlFactory } from "../idlFactoryOGY";

interface NeuronsWTN {
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

const useFetchSNSNeuronsWTN = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<NeuronsWTN>, "queryKey" | "queryFn"> = {}
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["FETCH_SNS_NEURONS_WTN"],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const results = (await actor.list_ogy_neurons()) as ListNeuronsResponse;
        const neurons = parseNeuronsOGY(results.neurons.wtn_neurons ?? []);
        return neurons;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch SNS WTN neurons error! Please retry later.");
      }
    },
    placeholderData,
    refetchInterval,
    enabled,
  });
};

export default useFetchSNSNeuronsWTN;
