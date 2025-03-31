export type DissolveState = "NotDissolving" | "Dissolving" | "Dissolved";

export type Reward =
  {
    name: string;
    amount: bigint;
  }


export type Stake = {
  is_dissolved: boolean;
  rewards: Reward[];
  created_at: bigint;
  id: bigint;
  unstake_early_fee: bigint;
  is_unstakable: boolean;
  is_unlockable: boolean;
  dissolve_state: string;
  age_bonus_multiplier: number;
  amount: bigint;
};
