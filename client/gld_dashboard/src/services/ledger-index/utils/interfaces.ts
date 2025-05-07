export type Ledger = "ogy" | "gldt" | "icp" | "goldao" | "ckusdt" | "ckusdc" | "wtn";

export interface Transaction {
  index: number;
  timestamp: string;
  from: string | undefined;
  to: string | undefined;
  amount: bigint | undefined;
  fee: bigint | undefined;
  memo: string | undefined;
  kind: string;
  is_credit: boolean;
}

export interface Transactions {
  data: Transaction[];
  cursor_index?: number | null;
}

export interface Account {
  id: string;
  owner: string;
  subaccount: string;
  balance: string;
  total_transactions: number;
  created_timestamp: number;
  latest_transaction_index: number;
  updated_at: string;
}