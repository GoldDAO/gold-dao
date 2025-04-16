import { useMutation } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "@services/gldt_stake/idlFactory";
import claim_reward from "../claim_reward";

type RewardsArgs = { token: string; position_ids: bigint[] };

const useClaimReward = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
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
  });
};

export default useClaimReward;
