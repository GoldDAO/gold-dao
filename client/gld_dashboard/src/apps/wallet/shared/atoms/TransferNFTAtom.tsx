import { atomWithReducer } from "jotai/utils";

type Mutation = {
  count_started: number;
  count_settled: number;
};

export type TransferNFTState = {
  transfer_tab: "send" | "receive";
  is_open_receive_dialog: boolean;
  is_open_send_dialog_form: boolean;
  is_open_send_dialog_confirm: boolean;
  is_open_send_dialog_details: boolean;
  send_receive_address: string;
  collection_mutations: {
    "1G": Mutation;
    "10G": Mutation;
    "100G": Mutation;
    "1KG": Mutation;
  };
};

const initialState: TransferNFTState = {
  transfer_tab: "send",
  is_open_receive_dialog: false,
  is_open_send_dialog_form: false,
  is_open_send_dialog_confirm: false,
  is_open_send_dialog_details: false,
  send_receive_address: "",
  collection_mutations: {
    "1G": { count_started: 0, count_settled: 0 },
    "10G": { count_started: 0, count_settled: 0 },
    "100G": { count_started: 0, count_settled: 0 },
    "1KG": { count_started: 0, count_settled: 0 },
  },
};

const reducer = (
  prev: TransferNFTState,
  action:
    | {
        type: "SET_TAB";
        value: "send" | "receive";
      }
    | { type: "OPEN_TRANSFER_DIALOG" }
    | { type: "OPEN_SEND_DIALOG_CONFIRM"; value: string }
    | { type: "CANCEL_SEND_CONFIRM" }
    | { type: "OPEN_SEND_DIALOG_DETAILS" }
    | { type: "RESET" }
) => {
  switch (action.type) {
    case "SET_TAB":
      return {
        ...prev,
        transfer_tab: action.value,
        is_open_receive_dialog: action.value === "receive",
        is_open_send_dialog_form: action.value === "send",
      };

    case "OPEN_TRANSFER_DIALOG":
      return {
        ...prev,
        ...initialState,
        is_open_send_dialog_form: true,
      };

    case "OPEN_SEND_DIALOG_CONFIRM":
      return {
        ...prev,
        send_receive_address: action.value,
        is_open_send_dialog_form: false,
        is_open_send_dialog_confirm: true,
      };

    case "CANCEL_SEND_CONFIRM":
      return {
        ...prev,
        is_open_send_dialog_confirm: false,
        is_open_send_dialog_form: true,
      };

    case "OPEN_SEND_DIALOG_DETAILS":
      return {
        ...prev,
        is_open_send_dialog_confirm: false,
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
