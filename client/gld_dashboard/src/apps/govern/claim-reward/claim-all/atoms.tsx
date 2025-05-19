import { atomWithReducer } from "jotai/utils";
import { atom } from "jotai";
import _ from "lodash";

import {
  GOLDAO_LEDGER_CANISTER_ID,
  ICP_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
  WTN_LEDGER_CANISTER_ID,
} from "@constants";

import { Reward } from "../../utils";
import { RewardFeeData } from "@utils/useRewardsFee";
import { TokensRewards } from "../../utils/useGetAllNeuronsRewards";

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
      is_selected: true,
      is_claimable: false,
      amount: 0n,
      amount_usd: 0,
      neurons: [],
    },
    {
      id: "icp",
      name: "ICP",
      label: "Internet Computer",
      canister_id: ICP_LEDGER_CANISTER_ID,
      is_selected: true,
      is_claimable: false,
      amount: 0n,
      amount_usd: 0,
      neurons: [],
    },
    {
      id: "ogy",
      name: "OGY",
      label: "Origyn",
      canister_id: OGY_LEDGER_CANISTER_ID,
      is_selected: true,
      is_claimable: false,
      amount: 0n,
      amount_usd: 0,
      neurons: [],
    },
    {
      id: "wtn",
      name: "WTN",
      label: "Waterneuron",
      canister_id: WTN_LEDGER_CANISTER_ID,
      is_selected: true,
      is_claimable: false,
      amount: 0n,
      amount_usd: 0,
      neurons: [],
    },
  ],
  is_rewards_initialized: false,
};

const claimRewardReducer = (
  prev: ClaimRewardState,
  action:
    | {
        type: "SET_REWARDS";
        value: { rewards: TokensRewards[]; rewards_fee: RewardFeeData[] };
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
          (rewardFee) => rewardFee.id === reward.id
        );
        const is_claimable = found
          ? (reward.amount as bigint) >= found.fee
          : false;

        const neurons_claimable = found
          ? reward.neurons.filter((neuron) => neuron.reward >= found.fee)
          : [];

        return {
          ...reward,
          is_selected: is_claimable,
          is_claimable: !!neurons_claimable.length,
          neurons: neurons_claimable,
        };
      });
      const merged = _.values(
        _.merge(_.keyBy(prev.rewards, "id"), _.keyBy(rewards, "id"))
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
