import { Ledger } from "@services/ledger/utils/interfaces";

import { GLDT_LEDGER_CANISTER_ID } from "@constants";

export interface Token {
  id: Ledger;
  name: string;
  label: string;
  canisterId: string;
}

export const GLDTToken: Token = {
  id: "gldt",
  name: "GLDT",
  label: "GLDT",
  canisterId: GLDT_LEDGER_CANISTER_ID,
};
