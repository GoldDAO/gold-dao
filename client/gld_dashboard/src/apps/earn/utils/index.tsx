import { Ledger } from "@services/ledger/utils/interfaces";

import {
  GLDT_LEDGER_CANISTER_ID,
  GOLDAO_LEDGER_CANISTER_ID,
  ICP_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";

export interface Token {
  id: Ledger;
  name: string;
  label: string;
  canisterId: string;
}

export const GLDTToken: Token = {
  id: "gldt",
  name: "GLDT",
  label: "Gold Token",
  canisterId: GLDT_LEDGER_CANISTER_ID,
};

export interface Position {
  id: bigint;
  amount: bigint;
}

export interface PositionRewards {
  amount: bigint;
  amount_usd: number;
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
  amount_usd: number;
  positions: Position[];
}

export const TokenRewardsList: Token[] = [
  {
    id: "goldao",
    name: "GOLDAO",
    label: "GOLDAO",
    canisterId: GOLDAO_LEDGER_CANISTER_ID,
  },
  {
    id: "ogy",
    name: "OGY",
    label: "Origyn",
    canisterId: OGY_LEDGER_CANISTER_ID,
  },
  {
    id: "icp",
    name: "ICP",
    label: "Internet Computer",
    canisterId: ICP_LEDGER_CANISTER_ID,
  },
];
