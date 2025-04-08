import { atomWithReset, atomWithReducer } from "jotai/utils";
import { atom } from "jotai";
import _ from "lodash";

import {
  GOLDAO_LEDGER_CANISTER_ID,
  ICP_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";

import { PositionRewards } from "./utils/index";
import { Reward } from "./utils/index";
import { RewardFeeData } from "@utils/useRewardsFee";

type ClaimRewardState = {
  is_open_claim_dialog_confirm: boolean;
  is_open_claim_dialog_details: boolean;
  rewards: Reward[];
  is_rewards_initialized: boolean;
};

const initialState: ClaimRewardState = {
  is_open_claim_dialog_confirm: false,
  is_open_claim_dialog_details: false,
  rewards: [
    {
      id: "goldao",
      name: "GOLDAO",
      label: "GOLDAO",
      canister_id: GOLDAO_LEDGER_CANISTER_ID,
      is_selected: false,
      is_claimable: false,
      amount: 0n,
      positions: [],
    },
    {
      id: "icp",
      name: "ICP",
      label: "Internet Computer",
      canister_id: ICP_LEDGER_CANISTER_ID,
      is_selected: false,
      is_claimable: false,
      amount: 0n,
      positions: [],
    },
    {
      id: "ogy",
      name: "OGY",
      label: "OGY",
      canister_id: OGY_LEDGER_CANISTER_ID,
      is_selected: false,
      is_claimable: false,
      amount: 0n,
      positions: [],
    },
  ],
  is_rewards_initialized: false,
};

const claimRewardReducer = (
  prev: ClaimRewardState,
  action:
    | {
        type: "SET_REWARDS";
        value: { rewards: PositionRewards[]; rewards_fee: RewardFeeData[] };
      }
    | { type: "SET_SELECTED_REWARD"; value: { name: string } }
    | { type: "OPEN_DIALOG_CONFIRM" }
    | { type: "CANCEL" }
    | { type: "CONFIRM" }
    | { type: "RESET" }
) => {
  switch (action.type) {
    case "SET_REWARDS": {
      const rewards = action.value.rewards.map((reward) => {
        const found = action.value.rewards_fee.find(
          (rewardFee) => rewardFee.name === reward.name
        );
        const is_claimable = found
          ? (reward.amount as bigint) >= found.fee
          : false;

        const positions_claimable = found
          ? reward.positions.filter((p) => p.amount >= found.fee)
          : [];

        return {
          ...reward,
          is_selected: is_claimable,
          is_claimable: !!positions_claimable.length,
          positions: positions_claimable,
        };
      });
      const merged = _.values(
        _.merge(_.keyBy(prev.rewards, "name"), _.keyBy(rewards, "name"))
      );

      return {
        ...prev,
        rewards: merged,
        is_rewards_initialized: true,
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
    case "OPEN_DIALOG_CONFIRM":
      return {
        ...prev,
        is_open_claim_dialog_confirm: true,
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

export const TotalSelectedAmountAtom = atom((get) => {
  const state = get(ClaimRewardStateReducerAtom);
  return state.rewards
    .filter((reward) => reward.is_selected)
    .reduce((total, reward) => total + (reward.amount || 0n), 0n);
});

export const ConfirmClaimEnableAtom = atom((get) => {
  const state = get(ClaimRewardStateReducerAtom);
  return state.rewards.some((reward) => reward.is_selected);
});

export const SelectedRewardsAtom = atom((get) => {
  const state = get(ClaimRewardStateReducerAtom);
  return state.rewards.filter((reward) => reward.is_selected);
});
