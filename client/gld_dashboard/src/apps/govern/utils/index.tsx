export interface Neuron {
  id: string;
  reward: bigint;
  reward_usd: number;
}

export interface Reward {
  id: string;
  name: string;
  label: string;
  canister_id: string;
  is_selected: boolean;
  is_claimable: boolean;
  amount: bigint;
  amount_usd: number;
  neurons: Neuron[];
}
