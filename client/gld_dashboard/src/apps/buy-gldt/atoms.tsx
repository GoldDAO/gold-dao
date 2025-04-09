import { atomWithReducer } from "jotai/utils";

import { GLDT_LEDGER_CANISTER_ID_IC } from "@constants";

import TOKENS_LIST, { Token } from "./tokensList.utils";

type BuyGLDTState = {
  pay_token: {
    token: Token;
    amount: bigint | null;
    amount_usd: number | null;
    decimals: number | null;
    user_balance: bigint | null;
    fee: bigint | null;
  };
  receive_token: {
    token: Token;
    amount: bigint;
    amount_usd: number | null;
    amount_gold: bigint;
    fee: bigint | null;
    decimals: number | null;
  };
  is_open_confirm_dialog: boolean;
  is_open_details_dialog: boolean;
};

const initialState: BuyGLDTState = {
  pay_token: {
    token: TOKENS_LIST[0], // default to ICP,
    amount: null,
    amount_usd: null,
    decimals: null,
    user_balance: null,
    fee: null,
  },
  receive_token: {
    token: {
      id: "gldt",
      name: "GLDT",
      label: "GLDT",
      canisterId: GLDT_LEDGER_CANISTER_ID_IC,
    },
    amount: 0n,
    amount_usd: 0,
    amount_gold: 0n,
    decimals: null,
    fee: null,
  },
  is_open_confirm_dialog: false,
  is_open_details_dialog: false,
};

const reducer = (
  prev: BuyGLDTState,
  action:
    | {
        type: "SET_PAY_TOKEN";
        value: Token;
      }
    | {
        type: "SET_PAY_TOKEN_DATA";
        value: {
          amount: bigint;
          amount_usd: number;
          decimals: number;
          user_balance: bigint;
          fee: bigint;
        };
      }
    | {
        type: "SET_RECEIVE_TOKEN_DATA";
        value: {
          amount: bigint;
          amount_usd: number;
          amount_gold: bigint;
          decimals: number;
          fee: bigint;
        };
      }
    | { type: "OPEN_DIALOG_CONFIRM" }
    | { type: "OPEN_DIALOG_DETAILS" }
    | { type: "CONFIRM" }
    | { type: "CANCEL" }
    | { type: "RESET" }
) => {
  switch (action.type) {
    case "SET_PAY_TOKEN":
      return {
        ...prev,
        pay_token: {
          ...prev.pay_token,
          token: action.value,
        },
      };
    case "SET_PAY_TOKEN_DATA":
      return {
        ...prev,
        pay_token: {
          ...prev.pay_token,
          ...action.value,
        },
      };
    case "SET_RECEIVE_TOKEN_DATA":
      return {
        ...prev,
        receive_token: {
          ...prev.receive_token,
          ...action.value,
        },
      };
    case "OPEN_DIALOG_CONFIRM":
      return {
        ...prev,
        is_open_confirm_dialog: true,
      };
    case "OPEN_DIALOG_DETAILS":
      return {
        ...prev,
        is_open_confirm_dialog: true,
      };
    case "CANCEL":
      return { ...prev, is_open_confirm_dialog: false };
    case "CONFIRM":
      return {
        ...prev,
        is_open_confirm_dialog: false,
        is_open_details_dialog: true,
      };
    case "RESET":
      return initialState;
  }
};

export const BuyGLDTStateReducerAtom = atomWithReducer(initialState, reducer);

export default BuyGLDTStateReducerAtom;
