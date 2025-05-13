import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "@services/gldt_stake/idlFactory";
import claim_reward from "../claim_reward";

type RewardsArgs = { token: string; position_ids: bigint[] };

const useClaimReward = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ position_ids, token }: RewardsArgs) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        await Promise.all(
          position_ids.map(async (id) => {
            return claim_reward(actor, {
              id,
              token,
            });
          })
        );
      } catch (err) {
        console.error(err);
        throw new Error(`Claim rewards error! Please retry later.`);
      }
    },
    onError: (err) => {
      console.log(err);
    },
    onSuccess: () => {
      // console.log(res);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: ["USER_POSITIONS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_POSITIONS_REWARDS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_POSITIONS_TOTAL_STAKED_AMOUNT"],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE"],
      });
    },
  });
};

export default useClaimReward;
