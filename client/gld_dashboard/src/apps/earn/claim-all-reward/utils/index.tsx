import { Ledger } from "@services/ledger/utils/interfaces";
export interface Position {
  id: bigint;
  amount: bigint;
}

export interface PositionRewards {
  amount: bigint;
  positions: Position[];
  name: string;
}

export interface Reward {
  id: Ledger;
  name: string;
  label: string;
  canister_id: string;
  is_selected: boolean;
  is_claimable: boolean;
  amount: bigint;
  positions: Position[];
}
