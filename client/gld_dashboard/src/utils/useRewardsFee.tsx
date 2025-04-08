import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import {
  GOLDAO_LEDGER_CANISTER_ID,
  ICP_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";

import { idlFactory } from "@services/ledger/idlFactory";
import icrc1_fee from "@services/ledger/icrc1_fee";
import { Ledger } from "@services/ledger/utils/interfaces";

export interface RewardFeeData {
  id: Ledger;
  name: string;
  canister_id: string;
  fee: bigint;
}

const rewardsData: RewardFeeData[] = [
  {
    id: "goldao",
    name: "GOLDAO",
    canister_id: GOLDAO_LEDGER_CANISTER_ID,
    fee: 0n,
  },
  {
    id: "icp",
    name: "ICP",
    canister_id: ICP_LEDGER_CANISTER_ID,
    fee: 0n,
  },
  {
    id: "ogy",
    name: "OGY",
    canister_id: OGY_LEDGER_CANISTER_ID,
    fee: 0n,
  },
  {
    id: "wtn",
    name: "WTN",
    canister_id: OGY_LEDGER_CANISTER_ID,
    fee: 0n,
  },
];

const useRewardsFee = (
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<RewardFeeData[], Error>, "queryKey" | "queryFn">
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["USER_REWARDS_FEE"],
    queryFn: async (): Promise<RewardFeeData[]> => {
      try {
        const data = await Promise.all(
          rewardsData.map(async (reward) => {
            const actor = Actor.createActor(idlFactory, {
              agent,
              canisterId: reward.canister_id,
            });
            const fee = await icrc1_fee(actor);
            return {
              ...reward,
              fee,
            };
          })
        );
        return data;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch rewards fee error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useRewardsFee;
