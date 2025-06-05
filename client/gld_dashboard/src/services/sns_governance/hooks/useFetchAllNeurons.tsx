import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { SortingState } from "@tanstack/react-table";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

import { NeuronPartial } from "./interfaces";
import fetch_all_neurons from "../fetch_all_neurons";
import { numberToLocaleString } from "@shared/utils/numbers";
import {
  calculateTimeDifferenceInSeconds,
  formatRoundedTimeUnits,
} from "@shared/utils/dates";

export interface Neurons {
  data: NeuronPartial[];
  total_neurons: number;
  max_neuron_index: number;
}

const useFetchAllNeurons = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<Neurons>, "queryKey" | "queryFn"> & {
    limit?: number;
    offset?: number;
    sorting?: SortingState;
    snsRootCanisterId: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    limit,
    offset,
    sorting,
    snsRootCanisterId,
  } = options;

  return useQuery({
    queryKey: ["FETCH_ALL_NEURONS", limit, offset, sorting],
    queryFn: async () => {
      const actor = Actor.createActor(idlFactory, {
        agent,
        canisterId,
      });

      let sort_by = undefined;
      if (sorting && sorting[0]) {
        const { id, desc } = sorting[0];
        sort_by = desc ? `-${id}` : id;
      }
      try {
        const results = await fetch_all_neurons({
          limit,
          offset,
          sort_by: sort_by ?? "",
          canisterId: snsRootCanisterId,
          actor,
        });

        const data = results.data.map((neuron) => {
          const isVotingPower =
            neuron.voting_power &&
            neuron.dissolve_delay &&
            neuron.neuron_minimum_dissolve_delay_to_vote_seconds &&
            calculateTimeDifferenceInSeconds(neuron.dissolve_delay as number) >
              neuron.neuron_minimum_dissolve_delay_to_vote_seconds
              ? true
              : false;

          return {
            id: neuron.id,
            state: neuron.state,
            created_at: neuron.created_at,
            staked_amount: numberToLocaleString({
              value: neuron.staked_amount,
            }),
            total_maturity: numberToLocaleString({
              value: neuron.total_maturity,
              decimals: 2,
            }),
            dissolve_delay: neuron.dissolve_delay
              ? formatRoundedTimeUnits(neuron.dissolve_delay)
              : "-",
            age: neuron.age ? formatRoundedTimeUnits(neuron.age) : "-",
            voting_power: neuron.voting_power
              ? numberToLocaleString({
                  value: neuron.voting_power,
                })
              : "-",
            is_voting_power: isVotingPower,
            auto_stake_maturity: neuron.auto_stake_maturity ? "True" : "False",
            dissolve_delay_bonus: isVotingPower
              ? `+${neuron.dissolve_delay_bonus}%`
              : "-",
            age_bonus: isVotingPower ? `+${neuron.age_bonus}%` : "-",
            total_bonus: isVotingPower ? `+${neuron.total_bonus}%` : "-",
          };
        });

        return {
          data: data,
          total_neurons: results.total_neurons,
          max_neuron_index: results.max_neuron_index,
        };
      } catch (err) {
        console.error(err);
        throw new Error("Fetch all neurons error! Please retry later.");
      }
    },
    placeholderData,
    refetchInterval,
    enabled,
  });
};

export default useFetchAllNeurons;
