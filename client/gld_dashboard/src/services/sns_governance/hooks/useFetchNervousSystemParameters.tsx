import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

export interface NervousSystemParametersData {
  status: string;
  token_name: string;
  token_symbol: string;
  max_dissolve_delay: string;
  max_dissolve_delay_bonus_percentage: number;
  neuron_minimum_stake: number;
  max_neuron_age_for_age_bonus: string;
  initial_voting_period: string;
  neuron_minimum_dissolve_delay_to_vote: string;
  reject_cost: number;
  wait_for_quiet_deadline_increase: string;
  transaction_fee: number;
  max_age_bonus_percentage: number;
  initial_reward_rate_basis_points: number;
}

import get_nervous_system_parameters from "../get_nervous_system_parameters";
import { divideBy1e8 } from "@shared/utils/numbers";
import { formatRoundedTimeUnits } from "@shared/utils/dates";

const useFetchNervousSystemParameters = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<NervousSystemParametersData>,
    "queryKey" | "queryFn"
  > = {}
) => {
  const { enabled = true, placeholderData = keepPreviousData } = options;

  return useQuery({
    queryKey: ["FETCH_NERVOUS_SYSTEM_PARAMETERS"],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const data = await get_nervous_system_parameters({
          actor,
        });

        return {
          status: "SNS Launched",
          token_name: "Gold Governance Token",
          token_symbol: "GLDGov",
          reject_cost: divideBy1e8(data.reject_cost_e8s),
          neuron_minimum_stake: divideBy1e8(data.neuron_minimum_stake_e8s),
          transaction_fee: divideBy1e8(data.transaction_fee_e8s),
          max_dissolve_delay: formatRoundedTimeUnits(
            data.max_dissolve_delay_seconds
          ),
          max_dissolve_delay_bonus_percentage:
            data.max_dissolve_delay_bonus_percentage,
          neuron_minimum_dissolve_delay_to_vote: formatRoundedTimeUnits(
            data.neuron_minimum_dissolve_delay_to_vote_seconds,
            { startFromUnit: "days" }
          ),
          initial_voting_period: formatRoundedTimeUnits(
            data.initial_voting_period_seconds
          ),
          wait_for_quiet_deadline_increase: formatRoundedTimeUnits(
            data.wait_for_quiet_deadline_increase_seconds
          ),
          max_neuron_age_for_age_bonus: formatRoundedTimeUnits(
            data.max_neuron_age_for_age_bonus
          ),
          max_age_bonus_percentage: data.max_age_bonus_percentage,
          initial_reward_rate_basis_points:
            data.initial_reward_rate_basis_points / 100,
        };
      } catch (err) {
        console.error(err);
        throw new Error(
          "Fetch nervous system parameters error! Please retry later."
        );
      }
    },
    placeholderData,
    enabled,
  });
};

export default useFetchNervousSystemParameters;
