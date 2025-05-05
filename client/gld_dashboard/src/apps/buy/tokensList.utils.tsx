import { Ledger } from "@services/ledger/utils/interfaces";

import {
  ICP_LEDGER_CANISTER_ID,
  CK_USDT_LEDGER_CANISTER_ID_IC,
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
    canisterId: ICP_LEDGER_CANISTER_ID,
  },
  {
    id: "ckusdt",
    name: "ckUSDT",
    label: "ckUSDT",
    canisterId: CK_USDT_LEDGER_CANISTER_ID_IC,
  },
];

export default tokensList;
