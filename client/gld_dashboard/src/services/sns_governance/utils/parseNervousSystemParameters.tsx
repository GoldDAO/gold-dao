import { NervousSystemParameters } from "../interfaces";
import { NervousSystemParameter } from "./interfaces";

export const parseNervousSystemParameters = (
  nervousSystemParameters?: NervousSystemParameters
) => {
  const max_dissolve_delay_seconds = nervousSystemParameters
    ?.max_dissolve_delay_seconds[0]
    ? Number(nervousSystemParameters.max_dissolve_delay_seconds[0])
    : undefined;

  const max_dissolve_delay_bonus_percentage = nervousSystemParameters
    ?.max_dissolve_delay_bonus_percentage[0]
    ? Number(nervousSystemParameters.max_dissolve_delay_bonus_percentage[0])
    : undefined;

  const neuron_minimum_stake_e8s = nervousSystemParameters
    ?.neuron_minimum_stake_e8s[0]
    ? Number(nervousSystemParameters.neuron_minimum_stake_e8s[0])
    : undefined;

  const max_neuron_age_for_age_bonus = nervousSystemParameters
    ?.max_neuron_age_for_age_bonus[0]
    ? Number(nervousSystemParameters?.max_neuron_age_for_age_bonus[0])
    : undefined;

  const initial_voting_period_seconds = nervousSystemParameters
    ?.initial_voting_period_seconds[0]
    ? Number(nervousSystemParameters?.initial_voting_period_seconds[0])
    : undefined;

  const neuron_minimum_dissolve_delay_to_vote_seconds = nervousSystemParameters
    ?.neuron_minimum_dissolve_delay_to_vote_seconds[0]
    ? Number(
        nervousSystemParameters.neuron_minimum_dissolve_delay_to_vote_seconds[0]
      )
    : undefined;

  const reject_cost_e8s = nervousSystemParameters?.reject_cost_e8s[0]
    ? Number(nervousSystemParameters.reject_cost_e8s[0])
    : undefined;

  const wait_for_quiet_deadline_increase_seconds = nervousSystemParameters
    ?.wait_for_quiet_deadline_increase_seconds[0]
    ? Number(
        nervousSystemParameters.wait_for_quiet_deadline_increase_seconds[0]
      )
    : undefined;

  const transaction_fee_e8s = nervousSystemParameters?.transaction_fee_e8s[0]
    ? Number(nervousSystemParameters.transaction_fee_e8s[0])
    : undefined;

  const max_age_bonus_percentage = nervousSystemParameters
    ?.max_age_bonus_percentage[0]
    ? Number(nervousSystemParameters?.max_age_bonus_percentage[0])
    : undefined;

  const initial_reward_rate_basis_points = nervousSystemParameters
    ?.voting_rewards_parameters?.[0]?.initial_reward_rate_basis_points?.[0]
    ? Number(
        nervousSystemParameters?.voting_rewards_parameters?.[0]
          ?.initial_reward_rate_basis_points?.[0]
      )
    : undefined;

  const data = {
    max_dissolve_delay_seconds,
    max_dissolve_delay_bonus_percentage,
    neuron_minimum_stake_e8s,
    max_neuron_age_for_age_bonus,
    initial_voting_period_seconds,
    neuron_minimum_dissolve_delay_to_vote_seconds,
    reject_cost_e8s,
    wait_for_quiet_deadline_increase_seconds,
    transaction_fee_e8s,
    max_age_bonus_percentage,
    initial_reward_rate_basis_points,
  } as NervousSystemParameter;

  return data;
};
