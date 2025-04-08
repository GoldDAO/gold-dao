import { Ledger } from "@services/ledger/utils/interfaces";

export interface Neuron {
  id: string;
  staked_amount: bigint;
}

export interface Reward {
  id: Ledger;
  name: string;
  label: string;
  canister_id: string;
  is_selected: boolean;
  is_claimable: boolean;
  amount: bigint;
  neurons: Neuron[];
}
