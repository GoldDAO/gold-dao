import { atomWithReset, atomWithReducer } from "jotai/utils";

type UnlockState = {
  is_open_unlock_dialog_confirm: boolean;
  is_open_dissolve_dialog_details: boolean;
  is_open_unstake_early_dialog_details: boolean;
  stake_id: bigint | undefined;
  unlock_type: "DISSOLVE" | "UNSTAKE_EARLY";
};

const initialState: UnlockState = {
  is_open_unlock_dialog_confirm: false,
  is_open_dissolve_dialog_details: false,
  is_open_unstake_early_dialog_details: false,
  stake_id: undefined,
  unlock_type: "DISSOLVE",
};

const reducer = (
  prev: UnlockState,
  action:
    | { type: "OPEN_DIALOG_CONFIRM"; value: { stake_id: bigint } }
    | {
        type: "SET_UNLOCK_TYPE";
        value: { unlock_type: "DISSOLVE" | "UNSTAKE_EARLY" };
      }
    | { type: "CANCEL" }
    | { type: "CONFIRM" }
    | { type: "RESET" }
) => {
  switch (action.type) {
    case "OPEN_DIALOG_CONFIRM":
      return {
        ...prev,
        is_open_unlock_dialog_confirm: true,
        stake_id: action.value.stake_id,
      };
    case "SET_UNLOCK_TYPE":
      return {
        ...prev,
        unlock_type: action.value.unlock_type,
      };
    case "CANCEL":
      return initialState;
    case "CONFIRM":
      return {
        ...prev,
        is_open_unlock_dialog_confirm: false,
        is_open_dissolve_dialog_details: prev.unlock_type === "DISSOLVE",
        is_open_unstake_early_dialog_details:
          prev.unlock_type === "UNSTAKE_EARLY",
      };
    case "RESET":
      return initialState;
  }
};

export const UnlockStateReducerAtom = atomWithReducer(initialState, reducer);

export const UnlockStateAtom = atomWithReset<UnlockState>(initialState);
