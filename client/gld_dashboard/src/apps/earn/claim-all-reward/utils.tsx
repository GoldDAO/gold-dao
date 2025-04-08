import { Ledger } from "@services/ledger/utils/interfaces";

export interface Reward {
  id: Ledger;
  name: string;
  label: string;
  canister_id: string;
  is_selected: boolean;
  is_claimable: boolean;
  amount: bigint;
}
