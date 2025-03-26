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

export interface RewardFeeData {
  name: string;
  canister_id: string;
  fee: bigint;
}

const rewardsData: RewardFeeData[] = [
  {
    name: "GOLDAO",
    canister_id: GOLDAO_LEDGER_CANISTER_ID,
    fee: 0n,
  },
  {
    name: "ICP",
    canister_id: ICP_LEDGER_CANISTER_ID,
    fee: 0n,
  },
  {
    name: "OGY",
    canister_id: OGY_LEDGER_CANISTER_ID,
    fee: 0n,
  },
];

const useStakeRewardsFee = (
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<RewardFeeData[], Error>, "queryKey" | "queryFn">
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["USER_STAKE_REWARDS_FEE"],
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
        throw new Error("Fetch stake rewards fee error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useStakeRewardsFee;
