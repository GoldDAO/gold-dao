import { atomWithReducer } from "jotai/utils";

import { GLDT_LEDGER_CANISTER_ID_IC, GLDT_VALUE_1G_NFT } from "@constants";

import TOKENS_LIST, { Token } from "./tokensList.utils";

type BuyGLDTState = {
  pay_token: {
    token: Token;
    decimals: number | null;
    user_balance: bigint | null;
    fee: bigint | null;
    amount: bigint | null;
  };
  receive_token: {
    token: Token;
    amount: bigint;
    fee: bigint | null;
    amount_in_gold: bigint;
    decimals: number | undefined;
  };
  is_open_confirm_dialog: boolean;
  is_open_details_dialog: boolean;
};

const initialState: BuyGLDTState = {
  pay_token: {
    token: TOKENS_LIST[0], // default to ICP,
    decimals: null,
    user_balance: null,
    fee: null,
    amount: null,
  },
  receive_token: {
    token: {
      id: "gldt",
      name: "GLDT",
      label: "GLDT",
      canisterId: GLDT_LEDGER_CANISTER_ID_IC,
    },
    amount: 0n,
    amount_in_gold: 0n,
    fee: null,
    decimals: undefined,
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
          decimals: number;
          user_balance: bigint;
          fee: bigint;
          amount: number;
        };
      }
    | {
        type: "SET_RECEIVE_TOKEN_DATA";
        value: {
          amount: bigint;
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
          decimals: action.value.decimals,
          user_balance: action.value.user_balance,
          fee: action.value.fee,
          amount: BigInt(
            Math.round(action.value.amount * 10 ** action.value.decimals)
          ),
        },
      };
    case "SET_RECEIVE_TOKEN_DATA":
      return {
        ...prev,
        receive_token: {
          ...prev.receive_token,
          amount: action.value.amount,
          amount_in_gold: action.value.amount / BigInt(GLDT_VALUE_1G_NFT),
          decimals: action.value.decimals,
          fee: action.value.fee,
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
