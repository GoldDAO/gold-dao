import { atomWithReducer } from "jotai/utils";
import { atom } from "jotai";
import { Reward } from "@earn/interfaces";

type ClaimRewardsState = {
  rewards: Reward[];

  is_open_dialog: boolean;
  is_step_form: boolean;
  is_step_details: boolean;
};

const initialState: ClaimRewardsState = {
  rewards: [],

  is_open_dialog: false,
  is_step_form: true,
  is_step_details: false,
};

const claimRewardsReducer = (
  prev: ClaimRewardsState,
  action:
    | {
        type: "SET_REWARDS";
        value: { rewards: Reward[] };
      }
    | { type: "SET_SELECTED_REWARD"; value: { name: string } }
    | {
        type: "SET_IS_OPEN_DIALOG";
        value: boolean;
      }
    | {
        type: "SET_IS_STEP_DETAILS";
      }
    | {
        type: "RESET";
      }
) => {
  switch (action.type) {
    case "SET_REWARDS": {
      return {
        ...prev,
        rewards: action.value.rewards,
      };
    }
    case "SET_SELECTED_REWARD": {
      return {
        ...prev,
        rewards: prev.rewards.map((reward) => {
          if (reward.name === action.value.name) {
            return {
              ...reward,
              is_selected: !reward.is_selected,
            };
          }
          return reward;
        }),
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
        is_step_form: true,
      };

    case "SET_IS_STEP_DETAILS":
      return {
        ...prev,
        is_step_details: true,
        is_step_form: false,
      };

    case "RESET":
      return initialState;
  }
};

export const ClaimRewardsStateReducerAtom = atomWithReducer(
  initialState,
  claimRewardsReducer
);

export const TotalSelectedAmountUSDAtom = atom((get) => {
  const state = get(ClaimRewardsStateReducerAtom);
  return state.rewards
    .filter((reward) => reward.is_selected)
    .reduce((total, reward) => total + reward.amount_usd, 0);
});

export const IsDisabledClaimingRewardsAtom = atom((get) => {
  const state = get(ClaimRewardsStateReducerAtom);
  return !state.rewards.some((reward) => reward.is_selected);
});

export const SelectedRewardsAtom = atom((get) => {
  const state = get(ClaimRewardsStateReducerAtom);
  return state.rewards.filter((reward) => reward.is_selected);
});
