import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { NeuronFull } from "./interfaces";
import fetch_one_neuron from "../fetch_one_neuron";
import { numberToLocaleString } from "@utils/numbers";
import {
  calculateTimeDifferenceInSeconds,
  formatRoundedTimeUnits,
  getRoundedYears,
} from "@utils/dates";
import { idlFactory } from "../idlFactory";

const useFetchOneNeuron = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<NeuronFull>, "queryKey" | "queryFn"> & {
    neuronId: string;
    snsRootCanisterId: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    neuronId,
    snsRootCanisterId,
  } = options;

  return useQuery({
    queryKey: ["FETCH_ONE_NEURON", neuronId],
    queryFn: async () => {
      const actor = Actor.createActor(idlFactory, {
        agent,
        canisterId,
      });

      try {
        const neuron = await fetch_one_neuron({
          canisterId: snsRootCanisterId,
          actor,
          neuronId,
        });

        const isVotingPower =
          neuron.voting_power &&
          neuron.dissolve_delay &&
          neuron.neuron_minimum_dissolve_delay_to_vote_seconds &&
          calculateTimeDifferenceInSeconds(neuron.dissolve_delay as number) >
            neuron.neuron_minimum_dissolve_delay_to_vote_seconds
            ? true
            : false;

        const getDissolveDelay = (dissolve_delay: number | undefined) => {
          if (dissolve_delay) {
            if (getRoundedYears(dissolve_delay) > 0) {
              return formatRoundedTimeUnits(dissolve_delay);
            }
            return formatRoundedTimeUnits(dissolve_delay, {
              startFromUnit: "days",
              includeMinutesWithHours: true,
            });
          }
          return "-";
        };

        const getAge = (age: number | undefined) => {
          if (age) {
            if (getRoundedYears(age) > 0) {
              return formatRoundedTimeUnits(age);
            }
            return formatRoundedTimeUnits(age, {
              startFromUnit: "days",
              includeMinutesWithHours: true,
            });
          }
          return "-";
        };

        const getVotingPower = (
          voting_power: number | undefined,
          dissolve_delay: number | undefined,
          neuron_minimum_dissolve_delay_to_vote_seconds: number | undefined
        ) => {
          let displayedVotingPower = "-";
          if (voting_power) {
            displayedVotingPower = numberToLocaleString({
              value: voting_power,
              decimals: 2,
            });
          }
          if (
            dissolve_delay &&
            neuron_minimum_dissolve_delay_to_vote_seconds &&
            calculateTimeDifferenceInSeconds(dissolve_delay) <
              neuron_minimum_dissolve_delay_to_vote_seconds
          ) {
            displayedVotingPower = "None (< 91 days, cannot vote)";
          }
          return displayedVotingPower;
        };

        return {
          id: neuron.id,
          state: neuron.state,
          created_at: neuron.created_at,
          staked_amount: numberToLocaleString({
            value: neuron.staked_amount,
            decimals: 8,
          }),
          total_maturity: neuron.total_maturity
            ? numberToLocaleString({
                value: neuron.total_maturity,
                decimals: 2,
              })
            : "0",
          staked_maturity: neuron.staked_maturity
            ? numberToLocaleString({
                value: neuron.staked_maturity,
                decimals: 2,
              })
            : "0",
          dissolve_delay: getDissolveDelay(neuron.dissolve_delay),
          age: getAge(neuron.age),
          voting_power: getVotingPower(
            neuron.voting_power,
            neuron.dissolve_delay,
            neuron.neuron_minimum_dissolve_delay_to_vote_seconds
          ),
          is_voting_power: isVotingPower,
          auto_stake_maturity: neuron.auto_stake_maturity ? "Yes" : "No",
          dissolve_delay_bonus: isVotingPower
            ? `+${neuron.dissolve_delay_bonus}%`
            : "-",
          age_bonus: isVotingPower ? `+${neuron.age_bonus}%` : "-",
          total_bonus: isVotingPower ? `+${neuron.total_bonus}%` : "-",
          max_neuron_age_for_age_bonus: neuron.max_neuron_age_for_age_bonus
            ? formatRoundedTimeUnits(neuron.max_neuron_age_for_age_bonus)
            : "-",
          max_age_bonus_percentage: `+${neuron.max_age_bonus_percentage}%`,
        };
      } catch (err) {
        console.error(err);
        throw new Error("Fetch Gold DAO neuron error! Please retry later.");
      }
    },
    placeholderData,
    refetchInterval,
    enabled,
  });
};

export default useFetchOneNeuron;
