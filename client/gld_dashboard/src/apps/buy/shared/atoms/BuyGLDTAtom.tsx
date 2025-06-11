import { atomWithReducer } from "jotai/utils";
import {
  ICP_LEDGER_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID_IC,
  MAX_SWAP_SLIPPAGE,
} from "@constants";
import { SwapAmountsTxReply } from "@services/kongswap/interfaces";
import { Token, PayToken, ReceiveToken } from "@buy/shared/utils";

type BuyGLDTState = {
  pay_token: PayToken;
  receive_token: ReceiveToken;
  slippage: number;
  max_slippage: number;
  network_fee: bigint;
  lp_fee: bigint;
  is_open_confirm_dialog: boolean;
  is_open_details_dialog: boolean;
  is_open_disclaimer_confirm_high_slippage_dialog: boolean;
};

const initialState: BuyGLDTState = {
  pay_token: {
    token: {
      id: "icp",
      name: "ICP",
      label: "Internet Computer",
      canisterId: ICP_LEDGER_CANISTER_ID,
    },
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
  slippage: 0,
  max_slippage: MAX_SWAP_SLIPPAGE,
  network_fee: 0n,
  lp_fee: 0n,
  is_open_confirm_dialog: false,
  is_open_details_dialog: false,
  is_open_disclaimer_confirm_high_slippage_dialog: false,
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
    | { type: "OPEN_CONFIRM_HIGH_SLIPPAGE" }
    | { type: "OPEN_DIALOG_DETAILS" }
    | { type: "CONFIRM" }
    | { type: "CONFIRM_HIGH_SLIPPAGE"; value: { slippage: number } }
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
      const { txs, receive_token_amount, slippage } = action.value;
      const network_fee = txs.reduce((acc, tx) => acc + tx.gas_fee, 0n);
      const lp_fee = txs.reduce((acc, tx) => acc + tx.lp_fee, 0n);

      const ideal_amount = Number(receive_token_amount) / (1 - slippage / 100);
      const real_amount_of_gldt_without_tx_fee = Number(
        receive_token_amount + network_fee
      );
      const slippage_without_tx_fee =
        ((ideal_amount - real_amount_of_gldt_without_tx_fee) / ideal_amount) *
        100;

      return {
        ...prev,
        slippage: slippage_without_tx_fee,
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
    case "OPEN_CONFIRM_HIGH_SLIPPAGE":
      return {
        ...prev,
        is_open_confirm_dialog: false,
        is_open_disclaimer_confirm_high_slippage_dialog: true,
      };
    case "OPEN_DIALOG_DETAILS":
      return {
        ...prev,
        is_open_confirm_dialog: true,
      };
    case "CANCEL":
      return {
        ...prev,
        is_open_confirm_dialog: false,
        is_open_disclaimer_confirm_high_slippage_dialog: false,
      };
    case "CONFIRM":
      return {
        ...prev,
        is_open_confirm_dialog: false,
        is_open_details_dialog: true,
      };
    case "CONFIRM_HIGH_SLIPPAGE": {
      const { slippage } = action.value;
      return {
        ...prev,
        is_open_confirm_dialog: false,
        is_open_disclaimer_confirm_high_slippage_dialog: false,
        is_open_details_dialog: true,
        max_slippage: Math.ceil(slippage),
      };
    }
    case "RESET":
      return initialState;
  }
};

export const BuyGLDTStateReducerAtom = atomWithReducer(initialState, reducer);

export default BuyGLDTStateReducerAtom;
