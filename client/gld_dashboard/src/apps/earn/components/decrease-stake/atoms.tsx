import { atomWithReducer } from "jotai/utils";
import { UserStakedData } from "@earn/interfaces";
import { DissolveMode } from "./interfaces";
import { LedgerBalanceData } from "@shared/hooks/useFetchLedgerBalance";

interface StakedData extends UserStakedData {
  instant_dissolve_fee: number;
}

type Step =
  | "init"
  | "form"
  | "confirm"
  | "details_dissolving"
  | "details_dissolving_instantly";

type DecreaseStakeState = {
  unlock_amount: string;
  percentage_unlock_amount: number;
  dissolve_mode: DissolveMode;

  is_open_dialog: boolean;
  step: Step;

  user_balance_gldt: LedgerBalanceData;
  user_staked_data: StakedData;
};

const initialState: DecreaseStakeState = {
  unlock_amount: "20",
  percentage_unlock_amount: 0,

  dissolve_mode: "DISSOLVE",

  is_open_dialog: false,
  step: "init",

  user_staked_data: {
    staked_amount: 0,
    staked_amount_e8s: 0n,
    staked_amount_usd: 0,
    instant_dissolve_fee: 0,
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
  prev: DecreaseStakeState,
  action:
    | {
        type: "SET_UNLOCK_AMOUNT";
        value: string;
      }
    | {
        type: "SET_PERCENTAGE_UNLOCK_AMOUNT";
        value: number;
      }
    | {
        type: "SET_USER_STAKED_DATA";
        value: StakedData;
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
        type: "SET_DISSOLVE_MODE";
        value: DissolveMode;
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
    case "SET_UNLOCK_AMOUNT":
      return {
        ...prev,
        unlock_amount: action.value,
      };

    case "SET_PERCENTAGE_UNLOCK_AMOUNT":
      return {
        ...prev,
        percentage_unlock_amount: action.value,
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

    case "SET_DISSOLVE_MODE":
      return {
        ...prev,
        dissolve_mode: action.value,
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

export const DecreaseStakeStateReducerAtom = atomWithReducer(
  initialState,
  reducer
);
