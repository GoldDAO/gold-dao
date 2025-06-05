import { atomWithReducer } from "jotai/utils";

type TransferNFTState = {
  is_open_transfer_dialog: boolean;
  transfer_tab: "send" | "receive";
  is_step_send_form: boolean;
  is_step_send_confirm: boolean;
  is_open_send_dialog_details: boolean;
  send_receive_address: string;
};

const initialState: TransferNFTState = {
  is_open_transfer_dialog: false,
  transfer_tab: "send",
  is_step_send_form: true,
  is_step_send_confirm: false,
  is_open_send_dialog_details: false,
  send_receive_address: "",
};

const reducer = (
  prev: TransferNFTState,
  action:
    | { type: "OPEN_TRANSFER_DIALOG" }
    | {
        type: "SET_TAB";
        value: "send" | "receive";
      }
    | { type: "STEP_SEND_CONFIRM"; value: string }
    | { type: "CANCEL_SEND_CONFIRM" }
    | { type: "SEND_CONFIRM" }
    | { type: "RESET" }
) => {
  switch (action.type) {
    case "OPEN_TRANSFER_DIALOG":
      return {
        ...prev,
        ...initialState,
        is_open_transfer_dialog: true,
      };
    case "SET_TAB":
      return {
        ...prev,
        transfer_tab: action.value,
      };
    case "STEP_SEND_CONFIRM":
      return {
        ...prev,
        send_receive_address: action.value,
        is_step_send_form: false,
        is_step_send_confirm: true,
      };
    case "CANCEL_SEND_CONFIRM":
      return {
        ...prev,
        is_step_send_confirm: false,
        is_step_send_form: true,
      };
    case "SEND_CONFIRM":
      return {
        ...prev,
        is_open_transfer_dialog: false,
        is_open_send_dialog_details: true,
      };
    case "RESET":
      return initialState;
  }
};

export const TransferNFTStateReducerAtom = atomWithReducer(
  initialState,
  reducer
);
