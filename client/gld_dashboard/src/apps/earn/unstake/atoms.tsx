import { atomWithReset, atomWithReducer } from "jotai/utils";

type UnstakeState = {
  is_open_unstake_dialog_confirm: boolean;
  is_open_unstake_dialog_details: boolean;
  stake_id: bigint | undefined;
};

const initialState: UnstakeState = {
  is_open_unstake_dialog_confirm: false,
  is_open_unstake_dialog_details: false,
  stake_id: undefined,
};

const reducer = (
  prev: UnstakeState,
  action:
    | { type: "OPEN_DIALOG_CONFIRM"; value: { stake_id: bigint } }
    | { type: "CANCEL" }
    | { type: "CONFIRM" }
    | { type: "RESET" }
) => {
  switch (action.type) {
    case "OPEN_DIALOG_CONFIRM":
      return {
        ...prev,
        is_open_unstake_dialog_confirm: true,
        stake_id: action.value.stake_id,
      };
    case "CANCEL":
      return initialState;
    case "CONFIRM":
      return {
        ...prev,
        is_open_unstake_dialog_confirm: false,
        is_open_unstake_dialog_details: true,
      };
    case "RESET":
      return initialState;
  }
};

export const UnstakeStateReducerAtom = atomWithReducer(initialState, reducer);

export const UnstakeStateAtom = atomWithReset<UnstakeState>(initialState);
