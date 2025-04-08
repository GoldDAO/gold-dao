export interface NeuronData {
  staked_amount: bigint;
  staked_maturity: number;
  total_maturity: number;
  age: number | undefined;
  state: string;
  voting_power: number | undefined;
  dissolve_delay: number | undefined;
  id: string;
  created_at: string;
  max_neuron_age_for_age_bonus: number | undefined;
  max_age_bonus_percentage: number | undefined;
  age_bonus: number | undefined;
  dissolve_delay_bonus: number | undefined;
  vestingPeriod?: number | undefined;
  auto_stake_maturity: boolean;
  total_bonus: number | undefined;
  neuron_minimum_dissolve_delay_to_vote_seconds: number | undefined;
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

export interface NeuronsData {
  data: NeuronData[];
  cursor_index: string | null;
}

export interface NeuronPartialData {
  data: NeuronPartialData[];
  cursor_index: string | null;
}

export interface NeuronUser {
  id: string;
  staked_amount: bigint;
  state: string;
  dissolve_delay: string;
  age: string;
}

export interface NeuronUserData {
  data: NeuronUser[];
  cursor_index: string | null;
}

export interface Proposal {
  action: string;
  decided_timestamp_seconds: number;
  executed_timestamp_seconds: number;
  failed_timestamp_seconds: number;
  failure_reason: string | null;
  id: string;
  initial_voting_period_seconds: number;
  is_eligible_for_rewards: boolean;
  latest_tally: {
    no: number;
    yes: number;
    total: number;
    timestamp_seconds: number;
  };
  minimum_yes_proportion_of_exercised: {
    basis_points: number;
  };
  minimum_yes_proportion_of_total: {
    basis_points: number;
  };
  nervous_system_function: {
    id: string;
    name: string;
  };
  payload_text_rendering: string;
  proposal_action_payload: string | null;
  proposal_action_type: string;
  proposal_creation_timestamp_seconds: number;
  proposal_title: string;
  proposal_url: string;
  proposer: string;
  reject_cost_e8s: number;
  reward_event_end_timestamp_seconds: number;
  reward_event_round: number;
  reward_status: string;
  root_canister_id: string;
  status: string;
  summary: string;
  votes: string;
  wait_for_quiet_deadline_increase_seconds: number;
  wait_for_quiet_state_current_deadline_timestamp_seconds: number;
}

export interface ProposalData {
  id: string;
  proposer: string;
  title: string;
  url: string;
  created_at: string;
  timeRemaining: number;
  type: string;
  summary: string;
  status: string;
  payload: string;
  latestTally: {
    yes: number;
    no: number;
    total: number;
  };
  votes: number;
  riskedOGY: number;
}

export interface NervousSystemParameter {
  max_dissolve_delay_seconds: number;
  max_dissolve_delay_bonus_percentage: number;
  neuron_minimum_stake_e8s: number;
  max_neuron_age_for_age_bonus: number;
  initial_voting_period_seconds: number;
  neuron_minimum_dissolve_delay_to_vote_seconds: number;
  reject_cost_e8s: number;
  wait_for_quiet_deadline_increase_seconds: number;
  transaction_fee_e8s: number;
  max_age_bonus_percentage: number;
  initial_reward_rate_basis_points: number;
}
