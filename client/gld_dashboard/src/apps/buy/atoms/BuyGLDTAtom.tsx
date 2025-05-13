import { atomWithReducer } from "jotai/utils";
import { GLDT_LEDGER_CANISTER_ID_IC, MAX_SWAP_SLIPPAGE } from "@constants";
import Tokens, { Token } from "../utils";
import { SwapAmountsTxReply } from "@services/kongswap/interfaces";

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
  slippage: number | null;
  max_slippage: number | null;
  network_fee: bigint | null;
  lp_fee: bigint | null;
  is_open_confirm_dialog: boolean;
  is_open_details_dialog: boolean;
};

const initialState: BuyGLDTState = {
  pay_token: {
    token: Tokens[0], // default to ICP,
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
      label: "Gold Token",
      canisterId: GLDT_LEDGER_CANISTER_ID_IC,
    },
    amount: 0n,
    amount_usd: 0,
    amount_gold: 0n,
    decimals: null,
    fee: null,
  },
  slippage: null,
  max_slippage: MAX_SWAP_SLIPPAGE,
  network_fee: null,
  lp_fee: null,
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
        type: "SET_PRICE_DATA";
        value: {
          slippage: number;
          txs: Array<SwapAmountsTxReply>;
          receive_token_decimals: number;
          receive_token_amount: bigint;
        };
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
    case "SET_PRICE_DATA": {
      const { txs, slippage } = action.value;
      // const { txs, receive_token_decimals, receive_token_amount, slippage } =
      //   action.value;
      const network_fee = txs.reduce((acc, tx) => acc + tx.gas_fee, 0n);
      const lp_fee = txs.reduce((acc, tx) => acc + tx.lp_fee, 0n);

      // const network_fee_number =
      //   (Number(network_fee) + Number(lp_fee)) / 10 ** receive_token_decimals;

      // const receive_token_amount_number =
      //   Number(receive_token_amount) / 10 ** receive_token_decimals;

      // console.log("fee", network_fee_number);
      // console.log("slippage", slippage);
      // console.log(
      //   "slippage with fee deducted",
      //   Math.abs(network_fee_number / receive_token_amount_number - slippage)
      // );

      return {
        ...prev,
        slippage,
        network_fee,
        lp_fee,
      };
    }
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
