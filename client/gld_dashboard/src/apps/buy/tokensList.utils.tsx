import { Ledger } from "@services/ledger/utils/interfaces";

import {
  ICP_LEDGER_CANISTER_ID,
  // ICP_LEDGER_CANISTER_ID_IC,
  CK_USDT_LEDGER_CANISTER_ID_IC,
  CK_USDC_LEDGER_CANISTER_ID_IC,
} from "@constants";

export interface Token {
  id: Ledger;
  name: string;
  label: string;
  canisterId: string;
}

const tokensList: Token[] = [
  {
    id: "icp",
    name: "ICP",
    label: "Internet Computer",
    canisterId: ICP_LEDGER_CANISTER_ID, //ICP_LEDGER_CANISTER_ID_IC,
  },
  {
    id: "ckusdt",
    name: "ckUSDT",
    label: "ckUSDT",
    canisterId: CK_USDT_LEDGER_CANISTER_ID_IC,
  },
  {
    id: "ckusdc",
    name: "ckUSDC",
    label: "ckUSDC",
    canisterId: CK_USDC_LEDGER_CANISTER_ID_IC,
  },
];

export default tokensList;
