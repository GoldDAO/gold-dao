import { Ledger } from "@services/ledger/utils/interfaces";
import { ICP_LEDGER_CANISTER_ID, CKUSDT_LEDGER_CANISTER_ID } from "@constants";

export interface Token {
  id: Ledger;
  name: string;
  label: string;
  canisterId: string;
}

const Tokens: Token[] = [
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

export default Tokens;
