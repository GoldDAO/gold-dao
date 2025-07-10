import { atomWithReducer } from "jotai/utils";
import { MAX_SWAP_SLIPPAGE } from "@constants";
import { SwapAmountsTxReply } from "@services/kongswap/interfaces";
import { Token, TOKEN_GLDT, TOKEN_ICP } from "@shared/utils/tokens";
import { TokenSwapData } from "@wallet/swap/utils";
import { FieldErrors, FieldValues } from "react-hook-form";

type SwapState = {
  token_from: TokenSwapData;
  token_to: TokenSwapData;

  send_amount_input: string;

  form_state: {
    errors: FieldErrors<FieldValues>;
    isValid: boolean;
  };

  slippage_without_tx_fee: number;
  slippage_with_tx_fee: number;
  max_slippage: number;
  network_fee: bigint;
  lp_fee: bigint;

  is_open_form_dialog: boolean;
  is_open_confirm_dialog: boolean;
  is_open_details_dialog: boolean;
  is_open_disclaimer_confirm_high_slippage_dialog: boolean;
};

const initialState: SwapState = {
  token_from: {
    token: TOKEN_GLDT,
    amount_e8s: 0n,
    amount_usd: 0,
    user_balance: 0n,
    decimals: 0,
    fee: 0n,
  },
  token_to: {
    token: TOKEN_ICP,
    amount_e8s: 0n,
    amount_usd: 0,
    user_balance: 0n,
    decimals: 0,
    fee: 0n,
  },

  send_amount_input: "",
  form_state: {
    errors: {},
    isValid: false,
  },

  slippage_without_tx_fee: 0,
  slippage_with_tx_fee: 0,
  max_slippage: MAX_SWAP_SLIPPAGE,
  network_fee: 0n,
  lp_fee: 0n,

  is_open_form_dialog: false,
  is_open_confirm_dialog: false,
  is_open_details_dialog: false,
  is_open_disclaimer_confirm_high_slippage_dialog: false,
};

const reducer = (
  prev: SwapState,
  action:
    | {
        type: "SET_TOKEN_FROM";
        value: Token;
      }
    | {
        type: "SET_TOKEN_FROM_DATA";
        value: {
          amount_e8s: bigint;
          amount_usd: number;
          user_balance: bigint;
          decimals: number;
          fee: bigint;
        };
      }
    | {
        type: "SET_TOKEN_TO";
        value: Token;
      }
    | {
        type: "SET_TOKEN_TO_DATA";
        value: {
          amount_e8s: bigint;
          amount_usd: number;
          user_balance: bigint;
          decimals: number;
          fee: bigint;
        };
      }
    | {
        type: "SET_SEND_AMOUNT";
        value: string;
      }
    | {
        type: "SET_TX_DATA";
        value: {
          slippage: number;
          txs: Array<SwapAmountsTxReply>;
          receive_token_amount: bigint;
        };
      }
    | {
        type: "SET_FORM_STATE";
        value: {
          errors: FieldErrors<FieldValues>;
          isValid: boolean;
        };
      }
    | {
        type: "OPEN_DIALOG_FORM";
        value: { token_from: Token; token_to?: Token };
      }
    | {
        type: "CLOSE_DIALOG_FORM";
      }
    | { type: "OPEN_DIALOG_CONFIRM" }
    | { type: "BACK_DIALOG_CONFIRM" }
    | { type: "CLOSE_DIALOG_CONFIRM" }
    | { type: "OPEN_DIALOG_CONFIRM_HIGH_SLIPPAGE" }
    | { type: "OPEN_DIALOG_DETAILS" }
    | { type: "CONFIRM" }
    | { type: "CONFIRM_HIGH_SLIPPAGE"; value: { slippage_with_tx_fee: number } }
    | { type: "CANCEL" }
    | { type: "RESET" }
) => {
  switch (action.type) {
    case "SET_TOKEN_FROM":
      return {
        ...prev,
        token_from: {
          ...prev.token_from,
          token: action.value,
        },
      };
    case "SET_TOKEN_FROM_DATA":
      return {
        ...prev,
        token_from: {
          ...prev.token_from,
          ...action.value,
        },
      };

    case "SET_TOKEN_TO":
      return {
        ...prev,
        token_to: {
          ...prev.token_to,
          token: action.value,
        },
      };

    case "SET_TOKEN_TO_DATA":
      return {
        ...prev,
        token_to: {
          ...prev.token_to,
          ...action.value,
        },
      };

    case "SET_SEND_AMOUNT":
      return {
        ...prev,
        send_amount_input: action.value,
      };

    case "SET_TX_DATA": {
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
        slippage_without_tx_fee,
        slippage_with_tx_fee: slippage,
        network_fee,
        lp_fee,
      };
    }

    case "SET_FORM_STATE":
      return {
        ...prev,
        form_state: {
          errors: action.value.errors,
          isValid: action.value.isValid,
        },
      };

    case "OPEN_DIALOG_FORM":
      return {
        ...initialState,
        is_open_form_dialog: true,
        token_from: {
          ...prev.token_from,
          token: action.value.token_from,
        },
        token_to: {
          ...prev.token_to,
          token: action.value.token_to
            ? action.value.token_to
            : action.value.token_from.id === "icp"
            ? TOKEN_GLDT
            : TOKEN_ICP,
        },
      };

    case "CLOSE_DIALOG_FORM":
      return {
        ...prev,
        is_open_form_dialog: false,
      };

    case "OPEN_DIALOG_CONFIRM":
      return {
        ...prev,
        is_open_form_dialog: false,
        is_open_confirm_dialog: true,
      };

    case "BACK_DIALOG_CONFIRM":
      return {
        ...prev,
        is_open_confirm_dialog: false,
        is_open_form_dialog: true,
      };

    case "CLOSE_DIALOG_CONFIRM":
      return {
        ...prev,
        is_open_confirm_dialog: false,
      };

    case "OPEN_DIALOG_CONFIRM_HIGH_SLIPPAGE":
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
      const { slippage_with_tx_fee } = action.value;
      return {
        ...prev,
        is_open_confirm_dialog: false,
        is_open_disclaimer_confirm_high_slippage_dialog: false,
        is_open_details_dialog: true,
        max_slippage: Math.ceil(slippage_with_tx_fee),
      };
    }

    case "RESET": {
      return initialState;
    }
  }
};

export const SwapStateReducerAtom = atomWithReducer(initialState, reducer);

export default SwapStateReducerAtom;
