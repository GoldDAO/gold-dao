export type DissolveState = "NotDissolving" | "Dissolving" | "Dissolved";

export type Stake = {
  is_dissolved: boolean;
  claimable_rewards: {
    list: {
      name: string;
      amount: bigint;
      is_claimable: boolean;
    }[];
    total_amount: bigint;
  };
  created_at: bigint;
  id: bigint;
  unstake_early_fee: bigint;
  is_unstakable: boolean;
  is_unlockable: boolean;
  dissolve_state: string;
  age_bonus_multiplier: number;
  amount: bigint;
};
