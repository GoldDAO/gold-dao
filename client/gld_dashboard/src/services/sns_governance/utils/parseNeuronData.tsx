import { Buffer } from "buffer";

import { divideBy1e8 } from "@utils/numbers";
import {
  getCurrentTimestampSeconds,
  calculateTimeDifferenceInSeconds,
  getDateUTC,
} from "@utils/dates";

import { Neuron } from "../interfaces";
import { NeuronData, NervousSystemParameter } from "./interfaces";

interface ExtendedNeuron extends Neuron {
  total_maturity_e8s_equivalent?: number[];
}

const getState = (
  dissolveDelaySeconds: number | undefined,
  whenDissolvedTimestampSeconds: number | undefined,
  currentTimestampSeconds: number
) => {
  if (dissolveDelaySeconds && dissolveDelaySeconds !== 0) {
    return "Not dissolving";
  } else if (
    whenDissolvedTimestampSeconds &&
    whenDissolvedTimestampSeconds > currentTimestampSeconds
  ) {
    return "Dissolving";
  } else {
    return "Dissolved";
  }
};

const getDissolveDelay = (
  dissolveDelaySeconds: number | undefined,
  whenDissolvedTimestampSeconds: number | undefined,
  currentTimestampSeconds: number
) => {
  if (dissolveDelaySeconds) {
    return dissolveDelaySeconds + currentTimestampSeconds;
  } else if (whenDissolvedTimestampSeconds) {
    return whenDissolvedTimestampSeconds;
  }
  return undefined;
};

const getDissolveDelayBonus = (
  dissolveDelaySeconds: number | undefined,
  whenDissolvedTimestampSeconds: number | undefined,
  neuron_minimum_dissolve_delay_to_vote_seconds: number | undefined,
  maxDissolveDelaySeconds: number | undefined,
  maxDissolveDelayBonusPercentage: number | undefined,
  currentTimestampSeconds: number
) => {
  if (
    neuron_minimum_dissolve_delay_to_vote_seconds &&
    (dissolveDelaySeconds || whenDissolvedTimestampSeconds) &&
    maxDissolveDelaySeconds &&
    maxDissolveDelayBonusPercentage
  ) {
    if (dissolveDelaySeconds) {
      return (
        (dissolveDelaySeconds /
          (maxDissolveDelaySeconds * maxDissolveDelayBonusPercentage)) *
        100
      );
    } else if (whenDissolvedTimestampSeconds) {
      return (
        ((whenDissolvedTimestampSeconds - currentTimestampSeconds) /
          (maxDissolveDelaySeconds * maxDissolveDelayBonusPercentage)) *
        100
      );
    }
  }
  return undefined;
};

export type { NeuronData };

