import { SortingState } from "@tanstack/react-table";

export interface TableParams {
  limit?: number;
  offset?: number;
  sorting?: SortingState;
}

export interface NeuronPartial {
  staked_amount: string;
  total_maturity: string;
  age: string;
  state: string;
  voting_power: string;
  dissolve_delay: string;
  id: string;
  created_at: string;
  age_bonus: string;
  dissolve_delay_bonus: string;
  auto_stake_maturity: string;
  total_bonus: string;
  is_voting_power: boolean;
}

export interface NeuronUser {
  id: string;
  created_at: string;
  age_bonus: string;
  dissolve_delay_bonus: string;
  auto_stake_maturity: string;
  total_bonus: string;
}

export interface NeuronFull extends NeuronPartial {
  staked_maturity: string;
  max_neuron_age_for_age_bonus: string;
  max_age_bonus_percentage: string;
  vesting_period?: number | undefined;
}