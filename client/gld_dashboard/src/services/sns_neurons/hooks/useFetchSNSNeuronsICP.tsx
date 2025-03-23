import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { ListNeuronsResponse, NeuronWithMetric } from "../interfaces/icp";
import {
  formatTimestampToYearsDifference,
  getCurrentTimestampSeconds,
  formatTimestampToYears,
} from "@utils/dates";
import { divideBy1e8, roundAndFormatLocale } from "@utils/numbers";

import { idlFactory } from "../idlFactoryICP";

interface NeuronsICP {
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

const parseNeuronsICP = (neurons: Array<NeuronWithMetric>) => {
  const currentTimestampSeconds = getCurrentTimestampSeconds();
  const data = neurons.map((neuron) => {
    const id = Number(neuron.id);
    const dissolving = neuron.dissolving;
    const dissolve_delay = !dissolving
      ? formatTimestampToYearsDifference(
          currentTimestampSeconds + Number(neuron.dissolve_delay)
        )
      : formatTimestampToYears(
          Number(neuron.dissolve_delay) - currentTimestampSeconds
        );
    const staked_amount = divideBy1e8(neuron.staked_amount);
    const maturity = Number(neuron.maturity);
    return {
      id,
      dissolving,
      dissolve_delay,
      staked_amount,
      maturity,
    };
  });
  const totalStakedAmount = data.reduce(
    (acc, cur) => acc + cur.staked_amount,
    0
  );
  return {
    totalStakedAmount: {
      number: totalStakedAmount,
      string: roundAndFormatLocale({ number: totalStakedAmount }),
    },
    neurons: data,
  };
};

const useFetchSNSNeuronsICP = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<NeuronsICP>, "queryKey" | "queryFn"> = {}
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["FETCH_SNS_NEURONS_ICP"],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const results = (await actor.list_neurons()) as ListNeuronsResponse;

        const neurons = parseNeuronsICP(results?.neurons?.active ?? []);
        return neurons;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch SNS ICP neurons error! Please retry later.");
      }
    },
    placeholderData,
    refetchInterval,
    enabled,
  });
};

export default useFetchSNSNeuronsICP;
