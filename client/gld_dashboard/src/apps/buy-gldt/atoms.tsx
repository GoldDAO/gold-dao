import { atomWithReset } from "jotai/utils";

import { GLDT_LEDGER_CANISTER_ID_IC } from "@constants";

import TOKENS_LIST, { Token } from "./tokensList.utils";

const BuyGLDTStateAtom = atomWithReset<{
  pay_amount: bigint | null;
  receive_amount: number;
  pay_token: Token;
  receive_token: Token;
  pay_token_decimals: number | null;
  pay_token_user_balance: bigint | null;
  is_open_confirm_dialog: boolean;
  is_open_details_dialog: boolean;
  is_new_swap: boolean;
}>({
  pay_amount: null,
  receive_amount: 0,
  pay_token: TOKENS_LIST[0], // default to ICP
  receive_token: {
    id: "gldt",
    name: "GLDT",
    label: "GLDT",
    canisterId: GLDT_LEDGER_CANISTER_ID_IC,
  },
  pay_token_decimals: null,
  pay_token_user_balance: null,
  is_open_confirm_dialog: false,
  is_open_details_dialog: false,
  is_new_swap: true,
});

export default BuyGLDTStateAtom;