export const parseNeuronData = (
  neuron: ExtendedNeuron,
  nervousSystemParameters?: NervousSystemParameter
) => {
  const currentTimestampSeconds = getCurrentTimestampSeconds();
  const minDissolveDelayToVote =
    nervousSystemParameters?.neuron_minimum_dissolve_delay_to_vote_seconds; // voting power is only calculated for neurons that have been dissolved for more than 91 days

  const dissolveState = Array.isArray(neuron.dissolve_state)
    ? neuron.dissolve_state[0]
    : neuron.dissolve_state;

  const dissolveDelaySeconds =
    dissolveState &&
    "DissolveDelaySeconds" in dissolveState &&
    dissolveState.DissolveDelaySeconds
      ? Number(dissolveState.DissolveDelaySeconds)
      : undefined;

  const whenDissolvedTimestampSeconds =
    dissolveState &&
    "WhenDissolvedTimestampSeconds" in dissolveState &&
    dissolveState.WhenDissolvedTimestampSeconds
      ? Number(dissolveState.WhenDissolvedTimestampSeconds)
      : undefined;

  const stakedMaturityEquivalent = Array.isArray(
    neuron.staked_maturity_e8s_equivalent
  )
    ? neuron.staked_maturity_e8s_equivalent[0]
    : neuron.staked_maturity_e8s_equivalent;

  const totalMaturityEquivalent = Array.isArray(
    neuron.total_maturity_e8s_equivalent
  )
    ? neuron.total_maturity_e8s_equivalent[0]
    : neuron.total_maturity_e8s_equivalent;

  const id =
    Array.isArray(neuron.id) && neuron?.id[0]?.id
      ? Buffer.from(neuron.id[0].id).toString("hex")
      : neuron.id;

  const state = getState(
    dissolveDelaySeconds,
    whenDissolvedTimestampSeconds,
    currentTimestampSeconds
  );

  const dissolve_delay =
    getDissolveDelay(
      dissolveDelaySeconds,
      whenDissolvedTimestampSeconds,
      currentTimestampSeconds
    ) ?? undefined;

  const staked_amount = divideBy1e8(
    Number(neuron.cached_neuron_stake_e8s ?? 0)
  );

  const staked_maturity = divideBy1e8(Number(stakedMaturityEquivalent ?? 0));

  const total_maturity = divideBy1e8(Number(totalMaturityEquivalent ?? 0));

  const created_at = Number(neuron.created_timestamp_seconds);

  const age = Number(neuron.aging_since_timestamp_seconds);

  const auto_stake_maturity = neuron.auto_stake_maturity ? true : false;

  const max_neuron_age_for_age_bonus =
    nervousSystemParameters?.max_neuron_age_for_age_bonus;

  const max_age_bonus_percentage =
    nervousSystemParameters?.max_age_bonus_percentage;

  const age_bonus =
    dissolveDelaySeconds &&
    max_neuron_age_for_age_bonus &&
    max_age_bonus_percentage
      ? (((currentTimestampSeconds - age) / max_neuron_age_for_age_bonus) *
          max_age_bonus_percentage) /
        100
      : 0;

  const maxDissolveDelayBonusPercentage =
    nervousSystemParameters?.max_dissolve_delay_bonus_percentage;

  const maxDissolveDelaySeconds =
    nervousSystemParameters?.max_dissolve_delay_seconds;

  const neuron_minimum_dissolve_delay_to_vote_seconds =
    nervousSystemParameters?.neuron_minimum_dissolve_delay_to_vote_seconds;

  const dissolve_delay_bonus = getDissolveDelayBonus(
    dissolveDelaySeconds,
    whenDissolvedTimestampSeconds,
    neuron_minimum_dissolve_delay_to_vote_seconds,
    maxDissolveDelaySeconds,
    maxDissolveDelayBonusPercentage,
    currentTimestampSeconds
  );

  const voting_power =
    neuron_minimum_dissolve_delay_to_vote_seconds !== undefined &&
    dissolve_delay !== undefined &&
    dissolve_delay_bonus !== undefined &&
    minDissolveDelayToVote &&
    dissolve_delay >= neuron_minimum_dissolve_delay_to_vote_seconds &&
    calculateTimeDifferenceInSeconds(dissolve_delay) > minDissolveDelayToVote
      ? (staked_amount + staked_maturity) *
        (1 + age_bonus) *
        (1 + dissolve_delay_bonus)
      : undefined;

  const total_bonus = dissolve_delay_bonus
    ? (1 + age_bonus) * (1 + dissolve_delay_bonus) - 1
    : undefined;

  const data = {
    id,
    staked_amount,
    staked_maturity,
    total_maturity,
    age: dissolve_delay ? calculateTimeDifferenceInSeconds(age) : undefined,
    state: state.toLowerCase(),
    voting_power: voting_power ? Math.round(voting_power) : undefined,
    dissolve_delay: dissolve_delay
      ? calculateTimeDifferenceInSeconds(dissolve_delay)
      : undefined,
    created_at: created_at
      ? getDateUTC(created_at, { fromSeconds: true })
      : undefined,
    max_neuron_age_for_age_bonus: max_neuron_age_for_age_bonus ?? undefined,
    max_age_bonus_percentage: max_age_bonus_percentage ?? undefined,
    age_bonus: voting_power
      ? Math.round((age_bonus * 100 + Number.EPSILON) * 100) / 100
      : undefined,
    dissolve_delay_bonus:
      dissolve_delay_bonus && voting_power
        ? Math.round((dissolve_delay_bonus * 100 + Number.EPSILON) * 100) / 100
        : undefined,
    auto_stake_maturity,
    total_bonus:
      total_bonus && voting_power
        ? Math.round((total_bonus * 100 + Number.EPSILON) * 100) / 100
        : undefined,
    neuron_minimum_dissolve_delay_to_vote_seconds:
      neuron_minimum_dissolve_delay_to_vote_seconds ?? undefined,
  } as NeuronData;

  return data;
};
