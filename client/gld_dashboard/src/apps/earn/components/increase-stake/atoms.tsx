import { atomWithReducer } from "jotai/utils";
import { FieldErrors, FieldValues } from "react-hook-form";
import { LedgerBalanceData } from "@shared/hooks/useFetchLedgerBalance";
import { UserStakedData } from "@earn/interfaces";

type Step = "init" | "form" | "details";

type IncreaseStakeState = {
  stake_amount: string;

  form_state: {
    errors: FieldErrors<FieldValues>;
    isValid: boolean;
  };

  is_open_dialog: boolean;
  step: Step;

  user_staked_data: UserStakedData;
  user_balance_gldt: LedgerBalanceData;
};

const initialState: IncreaseStakeState = {
  stake_amount: "",

  form_state: {
    errors: {},
    isValid: false,
  },

  is_open_dialog: false,
  step: "init",

  user_staked_data: {
    staked_amount: 0,
    staked_amount_e8s: 0n,
    staked_amount_usd: 0,
  },
  user_balance_gldt: {
    balance: 0,
    balance_e8s: 0n,
    balance_usd: 0,
    decimals: 0,
    fee: 0,
    fee_usd: 0,
    fee_e8s: 0n,
    price_usd: 0,
  },
};

const reducer = (
  prev: IncreaseStakeState,
  action:
    | {
        type: "SET_STAKE_AMOUNT";
        value: string;
      }
    | {
        type: "SET_FORM_STATE";
        value: {
          errors: FieldErrors<FieldValues>;
          isValid: boolean;
        };
      }
    | {
        type: "SET_USER_STAKED_DATA";
        value: UserStakedData;
      }
    | {
        type: "SET_USER_BALANCE_GLDT";
        value: LedgerBalanceData;
      }
    | {
        type: "SET_IS_OPEN_DIALOG";
        value: boolean;
      }
    | {
        type: "SET_STEP";
        value: Step;
      }
    | {
        type: "RESET";
      }
) => {
  switch (action.type) {
    case "SET_STAKE_AMOUNT":
      return {
        ...prev,
        stake_amount: action.value,
      };

    case "SET_FORM_STATE":
      return {
        ...prev,
        form_state: {
          errors: action.value.errors,
          isValid: action.value.isValid,
        },
      };

    case "SET_USER_STAKED_DATA":
      return {
        ...prev,
        user_staked_data: action.value,
      };

    case "SET_USER_BALANCE_GLDT":
      return {
        ...prev,
        user_balance_gldt: action.value,
      };

    case "SET_IS_OPEN_DIALOG":
      return {
        ...prev,
        is_open_dialog: action.value,
      };

    case "SET_STEP":
      return {
        ...prev,
        step: action.value,
      };

    case "RESET":
      return initialState;
  }
};

export const IncreaseStakeStateReducerAtom = atomWithReducer(
  initialState,
  reducer
);

// export const IsReceiveAmountValidAtom = atom((get) => {
//   const state = get(IncreaseStakeStateReducerAtom);
//   const { isValid } = state.form_state;
//   const { receive_amount, send_amount_input } = state;
//   return !(
//     isValid &&
//     Number(send_amount_input) > 0 &&
//     receive_amount !== undefined &&
//     receive_amount <= 0n
//   );
// });

// export const FormIsValidAtom = atom((get) => {
//   const state = get(IncreaseStakeStateReducerAtom);
//   const isReceiveAmountValid = get(IsReceiveAmountValidAtom);
//   const { isValid } = state.form_state;
//   return isValid && isReceiveAmountValid;
// });
