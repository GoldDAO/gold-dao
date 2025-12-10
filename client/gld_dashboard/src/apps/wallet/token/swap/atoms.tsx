import { atom } from "jotai";
import { atomWithReducer } from "jotai/utils";
import { MAX_SWAP_SLIPPAGE } from "@constants";
import { Token, TOKEN_GLDT, TOKEN_GOLDAO } from "@shared/utils/tokens";
import { TokenSwapData } from "@wallet/token/swap/utils";
import { FieldErrors, FieldValues } from "react-hook-form";

type SwapState = {
  send_token: TokenSwapData;
  receive_token: TokenSwapData;

  send_amount_input: string;
  form_state: {
    errors: FieldErrors<FieldValues>;
    isValid: boolean;
  };

  receive_amount: bigint | undefined;
  slippage_without_tx_fee: number;
  slippage_with_tx_fee: number;
  network_fee: bigint;
  lp_fee: bigint;

  max_slippage: number;

  is_open_form_dialog: boolean;
  is_open_confirm_dialog: boolean;
  is_open_details_dialog: boolean;
  is_open_disclaimer_confirm_high_slippage_dialog: boolean;
};

const initialState: SwapState = {
  send_token: {
    token: TOKEN_GLDT,
    amount_e8s: 0n,
    amount_usd: 0,
    user_balance: 0n,
    decimals: 0,
    fee: 0n,
  },
  receive_token: {
    token: TOKEN_GOLDAO,
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

  receive_amount: undefined,
  slippage_without_tx_fee: 0,
  slippage_with_tx_fee: 0,

  network_fee: 0n,
  lp_fee: 0n,

  max_slippage: MAX_SWAP_SLIPPAGE,

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
        type: "SET_SEND_TOKEN_DATA";
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
        type: "SET_RECEIVE_TOKEN_DATA";
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
          receive_amount: bigint;
          slippage_without_tx_fee: number;
          slippage_with_tx_fee: number;
          network_fee: bigint;
          lp_fee: bigint;
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
        value: { send_token: Token; receive_token?: Token };
      }
    | {
        type: "CLOSE_DIALOG_FORM";
      }
    | { type: "OPEN_DIALOG_CONFIRM" }
    | { type: "BACK_DIALOG_CONFIRM" }
    | { type: "CLOSE_DIALOG_CONFIRM" }
    | { type: "OPEN_DIALOG_CONFIRM_HIGH_SLIPPAGE" }
    | { type: "CONFIRM" }
    | { type: "CONFIRM_HIGH_SLIPPAGE"; value: { slippage_with_tx_fee: number } }
    | { type: "CANCEL" }
    | { type: "RESET" }
) => {
  switch (action.type) {
    case "SET_TOKEN_FROM":
      return {
        ...prev,
        send_token: {
          ...prev.send_token,
          token: action.value,
        },
      };
    case "SET_SEND_TOKEN_DATA":
      return {
        ...prev,
        send_token: {
          ...prev.send_token,
          ...action.value,
        },
      };

    case "SET_TOKEN_TO":
      return {
        ...prev,
        receive_token: {
          ...prev.receive_token,
          token: action.value,
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

    case "SET_SEND_AMOUNT":
      return {
        ...prev,
        send_amount_input: action.value,
      };

    case "SET_TX_DATA": {
      const {
        receive_amount,
        slippage_without_tx_fee,
        slippage_with_tx_fee,
        network_fee,
        lp_fee,
      } = action.value;
      return {
        ...prev,
        receive_amount,
        slippage_without_tx_fee,
        slippage_with_tx_fee,
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
        send_token: {
          ...prev.send_token,
          token: action.value.send_token,
        },
        receive_token: {
          ...prev.receive_token,
          token: action.value.receive_token
            ? action.value.receive_token
            : action.value.send_token.id === "gldt"
            ? TOKEN_GOLDAO
            : TOKEN_GLDT,
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

    case "CONFIRM":
      return {
        ...prev,
        is_open_confirm_dialog: false,
        is_open_details_dialog: true,
      };

    case "CANCEL":
      return {
        ...prev,
        is_open_form_dialog: true,
        is_open_confirm_dialog: false,
        is_open_disclaimer_confirm_high_slippage_dialog: false,
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

export const IsReceiveAmountValidAtom = atom((get) => {
  const state = get(SwapStateReducerAtom);
  const { isValid } = state.form_state;
  const { receive_amount, send_amount_input } = state;
  return !(
    isValid &&
    Number(send_amount_input) > 0 &&
    receive_amount !== undefined &&
    receive_amount <= 0n
  );
});

export const FormIsValidAtom = atom((get) => {
  const state = get(SwapStateReducerAtom);
  const isReceiveAmountValid = get(IsReceiveAmountValidAtom);
  const { isValid } = state.form_state;
  return isValid && isReceiveAmountValid;
});
