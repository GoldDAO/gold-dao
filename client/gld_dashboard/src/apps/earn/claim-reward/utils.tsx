import { Ledger } from "@services/ledger/utils/interfaces";

import {
  GOLDAO_LEDGER_CANISTER_ID,
  ICP_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";

export interface TokenData {
  id: Ledger;
  name: string;
  label: string;
  canisterId: string;
}

export const Token: {
  GOLDAO: TokenData;
  ICP: TokenData;
  OGY: TokenData;
} = {
  GOLDAO: {
    id: "goldao",
    name: "GOLDAO",
    label: "GOLDAO",
    canisterId: GOLDAO_LEDGER_CANISTER_ID,
  },
  ICP: {
    id: "icp",
    name: "ICP",
    label: "Internet Computer",
    canisterId: ICP_LEDGER_CANISTER_ID,
  },
  OGY: {
    id: "ogy",
    name: "OGY",
    label: "OGY",
    canisterId: OGY_LEDGER_CANISTER_ID,
  },
};
