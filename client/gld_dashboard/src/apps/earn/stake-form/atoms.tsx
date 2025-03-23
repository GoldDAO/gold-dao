import { atomWithReset, atomWithReducer } from "jotai/utils";

type StakeState = {
  is_open_stake_dialog_confirm: boolean;
  is_open_stake_dialog_insufficient_balance: boolean;
  is_open_stake_dialog_details: boolean;
  amount: bigint | undefined;
  fee: bigint | undefined;
};

const initialState: StakeState = {
  is_open_stake_dialog_confirm: false,
  is_open_stake_dialog_details: false,
  is_open_stake_dialog_insufficient_balance: false,
  amount: undefined,
  fee: undefined,
};

const stakeStateReducer = (
  prev: StakeState,
  action:
    | { type: "CANCEL" }
    | { type: "CONFIRM" }
    | { type: "RESET" }
    | { type: "SUBMIT"; value: { amount: bigint; fee: bigint } }
) => {
  switch (action.type) {
    case "SUBMIT":
      return {
        ...prev,
        amount: action.value.amount,
        fee: action.value.fee,
        is_open_stake_dialog_confirm: true,
      };
    case "CANCEL":
      return { ...prev, is_open_stake_dialog_confirm: false };
    case "CONFIRM":
      return {
        ...prev,
        is_open_stake_dialog_confirm: false,
        is_open_stake_dialog_details: true,
      };
    case "RESET":
      return initialState;
  }
};

export const StakeStateReducerAtom = atomWithReducer(
  initialState,
  stakeStateReducer
);

export const StakeStateAtom = atomWithReset<StakeState>(initialState);
