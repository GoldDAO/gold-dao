export type Ledger = "ogy" | "gldt" | "icp" | "goldao" | "ckusdt" | "ckusdc" | "wtn";

export interface Transaction {
  index: number;
  timestamp: string;
  from_account: string;
  to_account: string;
  amount: string;
  fee: string;
  memo: string;
  kind: string;
}

export interface Transactions {
  data: Transaction[];
  total_transactions: number;
  max_transaction_index?: number;
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

