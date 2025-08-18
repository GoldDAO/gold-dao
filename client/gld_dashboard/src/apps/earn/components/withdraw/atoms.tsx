import { atomWithReducer } from "jotai/utils";
import { DissolveEvent } from "@earn/interfaces";
import { DissolvedData } from "./interfaces";

type WithdrawState = {
  is_open_dialog: boolean;
  is_step_confirm: boolean;
  is_step_details: boolean;

  dissolved_data: DissolvedData;
};

const initialState: WithdrawState = {
  is_open_dialog: false,
  is_step_confirm: false,
  is_step_details: false,

  dissolved_data: {
    amount: 0,
    amount_e8s: 0n,
    amount_usd: 0,
    position_count: 0,
  },
};

const reducer = (
  prev: WithdrawState,
  action:
    | {
        type: "SET_DISSOLVED_DATA";
        value: DissolveEvent[];
      }
    | {
        type: "SET_IS_OPEN_DIALOG";
        value: boolean;
      }
    | {
        type: "SET_IS_STEP_CONFIRM";
      }
    | {
        type: "SET_IS_STEP_DETAILS";
      }
    | {
        type: "RESET";
      }
) => {
  switch (action.type) {
    case "SET_DISSOLVED_DATA": {
      const dissolved_positions = action.value.filter(
        (event) => event.is_withdrawable
      );

      if (!dissolved_positions.length) {
        return prev;
      }

      const amount = dissolved_positions.reduce(
        (acc, event) => acc + event.amount,
        0
      );
      const amount_e8s = dissolved_positions.reduce(
        (acc, event) => acc + BigInt(event.amount_e8s),
        0n
      );
      const amount_usd = dissolved_positions.reduce(
        (acc, event) => acc + event.amount_usd,
        0
      );
      const position_count = dissolved_positions.length;
      return {
        ...prev,
        dissolved_data: {
          amount,
          amount_e8s,
          amount_usd,
          position_count,
        },
      };
    }

    case "SET_IS_OPEN_DIALOG":
      if (!action.value)
        return {
          ...prev,
          is_open_dialog: false,
        };
      return {
        ...prev,
        is_open_dialog: true,
        is_step_confirm: true,
      };

    case "SET_IS_STEP_CONFIRM":
      return {
        ...prev,
        is_step_confirm: true,
      };

    case "SET_IS_STEP_DETAILS":
      return {
        ...prev,
        is_step_details: true,
        is_step_confirm: false,
      };

    case "RESET":
      return initialState;
  }
};

export const WithdrawStateReducerAtom = atomWithReducer(initialState, reducer);
