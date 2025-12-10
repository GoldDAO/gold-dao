import { Token } from "@shared/utils/tokens";

export interface TokenSwapData {
  token: Token;
  amount_e8s: bigint;
  amount_usd: number;
  user_balance: bigint;
  fee: bigint;
  decimals: number;
}
