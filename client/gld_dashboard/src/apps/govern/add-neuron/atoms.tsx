import { atomWithReducer } from "jotai/utils";

type AddNeuronState = {
  is_open: boolean;
};

const initialState: AddNeuronState = {
  is_open: false,
};

const addNeuronReducer = (
  prev: AddNeuronState,
  action: { type: "OPEN_DIALOG" } | { type: "RESET" }
) => {
  switch (action.type) {
    case "OPEN_DIALOG":
      return {
        ...prev,
        is_open: true,
      };
    case "RESET":
      return initialState;
  }
};

export const AddNeuronStateReducerAtom = atomWithReducer(
  initialState,
  addNeuronReducer
);
