import { atomWithReset, atomWithReducer } from "jotai/utils";

import {
  GOLDAO_LEDGER_CANISTER_ID,
  ICP_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";
import { TokenData } from "./utils";

type ClaimRewardState = {
  is_open_claim_dialog_confirm: boolean;
  is_open_claim_dialog_details: boolean;
  stake_id: bigint | undefined;
  token_selected: TokenData[];
};

const initialState: ClaimRewardState = {
  is_open_claim_dialog_confirm: false,
  is_open_claim_dialog_details: false,
  stake_id: undefined,
  token_selected: [
    {
      id: "goldao",
      name: "GOLDAO",
      label: "GOLDAO",
      canisterId: GOLDAO_LEDGER_CANISTER_ID,
    },
    {
      id: "icp",
      name: "ICP",
      label: "Internet Computer",
      canisterId: ICP_LEDGER_CANISTER_ID,
    },
    {
      id: "ogy",
      name: "OGY",
      label: "OGY",
      canisterId: OGY_LEDGER_CANISTER_ID,
    },
  ],
};

const claimRewardReducer = (
  prev: ClaimRewardState,
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
        is_open_claim_dialog_confirm: true,
        stake_id: action.value.stake_id,
      };
    case "CANCEL":
      return initialState;
    case "CONFIRM":
      return {
        ...prev,
        is_open_claim_dialog_confirm: false,
        is_open_claim_dialog_details: true,
      };
    case "RESET":
      return initialState;
  }
};

export const ClaimRewardStateReducerAtom = atomWithReducer(
  initialState,
  claimRewardReducer
);

export const ClaimRewardStateAtom =
  atomWithReset<ClaimRewardState>(initialState);
