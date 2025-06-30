import { ICP_LEDGER_CANISTER_ID, CKUSDT_LEDGER_CANISTER_ID } from "@constants";
import { Ledger } from "@services/ledger/utils/interfaces";

export interface Token {
  id: Ledger;
  name: string;
  label: string;
  canisterId: string;
}

export interface PayToken {
  token: Token;
  amount: bigint | null;
  amount_usd: number | null;
  decimals: number | null;
  user_balance: bigint | null;
  fee: bigint | null;
}

export interface ReceiveToken {
  token: Token;
  amount: bigint;
  amount_usd: number | null;
  amount_gold: bigint;
  fee: bigint | null;
  decimals: number | null;
}

export const TOKEN_LIST_AVAILABLE: Token[] = [
  {
    id: "icp",
    name: "ICP",
    label: "Internet Computer",
    canisterId: ICP_LEDGER_CANISTER_ID,
  },
  {
    id: "ckusdt",
    name: "ckUSDT",
    label: "ckUSDT",
    canisterId: CKUSDT_LEDGER_CANISTER_ID,
  },
];
