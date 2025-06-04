import { atomWithReducer } from "jotai/utils";
import { SelectNFTState } from "@shared/atoms/NFTStateAtom";

type Mode = "mint" | "burn";
type Step = "idle" | "submit" | "confirm" | "details";

export interface StateSwapNFT {
  mode: Mode;
  step: Step;
  collections: SelectNFTState | undefined;
}

const initialState: StateSwapNFT = {
  mode: "mint",
  step: "idle",
  collections: undefined,
};

const reducer = (
  prev: StateSwapNFT,
  action:
    | {
        type: "SET_MODE";
        value: Mode;
      }
    | {
        type: "INIT_MINT_MODE";
      }
    | {
        type: "INIT_BURN_MODE";
      }
    | {
        type: "SET_STEP";
        value: Step;
      }
    | {
        type: "SUBMIT";
        value: SelectNFTState;
      }
    | {
        type: "CONFIRM";
      }
    | {
        type: "RESET";
      }
) => {
  switch (action.type) {
    case "SET_MODE": {
      return {
        ...initialState,
        mode: action.value,
      };
    }
    case "INIT_MINT_MODE": {
      return {
        ...initialState,
        mode: "mint" as Mode,
        step: "submit" as Step,
      };
    }
    case "INIT_BURN_MODE": {
      return {
        ...initialState,
        mode: "burn" as Mode,
        step: "submit" as Step,
      };
    }
    case "SET_STEP": {
      return {
        ...prev,
        step: action.value,
      };
    }
    case "SUBMIT": {
      return {
        ...prev,
        step: "confirm" as Step,
        collections: action.value,
      };
    }
    case "CONFIRM": {
      return {
        ...prev,
        step: "details" as Step,
      };
    }
    case "RESET": {
      return initialState;
    }
  }
};

const SwapNFTReducerAtom = atomWithReducer(initialState, reducer);

export default SwapNFTReducerAtom;
