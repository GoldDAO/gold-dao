import { Token } from "@shared/utils/tokens";

export interface UserStakedData {
  staked_amount: number;
  staked_amount_e8s: bigint;
  staked_amount_usd: number;
  remaining_dissolve_events?: number;
}

export interface Reward extends Token {
  is_selected: boolean;
  is_claimable: boolean;
  amount: number;
  amount_e8s: bigint;
  amount_usd: number;
  is_amount_below_fee: boolean;
}

export interface DissolveEvent {
  amount: number;
  amount_e8s: bigint;
  amount_usd: number;
  dissolved_date: number;
  is_withdrawable: boolean;
  remaining_time: number;
}

export interface Position {
  rewards: Reward[];
  created_at: bigint;
  instant_dissolve_fee: number;
  instant_dissolve_fee_e8s: bigint;
  instant_dissolve_fee_percentage: number;
  age_bonus_multiplier: number;
  staked_amount: number;
  staked_amount_e8s: bigint;
  staked_amount_usd: number;
  total_rewards_amount: number;
  total_rewards_amount_e8s: bigint;
  total_rewards_amount_usd: number;
  is_enable_claiming_rewards: boolean;
  dissolve_events: DissolveEvent[];
  dissolve_events_count: number;
  remaining_dissolve_events: number;
  max_dissolve_events: number;
  is_enable_withdrawing: boolean;
}
